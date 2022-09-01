use crate::gui::{Message, DEFAULT_PADDING};
use iced::{Alignment, Color, Column, Container, Element, Length, ProgressBar, Row, Space, Text};
use log::*;
use rusb::{Context, Device};

#[derive(Default, Debug, Clone)]
pub struct InstallView {
    progress: f32,
}

impl InstallView {
    pub fn increment_progress(&mut self, amount: f32) {
        trace!("progress increment: {}", amount);
        self.progress += amount
    }

    pub fn view<'a>(&self, dfu: &Option<Device<Context>>) -> Element<'a, Message> {
        let status_text: Row<Message> = if dfu.is_some() {
            Row::new().push(Text::new("CONNECTED").color(Color::from_rgb8(100, 183, 93)))
        } else {
            Row::new().push(Text::new("WAITING FOR DEVICE").color(Color::from_rgb8(142, 110, 34)))
        };

        let message_text: Row<Message> = if dfu.is_some() {
            if self.progress > 0.1 {
                Row::new().push(Text::new("Installing firmware..."))
            } else {
                Row::new().push(Text::new("Preparing device for new firmware..."))
            }
        } else {
            Row::new().push(Text::new("Waiting for device to enter bootloader mode..."))
        };

        // progress bar
        let progress_bar: Element<Message> = if self.progress < 0.1 {
            Space::new(Length::Units(0), Length::Units(0)).into()
        } else {
            ProgressBar::new(0.0..=100.0, self.progress).into()
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
