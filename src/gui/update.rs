use std::{fs::remove_file, sync::Arc};

use async_std::{sync::Mutex, task};
use futures::{channel::mpsc, SinkExt};
use iced::Command;
use log::*;
use pirate_midi_rs::*;

use crate::command::{
    device::{enter_bootloader, install_binary},
    github::{fetch_asset, fetch_releases},
};

use super::{usb, Ahoy, Message};

pub(crate) fn handle_message(ahoy: &mut Ahoy, message: Message) -> Command<Message> {
    match message {
        Message::UpdateAvailable(Ok(result)) => {
            info!("is update available?: {result}");
        }
        Message::UpdateAvailable(Err(err)) => {
            error!("issue with updates: {err}");
        }
        Message::UpdateApplication => {}
        Message::FetchReleases => {
            info!("fetching releases");
            ahoy.releases = None;
            ahoy.selected_version = None;
            info!("refresh requested - attempt to fetch releases...");
            return Command::perform(fetch_releases(), Message::RetrievedReleases);
        }
        Message::RetrievedReleases(Ok(releases)) => {
            info!("retrieved releases");
            // grab first version that matches the filter
            ahoy.selected_version = releases
                .iter()
                .cloned()
                .find(|rel| ahoy.filter.matches(rel));

            // set our releases
            ahoy.releases = Some(releases);
        }
        Message::RetrievedReleases(Err(err)) => {
            ahoy.error = Some(super::Error::RemoteApi(err.to_string()))
        }
        Message::ReleaseFilterChanged(filter) => ahoy.filter = filter,
        Message::SelectedRelease(release) => ahoy.selected_version = Some(*release),
        Message::Download(asset) => {
            info!("downloading asset");
            return Command::perform(fetch_asset(*asset), Message::Downloaded);
        }
        Message::Downloaded(Ok(path)) => {
            info!("downloaded release to: {}", path.display());
            ahoy.installable_asset = Some(path.clone());
            ahoy.confirm_modal.show(path);
        }
        Message::Downloaded(Err(err)) => {
            ahoy.error = Some(super::Error::RemoteApi(err.to_string()))
        }
        Message::DeviceChangedAction(event) => match event {
            usb::Event::Connect(device) => {
                info!("DEVICE CONNECTED: {:?}", device);
                // if a DFU device connects, and we have an asset, install it!
                if ahoy.installable_asset.is_some() && device.is_dfu_device() {
                    // create our channel for sharing install progress
                    let (tx, rx) = mpsc::channel::<f32>(10);
                    ahoy.device = super::DeviceState::DFU(
                        Some(device.raw_device.unwrap()),
                        Some(tx),
                        Some(Arc::new(Mutex::new(rx))),
                    );
                    return self::handle_message(ahoy, Message::Install);
                }

                // if we detect a device, attempt to get the details
                if device.is_stm_device() {
                    info!("device is STM!");
                    // attempt to get the device details
                    match PirateMIDIDevice::new().send(pirate_midi_rs::Command::Check) {
                        Ok(response) => {
                            if let Response::Check(details) = response {
                                info!("DEVICE DETAILS: {:?}", details);
                                ahoy.device = super::DeviceState::Connected(details);

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
                match ahoy.device {
                    crate::gui::DeviceState::PostInstall => (), // do nothing
                    _ => {
                        ahoy.device = super::DeviceState::Disconnected;
                    }
                }
                return Command::none();
            }
        },
        Message::EnterBootloader => {
            // change the device state for quicker ui update
            ahoy.device = super::DeviceState::DFU(None, None, None);

            // hide the modal
            info!("hiding modal");
            ahoy.confirm_modal.hide();

            // send the command to enter bootloader mode
            info!("sending bootloader command...");
            return Command::perform(enter_bootloader(), Message::WaitForBootloader);
        }
        Message::WaitForBootloader(Ok(())) => {
            // wait for the DeviceChangedAction::Connect event!
            return Command::none();
        }
        Message::WaitForBootloader(Err(err)) => {
            ahoy.error = Some(super::Error::Install(err.to_string()));
            return self::handle_message(ahoy, Message::Cancel);
        }
        Message::InstallProgress(progress) => ahoy.install_progress += progress,
        Message::Install => {
            info!("installing!");

            match &ahoy.device {
                crate::gui::DeviceState::DFU(device, sender, _) => {
                    // get our firmware path
                    let binary_path = ahoy
                        .installable_asset
                        .as_ref()
                        .expect("downloaded asset went missing!");

                    let progress_fn = {
                        // get total file size to determine percentage
                        let total = binary_path.metadata().unwrap().len();
                        let mut tx = sender.as_ref().unwrap().clone();

                        move |uploaded| {
                            let percentage = (uploaded as f32 / total as f32) * 100.0;
                            match task::block_on(async { tx.send(percentage).await }) {
                                Ok(_) => (),
                                Err(err) => error!("error sending install percentage: {err}"),
                            };
                        }
                    };

                    return Command::perform(
                        install_binary(
                            binary_path.to_path_buf(),
                            Some(progress_fn),
                            device.clone(),
                        ),
                        Message::PostInstallResult,
                    );
                }
                _ => return self::handle_message(ahoy, Message::Cancel),
            }
        }
        Message::PostInstallResult(result) => {
            match result {
                Ok(_) => info!("post-install result: DONE"),
                Err(err) => {
                    error!("post-install result: {:?}", err);
                    ahoy.error = Some(super::Error::Install(err.to_string()));
                }
            };

            ahoy.device = super::DeviceState::PostInstall;
            return self::handle_message(ahoy, Message::Cancel); //send cancel to cleanup
        }
        Message::AttemptReset => {
            ahoy.device = super::DeviceState::Disconnected;
        }
        Message::Cancel => {
            info!("cancelling or cleaning up");
            // reset install progress
            ahoy.install_progress = 0.0;
            // delete the downloaded file if it exists
            match &ahoy.installable_asset {
                Some(asset_path) => {
                    if asset_path.exists() {
                        info!("deleting file: {}", asset_path.display());
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
            ahoy.error = None;
            // hide the modal - if open
            ahoy.confirm_modal.hide();
        }
    }
    Command::none()
}
