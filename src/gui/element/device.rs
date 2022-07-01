use iced::{Alignment, Color, Column, Container, Element, Length, Row, Text};
use iced_native::widget::Svg;

use crate::{
    gui::{Message, DEFAULT_PADDING},
    usb::serial::models::DeviceDetails,
};

#[derive(Default, Debug, Clone)]
pub struct DeviceView {}

impl DeviceView {
    pub fn view<'a>(&self, device: &'a DeviceDetails) -> Element<'a, Message> {
        // pull the brand for the device
        let model_brand = Svg::from_path(format!(
            "{}/resources/{}-light.svg",
            env!("CARGO_MANIFEST_DIR"),
            device.device_model.trim().to_lowercase(),
        ))
        .width(Length::Units(100));

        // build the brand column
        let status_text: Row<Message> = Row::new()
            .push(Text::new(format!("CONNECTED")).color(Color::from_rgb8(100, 183, 93)))
            .push(Text::new(format!(" - {}", device.device_model)));

        // build the status column
        let status_column: Column<Message> = Column::new()
            .align_items(Alignment::Start)
            // .padding([0, DEFAULT_PADDING])
            .width(Length::Fill)
            .push(status_text)
            .push(Text::new(format!("UID: {}", device.uid)));

        // build the device row
        let device_row: Row<Message> = Row::new()
            .align_items(Alignment::Center)
            .padding(DEFAULT_PADDING)
            .height(Length::Units(55))
            .width(Length::Fill)
            .push(status_column)
            .push(model_brand)
            .into();

        // wrap everything in a container.
        Container::new(device_row).width(Length::Fill).into()
    }
}
