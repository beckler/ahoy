use iced::{alignment::Horizontal, button, Alignment, Button, Column, Element, Length, Row, Text};
use iced_aw::{modal, Card, Modal};

use crate::gui::{style, Message, DEFAULT_PADDING};

#[derive(Default, Clone)]
struct ModalState {
    reset_state: button::State,
    ok_state: button::State,
}

#[derive(Default)]
pub struct UpdateModal {
    new_version: String,
    modal_state: modal::State<ModalState>,
}

impl UpdateModal {
    pub fn show(&mut self, new_version: String) {
        self.new_version = new_version;
        self.modal_state.show(true)
    }

    pub fn hide(&mut self) {
        self.modal_state.show(false)
    }

    pub fn view<'a>(&'a mut self, content: Element<'a, Message>) -> Element<'a, Message> {
        Modal::new(&mut self.modal_state, content, |state| {
            Card::new(
                Text::new(String::new()),
                Column::new()
                    .spacing(DEFAULT_PADDING)
                    .align_items(Alignment::Center)
                    .push(
                        Text::new("There is an update available for this application!")
                            .horizontal_alignment(Horizontal::Center),
                    )
                    .push(
                        Text::new(format!("Version: {} is now available!", self.new_version))
                            .horizontal_alignment(Horizontal::Center),
                    ),
            )
            .padding_body(DEFAULT_PADDING.into())
            .foot(
                Row::new()
                    .spacing(DEFAULT_PADDING)
                    .padding(DEFAULT_PADDING / 2)
                    .width(Length::Fill)
                    .push(
                        Button::new(
                            &mut state.reset_state,
                            Text::new("Dismiss").horizontal_alignment(Horizontal::Center),
                        )
                        .on_press(Message::IgnoreUpdate)
                        .padding(DEFAULT_PADDING)
                        .width(Length::Fill)
                        .style(style::Button::CancelAction),
                    )
                    .push(
                        Button::new(
                            &mut state.ok_state,
                            Text::new("Update and Quit").horizontal_alignment(Horizontal::Center),
                        )
                        .on_press(Message::UpdateApplication)
                        .padding(DEFAULT_PADDING)
                        .width(Length::Fill)
                        .style(style::Button::SuccessAction),
                    ),
            )
            .style(style::Card::Modal)
            .width(Length::Units(400))
            .on_close(Message::IgnoreUpdate)
            .into()
        })
        .on_esc(Message::IgnoreUpdate)
        .style(style::Modal::Default)
        .into()
    }
}
