use std::fs::remove_file;

use crate::command::install_binary;
use iced::Command;
use log::*;
use pirate_midi::*;

use crate::command::{enter_bootloader, fetch_asset, fetch_releases, UsbConnection};

use super::{usb, Ahoy, Error, Message};

pub(crate) fn handle_message<'a>(ahoy: &mut Ahoy, message: Message) -> Command<Message> {
    match message {
        Message::FetchReleases => {
            ahoy.error = None;
            ahoy.releases = None;
            ahoy.selected_version = None;
            info!("refresh requested - attempt to fetch releases...");
            if let Some(_) = ahoy.connection {
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
        Message::RetrievedReleases(Err(err)) => ahoy.error = Some(Error::APIError(err.to_string())),
        Message::ReleaseFilterChanged(filter) => ahoy.filter = filter,
        Message::SelectedRelease(release) => ahoy.selected_version = Some(release),
        Message::Download(asset) => {
            return Command::perform(fetch_asset(asset), Message::Downloaded);
        }
        Message::Downloaded(Ok(path)) => {
            ahoy.installable_asset = Some(path.clone());
            ahoy.confirm_modal.show(path);
        }
        Message::Downloaded(Err(err)) => ahoy.error = Some(Error::APIError(err.to_string())),
        Message::DeviceChangedAction(event) => match event {
            usb::Event::Connect(device) => {
                if ahoy.installing {
                    // if we have a match for the expected DFU bootloader product and vendor ids, trigger the install
                    if device.is_dfu_device() {
                        ahoy.connection = None;
                        ahoy.dfu_connection = Some(device);
                        return self::handle_message(ahoy, Message::Install);
                    }
                } else {
                    if device.is_stm_device() {
                        // attempt to get the device details
                        match PirateMIDIDevice::new().send(pirate_midi::Command::Check) {
                            Ok(response) => match response {
                                Response::Check(details) => {
                                    info!("DEVICE DETAILS: {:?}", details);
                                    ahoy.connection = Some(UsbConnection::new(device, details));

                                    // retrieve releases if we have a valid device
                                    return Command::perform(
                                        fetch_releases(),
                                        Message::RetrievedReleases,
                                    );
                                }
                                _ => (),
                            },
                            Err(err) => {
                                error!("error connecting to device: {:?}", err);
                                ahoy.connection = None;
                                ahoy.releases = None;
                            }
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
            ahoy.error = Some(Error::InstallError(err.to_string()));
            ahoy.installing = false;
            return self::handle_message(ahoy, Message::Cancel);
        }
        Message::Install => {
            info!("installing!");

            match &ahoy.dfu_connection {
                Some(device) => match &device.raw_device {
                    Some(raw_device) => {
                        // get our firmware path
                        let binary_path = ahoy
                            .installable_asset
                            .as_ref()
                            .expect("downloaded asset went missing!");

                        let progress_fn = {
                            let installer = ahoy.installer.clone();
                            move |count| {
                                installer.increment_progress(count);
                            }
                        };

                        return Command::perform(
                            install_binary(
                                raw_device.clone(),
                                binary_path.to_path_buf(),
                                Some(progress_fn),
                            ),
                            Message::PostInstall,
                        );
                    }
                    None => return self::handle_message(ahoy, Message::Cancel),
                },
                None => return self::handle_message(ahoy, Message::Cancel),
            }
        }
        Message::PostInstall(result) => {}
        Message::Cancel => {
            // cancel installation
            ahoy.installing = false;
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
            ahoy.confirm_modal.hide()
        }
    }
    Command::none()
}
