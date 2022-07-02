use iced::{alignment::Horizontal, button, Alignment, Button, Element, Length, Row, Space, Text};

use crate::gui::{style, Message, DEFAULT_PADDING};

#[derive(Default, Debug, Clone)]
pub struct InstallBar {
    reset_button: button::State,
    install_button: button::State,
}

impl InstallBar {
    pub fn view<'a>(&'a mut self) -> Element<'a, Message> {
        Row::new()
            .align_items(Alignment::Center)
            .padding([DEFAULT_PADDING, 0, 0, 0])
            .height(Length::Shrink)
            .width(Length::Fill)
            .push(
                Button::new(
                    &mut self.reset_button,
                    Text::new("Reset").horizontal_alignment(Horizontal::Center),
                )
                .on_press(Message::Reset)
                .padding(DEFAULT_PADDING)
                .width(Length::Units(100))
                .style(style::Button::CancelAction),
            )
            .push(Space::with_width(Length::Fill))
            .push(
                Button::new(
                    &mut self.install_button,
                    Text::new("Install").horizontal_alignment(Horizontal::Center),
                )
                .on_press(Message::Prompt)
                .padding(DEFAULT_PADDING)
                .width(Length::Units(100))
                .style(style::Button::ReleaseSelected),
            )
            .into()
    }
}
