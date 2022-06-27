use iced::{Alignment, Column, Element, Length, Text};
use octocrab::{models::repos::Release, Page};

use crate::gui::{Message, DEFAULT_PADDING};

/* VERSION SELECTOR */
pub fn version_list<'a>(releases: &mut Option<Page<Release>>) -> Element<'a, Message> {
    if let Some(release_page) = releases {
        release_page
            .items
            .iter_mut()
            .enumerate()
            .fold(
                Column::new().padding(DEFAULT_PADDING).spacing(5),
                |column, (_i, release)| {
                    let unknown = String::from("UNKNOWN");
                    let name = match &release.name {
                        Some(name) => name,
                        None => &unknown,
                    };

                    // column.push(Button::new(button::State::new()))

                    column.push(Text::new(name))
                },
            )
            .height(Length::Shrink)
            .width(Length::Shrink)
            .into()
    } else {
        Column::new()
            .push(Text::new("Connect a device"))
            .padding(DEFAULT_PADDING)
            .height(Length::Fill)
            .width(Length::Fill)
            .align_items(Alignment::Center)
            .into()
    }
}
