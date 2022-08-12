use iced::{svg::Handle, Alignment, Color, Column, Container, Element, Length, Row, Text};
use iced_native::widget::Svg;

use crate::gui::{
    Message, UsbConnection, DEFAULT_PADDING, IMAGE_BRIDGE_4_LIGHT, IMAGE_BRIDGE_6_LIGHT,
};

#[derive(Default, Debug, Clone)]
pub struct DeviceView {}

impl DeviceView {
    pub fn view<'a>(&self, conn: &'a UsbConnection) -> Element<'a, Message> {
        // pull the brand for the device
        let model_brand = match conn.details.device_model.trim().to_lowercase().as_str() {
            "bridge4" => Svg::new(Handle::from_memory(IMAGE_BRIDGE_4_LIGHT)),
            _ => Svg::new(Handle::from_memory(IMAGE_BRIDGE_6_LIGHT)),
        }
        .width(Length::Units(100));

        // build the brand column
        let status_text: Row<Message> = Row::new()
            .push(Text::new("CONNECTED").color(Color::from_rgb8(100, 183, 93)))
            .push(Text::new(format!(" - {}", conn.details.device_name)));

        // build the status column
        let status_column: Column<Message> = Column::new()
            .align_items(Alignment::Start)
            // .padding([0, DEFAULT_PADDING])
            .width(Length::Fill)
            .push(status_text)
            .push(Text::new(format!("UID: {}", conn.details.uid)));

        // build the device row
        let device_row: Element<Message> = Row::new()
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
