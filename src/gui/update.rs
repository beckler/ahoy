use std::fs::remove_file;

use iced::Command;
use log::*;
use pirate_midi_rs::*;

use crate::command::{
    device::{enter_bootloader, install_binary, UsbConnection},
    github::{fetch_asset, fetch_releases},
};

use super::{usb, Ahoy, Error, Message};

pub(crate) fn handle_message(ahoy: &mut Ahoy, message: Message) -> Command<Message> {
    match message {
        Message::FetchReleases => {
            ahoy.error = None;
            ahoy.releases = None;
            ahoy.selected_version = None;
            info!("refresh requested - attempt to fetch releases...");
            if ahoy.connection.is_some() {
                return Command::perform(fetch_releases(), Message::RetrievedReleases);
            }
        }
        Message::RetrievedReleases(Ok(releases)) => {
            // grab first version that matches the filter
            ahoy.selected_version = releases
                .iter()
                .cloned()
                .find(|rel| ahoy.filter.matches(rel));

            // set our releases
            ahoy.releases = Some(releases);
        }
        Message::RetrievedReleases(Err(err)) => {
            ahoy.error = Some(Error::RemoteApi(err.to_string()))
        }
        Message::ReleaseFilterChanged(filter) => ahoy.filter = filter,
        Message::SelectedRelease(release) => ahoy.selected_version = Some(*release),
        Message::Download(asset) => {
            return Command::perform(fetch_asset(*asset), Message::Downloaded);
        }
        Message::Downloaded(Ok(path)) => {
            ahoy.installable_asset = Some(path.clone());
            ahoy.confirm_modal.show(path);
        }
        Message::Downloaded(Err(err)) => ahoy.error = Some(Error::RemoteApi(err.to_string())),
        Message::DeviceChangedAction(event) => match event {
            usb::Event::Connect(device) => {
                if ahoy.installing {
                    // if we have a match for the expected DFU bootloader product and vendor ids, trigger the install
                    if device.is_dfu_device() {
                        ahoy.dfu_connection = Some(device);
                        return self::handle_message(ahoy, Message::Install);
                    }
                } else if device.is_stm_device() {
                    // attempt to get the device details
                    match PirateMIDIDevice::new().send(pirate_midi_rs::Command::Check) {
                        Ok(response) => {
                            if let Response::Check(details) = response {
                                info!("DEVICE DETAILS: {:?}", details);
                                ahoy.connection = Some(UsbConnection::new(device, details));

                                // retrieve releases if we have a valid device
                                return Command::perform(
                                    fetch_releases(),
                                    Message::RetrievedReleases,
                                );
                            }
                        }
                        Err(err) => {
                            error!("error connecting to device: {:?}", err);
                            return self::handle_message(ahoy, Message::Cancel);
                        }
                    }
                }
            }
            usb::Event::Disconnect(device) => {
                info!("DEVICE DISCONNECTED: {:?}", device);
                if !ahoy.installing {
                    ahoy.connection = None;
                    ahoy.releases = None;
                    ahoy.selected_version = None;
                }
            }
        },
        Message::EnterBootloader => {
            // hide the modal
            ahoy.confirm_modal.hide();

            // set the install flag to true
            ahoy.installing = true;

            // enter the bootloader - but only if we have a connected device
            match &ahoy.connection {
                Some(_) => {
                    return Command::perform(enter_bootloader(), Message::WaitForBootloader);
                }
                None => panic!(
                    "should not be able to reach this state - so I have no idea wtf happened"
                ),
            }
        }
        Message::WaitForBootloader(Ok(())) => {} // do nothing but wait for the DeviceChangedAction::Connect event!
        Message::WaitForBootloader(Err(err)) => {
            ahoy.error = Some(Error::Install(err.to_string()));
            ahoy.installing = false;
            return self::handle_message(ahoy, Message::Cancel);
        }
        Message::Install => {
            info!("installing!");

            match &ahoy.dfu_connection {
                // we only care that the connection exists
                Some(_) => {
                    // get our firmware path
                    let binary_path = ahoy
                        .installable_asset
                        .as_ref()
                        .expect("downloaded asset went missing!");

                    let progress_fn = {
                        let mut installer = ahoy.installer.clone();
                        move |count| {
                            installer.increment_progress(count);
                        }
                    };

                    return Command::perform(
                        install_binary(binary_path.to_path_buf(), Some(progress_fn)),
                        Message::PostInstall,
                    );
                }
                None => return self::handle_message(ahoy, Message::Cancel),
            }
        }
        Message::PostInstall(result) => {
            ahoy.installing = false;
            ahoy.post_install = true;
            info!("post-install result: {:?}", result);
        }
        Message::Cancel => {
            // reset the model
            ahoy.installing = false;
            ahoy.post_install = false;
            // delete the downloaded file if it exists
            match &ahoy.installable_asset {
                Some(asset_path) => {
                    if asset_path.exists() {
                        info!("canceled - deleting file: {}", asset_path.display());
                        match remove_file(asset_path) {
                            Ok(_) => ahoy.installable_asset = None,
                            Err(err) => error!("unable to delete file: {}", err.to_string()),
                        }
                    } else {
                        ahoy.installable_asset = None;
                    }
                }
                None => (), // do nothing
            }
            // hide the modal
            ahoy.confirm_modal.hide();
        }
    }
    Command::none()
}
