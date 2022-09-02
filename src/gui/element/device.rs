use iced::{Alignment, Color, Column, Container, Element, Length, Row, Text};
use iced_native::widget::Svg;
use pirate_midi_rs::check::CheckResponse;

use crate::gui::{
    Message, DEFAULT_PADDING, IMAGE_BRIDGE_4_DARK, IMAGE_BRIDGE_6_DARK, SECONDARY_FONT,
    SECONDARY_FONT_SIZE,
};

#[derive(Default, Debug, Clone)]
pub struct DeviceView {}

impl DeviceView {
    pub fn view<'a>(&self, conn: &'a CheckResponse) -> Element<'a, Message> {
        // pull the brand for the device
        let model_brand = match conn.device_model.trim().to_lowercase().as_str() {
            "bridge4" => Svg::new(IMAGE_BRIDGE_4_DARK.clone()),
            _ => Svg::new(IMAGE_BRIDGE_6_DARK.clone()),
        }
        .width(Length::Units(100));

        // build the brand column
        let status_text: Row<Message> = Row::new()
            .push(
                Text::new("CONNECTED")
                    .color(Color::from_rgb8(100, 183, 93))
                    .font(SECONDARY_FONT)
                    .size(SECONDARY_FONT_SIZE),
            )
            .push(
                Text::new(format!(" - {}", conn.device_name))
                    .font(SECONDARY_FONT)
                    .size(SECONDARY_FONT_SIZE),
            );

        // build the status column
        let status_column: Column<Message> = Column::new()
            .align_items(Alignment::Start)
            // .padding([0, DEFAULT_PADDING])
            .width(Length::Fill)
            .push(status_text)
            .push(
                Text::new(format!("UID: {}", conn.uid))
                    .font(SECONDARY_FONT)
                    .size(SECONDARY_FONT_SIZE),
            );

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
