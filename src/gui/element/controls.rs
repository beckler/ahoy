use iced::{alignment::Horizontal, button, Alignment, Button, Element, Length, Row, Space, Text};

use crate::gui::{style, Filter, Message, DEFAULT_PADDING};

#[derive(Debug, Default, Clone)]
pub struct ControlsView {
    fetch_button: button::State,
    all_button: button::State,
    stable_button: button::State,
    prerelease_button: button::State,
}

impl ControlsView {
    pub fn view(&mut self, filter: &Filter) -> Element<Message> {
        let ControlsView {
            fetch_button,
            all_button,
            stable_button,
            prerelease_button,
        } = self;

        let filter_button = |state, label, filter, &current_filter| {
            let label = Text::new(label).horizontal_alignment(Horizontal::Center);
            let button = Button::new(state, label)
                .padding(DEFAULT_PADDING)
                .width(Length::Units(75))
                .style(if filter == current_filter {
                    style::Button::FilterSelected
                } else {
                    style::Button::FilterActive
                });

            button.on_press(Message::ReleaseFilterChanged(filter))
        };

        let refresh_button: Element<Message> = Button::new(
            fetch_button,
            Text::new("Refresh").horizontal_alignment(Horizontal::Center),
        )
        .on_press(Message::FetchReleases)
        .padding(DEFAULT_PADDING)
        .width(Length::Units(100))
        .style(style::Button::SuccessAction)
        .into();

        Row::new()
            .align_items(Alignment::Center)
            .spacing(10)
            .padding(DEFAULT_PADDING)
            .push(
                Row::new()
                    .spacing(DEFAULT_PADDING)
                    .width(Length::Shrink)
                    .push(filter_button(
                        stable_button,
                        "Stable",
                        Filter::Stable,
                        filter,
                    ))
                    .push(filter_button(
                        prerelease_button,
                        "Beta",
                        Filter::PreRelease,
                        filter,
                    ))
                    .push(filter_button(all_button, "All", Filter::All, filter)),
            )
            .push(Space::new(Length::Fill, Length::Shrink))
            .push(refresh_button)
            .into()
    }
}
