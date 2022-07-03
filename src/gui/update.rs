use iced::Command;
use log::*;

use crate::command::{device::get_device_details, list::fetch_releases};

use super::{
    usb::{self, try_get_serial_port},
    Ahoy, Error, Message,
};

pub(crate) fn handle_message(ahoy: &mut Ahoy, message: Message) -> Command<Message> {
    match message {
        Message::Fetch => {
            ahoy.error = None;
            ahoy.releases = None;
            ahoy.selected_version = None;
            info!("refresh requested - attempt to fetch releases...");
            if let Some(_) = ahoy.device {
                return Command::perform(fetch_releases(), Message::Retrieved);
            }
        }
        Message::Cancel => ahoy.install_modal.hide(),
        Message::Prompt => ahoy.install_modal.show(),
        Message::Install => (),
        Message::Retrieved(Ok(releases)) => {
            // grab first version that matches the filter
            ahoy.selected_version = releases
                .iter()
                .cloned()
                .find(|rel| ahoy.filter.matches(rel));

            // set our releases
            ahoy.releases = Some(releases);
        }
        Message::Retrieved(Err(err)) => ahoy.error = Some(Error::APIError(err.to_string())),
        Message::FilterChanged(filter) => ahoy.filter = filter,
        Message::ReleaseSelected(release) => ahoy.selected_version = Some(release),
        Message::DeviceChangedAction(event) => match event {
            usb::Event::Connect(device) => match try_get_serial_port(&device) {
                Some(port) => {
                    info!("DEVICE CONNECTED: {:?}", device);
                    match get_device_details(port) {
                        Ok(details) => {
                            info!("DEVICE DETAILS: {:?}", details);
                            ahoy.device = Some(details);

                            // retrieve the releases
                            return Command::perform(fetch_releases(), Message::Retrieved);
                        }
                        Err(err) => {
                            error!("error connecting to device: {:?}", err);
                            ahoy.device = None;
                            ahoy.releases = None;
                        }
                    }
                }
                None => (),
            },
            usb::Event::Disconnect(device) => {
                info!("DEVICE DISCONNECTED: {:?}", device);
                ahoy.device = None;
                ahoy.releases = None;
                ahoy.selected_version = None;
            }
        },
    }
    Command::none()
}
