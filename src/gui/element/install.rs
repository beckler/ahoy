use std::sync::{Arc, Mutex};

use iced::{Alignment, Color, Column, Container, Element, Length, ProgressBar, Row, Text};

use crate::{
    gui::{Message, DEFAULT_PADDING},
    usb::observer::UsbDevice,
};

#[derive(Default, Debug, Clone)]
pub struct InstallView {
    progress: Arc<Mutex<f32>>,
}

impl InstallView {
    pub fn increment_progress(&self, amount: usize) {
        let mut state = self.progress.lock().unwrap();
        *state = amount as f32;
    }

    pub fn view<'a>(&self, dfu: &Option<UsbDevice>) -> Element<'a, Message> {
        // progress bar
        let progress_bar = ProgressBar::new(0.0..=100.0, *self.progress.lock().unwrap());

        let status_text: Row<Message> = if dfu.is_some() {
            Row::new().push(Text::new(format!("CONNECTED")).color(Color::from_rgb8(100, 183, 93)))
        } else {
            Row::new().push(
                Text::new(format!("WAITING FOR DEVICE")).color(Color::from_rgb8(142, 110, 34)),
            )
        };

        let message_text: Row<Message> = if dfu.is_some() {
            Row::new().push(Text::new(format!("Installing...")))
        } else {
            Row::new().push(Text::new(format!(
                "Waiting for device to enter bootloader mode..."
            )))
        };

        let primary_column: Column<Message> = Column::new()
            .padding(DEFAULT_PADDING)
            .spacing(DEFAULT_PADDING)
            .push(status_text)
            .push(message_text)
            .push(progress_bar)
            .align_items(Alignment::Center)
            .width(Length::Fill);

        // wrap everything in a container.
        Container::new(primary_column).width(Length::Fill).into()
    }
}
