use std::path::PathBuf;

use iced::{
    alignment::Horizontal, button, Alignment, Button, Column, Element, Length, Row, Svg, Text,
};
use iced_aw::{modal, Card, Modal};

use crate::gui::{style, Message, DEFAULT_PADDING, IMAGE_FLEXI_BRIDGE};

#[derive(Default, Clone)]
struct ModalState {
    reset_state: button::State,
    ok_state: button::State,
}

#[derive(Default)]
pub struct ConfirmModal {
    temp_file: PathBuf,
    modal_state: modal::State<ModalState>,
}

impl ConfirmModal {
    pub fn show(&mut self, path: PathBuf) {
        self.temp_file = path;
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
                        Text::new("The binary has been downloaded and is ready to install!")
                            .horizontal_alignment(Horizontal::Center),
                    )
                    .push(
                        Text::new("Next, take a TS or TRS cable and bridge Flexiports 1 and 2")
                            .horizontal_alignment(Horizontal::Center),
                    )
                    .push(Svg::new(IMAGE_FLEXI_BRIDGE.clone()).width(Length::Units(300)))
                    .push(
                        Text::new(
                            "PLEASE DO NOT UNPLUG YOUR DEVICE UNTIL THE INSTALLATION IS FINISHED.",
                        )
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
                        .on_press(Message::EnterBootloader)
                        .padding(DEFAULT_PADDING)
                        .width(Length::Fill)
                        .style(style::Button::SuccessAction),
                    ),
            )
            .style(style::Card::Modal)
            .width(Length::Units(400))
            .on_close(Message::Cancel)
            .into()
        })
        .backdrop(Message::Cancel)
        .on_esc(Message::Cancel)
        .style(style::Modal::Default)
        .into()
    }
}
