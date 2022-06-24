use iced::{Alignment, Column, Container, Element, Length, Row, Text};

use crate::{gui::Message, usb::serial::models::DeviceDetails};

pub fn device_bar<'a>(device: &'a mut Option<DeviceDetails>) -> Container<'a, Message> {
    let device_row: Row<Message> = Row::new()
        .height(Length::Units(100))
        .width(Length::Fill)
        .push(build_detail(device))
        .align_items(Alignment::Center)
        .into();

    // Wrap everything in a container.
    Container::new(device_row).width(Length::Fill)
}

fn build_detail<'a>(device: &'a mut Option<DeviceDetails>) -> Element<Message> {
    let detail_column: Column<Message> = Column::new().align_items(Alignment::Center);

    let detail_column: Column<Message> = if let Some(detail) = device {
        detail_column
            .push(Text::new(format!("Name: {}", detail.manufacturer)))
            .push(Text::new(format!("Model: {}", detail.device_model)))
            .push(Text::new(format!("UID: {}", detail.uid)))
            .push(Text::new(format!("Device Name: {}", detail.device_name)))
            .push(Text::new(format!("Profile ID: {}", detail.profile_id)))
    } else {
        detail_column.push(Text::new(format!("No device found")))
    };

    detail_column.into()
}
