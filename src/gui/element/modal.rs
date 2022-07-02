use iced::{alignment::Horizontal, button, Button, Element, Length, Row, Text};
use iced_aw::{modal, Card, Modal};

use crate::gui::{style, Message, DEFAULT_PADDING};

#[derive(Default)]
struct ModalState {
    reset_state: button::State,
    ok_state: button::State,
}

#[derive(Default)]
pub struct InstallModal {
    modal_state: modal::State<ModalState>,
}

impl InstallModal {
    pub fn show(&mut self) {
        self.modal_state.show(true)
    }

    pub fn hide(&mut self) {
        self.modal_state.show(false)
    }

    pub fn view<'a>(&'a mut self, content: Element<'a, Message>) -> Element<'a, Message> {
        Modal::new(&mut self.modal_state, content, |state| {
            Card::new(Text::new("My modal"), Text::new("This is a modal!"))
                .foot(
                    Row::new()
                        .spacing(10)
                        .padding(5)
                        .width(Length::Fill)
                        .push(
                            Button::new(
                                &mut state.reset_state,
                                Text::new("Cancel").horizontal_alignment(Horizontal::Center),
                            )
                            .on_press(Message::Cancel)
                            .padding(DEFAULT_PADDING)
                            .width(Length::Fill)
                            .style(style::Button::CancelAction),
                        )
                        .push(
                            Button::new(
                                &mut state.ok_state,
                                Text::new("Install").horizontal_alignment(Horizontal::Center),
                            )
                            .on_press(Message::Install)
                            .padding(DEFAULT_PADDING)
                            .width(Length::Fill)
                            .style(style::Button::ReleaseSelected),
                        ),
                )
                .style(style::Card::Modal)
                .max_width(300)
                .on_close(Message::Cancel)
                .into()
        })
        .backdrop(Message::Cancel)
        .on_esc(Message::Cancel)
        .into()
    }
}
