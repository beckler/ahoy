use std::fs::remove_file;

use iced::Command;
use log::*;

use crate::{
    command::{enter_bootloader, fetch_asset, fetch_releases, UsbConnection},
    usb::serial::PirateMIDISerialDevice,
};

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
        Message::Cancel(path) => {
            // delete the file
            if path.exists() {
                info!("canceled - deleting file: {}", path.display());
                match remove_file(path) {
                    Ok(_) => (),
                    Err(err) => error!("unable to delete file: {}", err.to_string()),
                }
            }
            ahoy.install_modal.hide()
        }
        Message::EnterBootloader(path) => {
            // hide the modal
            ahoy.install_modal.hide();

            // ensure the file exists
            if path.exists() {
                // set the install flag to true
                ahoy.installing = true;

                // enter the bootloader
                match &ahoy.connection {
                    Some(conn) => {
                        return Command::perform(
                            enter_bootloader(conn.clone(), path),
                            Message::Install,
                        );
                    }
                    None => {
                        ahoy.installing = false;
                        panic!("should not be able to reach this state - so I have no idea wtf happened!")
                    }
                }
            } else {
                ahoy.error = Some(Error::InstallError(format!(
                    "unable to locate file: {}",
                    path.display()
                )))
            }
        }
        Message::Install(Ok(path)) => {}
        Message::Install(Err(err)) => {}
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
        Message::DeviceChangedAction(event) => match event {
            usb::Event::Connect(device) => {
                if ahoy.installing {
                    // don't know what to do here
                } else {
                    match device.try_get_serial_port() {
                        Some(port) => {
                            info!("DEVICE CONNECTED: {:?}", device);

                            match PirateMIDISerialDevice::get_device_details(port.clone()) {
                                Ok(details) => {
                                    info!("DEVICE DETAILS: {:?}", details);
                                    ahoy.connection =
                                        Some(UsbConnection::new(port, device, details));

                                    // retrieve releases if we have a valid device
                                    return Command::perform(
                                        fetch_releases(),
                                        Message::RetrievedReleases,
                                    );
                                }
                                Err(err) => {
                                    error!("error connecting to device: {:?}", err);
                                    ahoy.connection = None;
                                    ahoy.releases = None;
                                }
                            }
                        }
                        None => (),
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
        Message::Download(asset) => {
            return Command::perform(fetch_asset(asset), Message::Downloaded);
        }
        Message::Downloaded(Ok(path)) => {
            ahoy.install_modal.show(path);
        }
        Message::Downloaded(Err(err)) => ahoy.error = Some(Error::APIError(err.to_string())),
    }
    Command::none()
}
