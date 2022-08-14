use std::fs::remove_file;

use iced::Command;
use log::*;
use pirate_midi_rs::*;

use crate::{
    command::{
        device::{enter_bootloader, install_binary, UsbConnection},
        release::{fetch_asset, fetch_releases},
    },
    gui::usb,
};

use super::Firmware;

pub(crate) fn handle_message(this: &mut Firmware, message: Message) -> Command<Message> {
    match message {
        Event::FetchReleases => {
            this.error = None;
            this.releases = None;
            this.selected_version = None;
            info!("refresh requested - attempt to fetch releases...");
            if this.connection.is_some() {
                return Command::perform(fetch_releases(), Event::RetrievedReleases);
            }
        }
        Event::RetrievedReleases(Ok(releases)) => {
            // grab first version that matches the filter
            this.selected_version = releases
                .iter()
                .cloned()
                .find(|rel| this.filter.matches(rel));

            // set our releases
            this.releases = Some(releases);
        }
        Event::RetrievedReleases(Err(err)) => this.error = Some(Error::RemoteApi(err.to_string())),
        Event::ReleaseFilterChanged(filter) => this.filter = filter,
        Event::SelectedRelease(release) => this.selected_version = Some(*release),
        Event::Download(asset) => {
            return Command::perform(fetch_asset(*asset), Event::Downloaded);
        }
        Event::Downloaded(Ok(path)) => {
            this.installable_asset = Some(path.clone());
            this.confirm_modal.show(path);
        }
        Event::Downloaded(Err(err)) => this.error = Some(Error::RemoteApi(err.to_string())),
        Event::DeviceChangedAction(event) => match event {
            usb::Event::Connect(device) => {
                if this.installing {
                    // if we have a match for the expected DFU bootloader product and vendor ids, trigger the install
                    if device.is_dfu_device() {
                        this.dfu_connection = Some(device);
                        return self::handle_message(this, Event::Install);
                    }
                } else if device.is_stm_device() {
                    // attempt to get the device details
                    match PirateMIDIDevice::new().send(pirate_midi_rs::Command::Check) {
                        Ok(response) => {
                            if let Response::Check(details) = response {
                                info!("DEVICE DETAILS: {:?}", details);
                                this.connection = Some(UsbConnection::new(device, details));

                                // retrieve releases if we have a valid device
                                return Command::perform(
                                    fetch_releases(),
                                    Event::RetrievedReleases,
                                );
                            }
                        }
                        Err(err) => {
                            error!("error connecting to device: {:?}", err);
                            return self::handle_message(this, Event::Cancel);
                        }
                    }
                }
            }
            usb::Event::Disconnect(device) => {
                info!("DEVICE DISCONNECTED: {:?}", device);
                if !this.installing {
                    this.connection = None;
                    this.releases = None;
                    this.selected_version = None;
                }
            }
        },
        Event::EnterBootloader => {
            // hide the modal
            this.confirm_modal.hide();

            // set the install flag to true
            this.installing = true;

            // enter the bootloader - but only if we have a connected device
            match &this.connection {
                Some(_) => {
                    return Command::perform(enter_bootloader(), Event::WaitForBootloader);
                }
                None => panic!(
                    "should not be able to reach this state - so I have no idea wtf happened"
                ),
            }
        }
        Event::WaitForBootloader(Ok(())) => {} // do nothing but wait for the DeviceChangedAction::Connect event!
        Event::WaitForBootloader(Err(err)) => {
            this.error = Some(Error::Install(err.to_string()));
            this.installing = false;
            return self::handle_message(this, Event::Cancel);
        }
        Event::Install => {
            info!("installing!");

            match &this.dfu_connection {
                // we only care that the connection exists
                Some(_) => {
                    // get our firmware path
                    let binary_path = this
                        .installable_asset
                        .as_ref()
                        .expect("downloaded asset went missing!");

                    let progress_fn = {
                        let mut installer = this.installer.clone();
                        move |count| {
                            installer.increment_progress(count);
                        }
                    };

                    return Command::perform(
                        install_binary(binary_path.to_path_buf(), Some(progress_fn)),
                        Event::PostInstall,
                    );
                }
                None => return self::handle_message(this, Event::Cancel),
            }
        }
        Event::PostInstall(result) => {
            this.installing = false;
            this.post_install = true;
            info!("post-install result: {:?}", result);
        }
        Event::Cancel => {
            // reset the model
            this.installing = false;
            this.post_install = false;
            // delete the downloaded file if it exists
            match &this.installable_asset {
                Some(asset_path) => {
                    if asset_path.exists() {
                        info!("canceled - deleting file: {}", asset_path.display());
                        match remove_file(asset_path) {
                            Ok(_) => this.installable_asset = None,
                            Err(err) => error!("unable to delete file: {}", err.to_string()),
                        }
                    } else {
                        this.installable_asset = None;
                    }
                }
                None => (), // do nothing
            }
            // hide the modal
            this.confirm_modal.hide();
        }
    }
    Command::none()
}
