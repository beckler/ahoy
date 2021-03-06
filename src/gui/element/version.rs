use iced::{
    alignment::Horizontal, button, scrollable, Alignment, Button, Column, Container, Element,
    Length, Row, Rule, Scrollable, Space, Text,
};
use octocrab::models::repos::Release;

use crate::{
    command::UsbConnection,
    gui::{
        self,
        style::{self},
        Filter, Message, DEFAULT_PADDING,
    },
};

#[derive(Default, Debug, Clone)]
struct ReleaseSelector {
    release: Option<Release>,
    state: button::State,
}

#[derive(Default, Debug, Clone)]
pub struct VersionList {
    button_states: Vec<ReleaseSelector>,
    detail_scroll: scrollable::State,
    version_scroll: scrollable::State,
    install_button: button::State,
}

impl VersionList {
    pub fn view<'a>(
        &'a mut self,
        error: &'a Option<gui::Error>,
        filter: &'a Filter,
        releases: &'a Option<Vec<Release>>,
        connection: &'a UsbConnection,
        selected_release: &'a Option<Release>,
    ) -> Element<'a, Message> {
        let version_row: Element<Message> = if let Some(error) = error {
            /* ERROR DISPLAY */
            Column::new()
                .padding(DEFAULT_PADDING)
                .push(
                    Container::new(
                        Text::new(error.to_string())
                            .horizontal_alignment(Horizontal::Center)
                            .width(Length::Fill),
                    )
                    .padding(DEFAULT_PADDING)
                    .width(Length::Fill)
                    .style(style::Container::Error),
                )
                .into()
        } else {
            /* RELEASE LIST */
            if let Some(release_list) = releases {
                // convert our release list to a format that we can display
                self.button_states = release_list.iter().fold(vec![], |mut selector, release| {
                    selector.push(ReleaseSelector {
                        release: Some(release.clone()),
                        state: button::State::default(),
                    });
                    selector
                });

                // build our selectable version column
                let release_selection_column = self.button_states.iter_mut().fold(
                    Column::new()
                        .padding(DEFAULT_PADDING)
                        .spacing(DEFAULT_PADDING),
                    |column, version| {
                        let release = version
                            .release
                            .as_ref()
                            .expect("something went terribly wrong!");

                        if filter.matches(release) {
                            column.push(
                                Button::new(
                                    &mut version.state,
                                    Text::new(release.tag_name.clone())
                                        .horizontal_alignment(Horizontal::Center),
                                )
                                .on_press(Message::SelectedRelease(release.clone()))
                                .padding(DEFAULT_PADDING)
                                .width(Length::Units(130))
                                .style(
                                    // TODO: clean up this abomination
                                    if let Some(selected) = selected_release {
                                        if release.id == selected.id {
                                            if release.prerelease {
                                                style::Button::PreReleaseSelected
                                            } else {
                                                style::Button::ReleaseSelected
                                            }
                                        } else {
                                            if release.prerelease {
                                                style::Button::PreRelease
                                            } else {
                                                style::Button::Release
                                            }
                                        }
                                    } else {
                                        if release.prerelease {
                                            style::Button::PreRelease
                                        } else {
                                            style::Button::Release
                                        }
                                    },
                                ),
                            )
                        } else {
                            column
                        }
                    },
                );

                let release_selected_detail: Element<Message> = match selected_release {
                    Some(selected) => {
                        let selected_asset = selected.assets.iter().find(|asset| {
                            asset.name.contains(
                                connection
                                    .details
                                    .device_model
                                    .trim()
                                    .to_lowercase()
                                    .as_str(),
                            )
                        });

                        let install_bar = Row::new()
                            .align_items(Alignment::Center)
                            .padding([DEFAULT_PADDING, 0, 0, 0])
                            .height(Length::Shrink)
                            .width(Length::Fill);

                        let install_bar = match selected_asset {
                            Some(asset) => install_bar
                                // .push(Text::new(format!("{}", asset.name)))
                                .push(Space::with_width(Length::Fill))
                                .push(
                                    Button::new(
                                        &mut self.install_button,
                                        Text::new("Download and Install")
                                            .horizontal_alignment(Horizontal::Center),
                                    )
                                    .on_press(Message::Download(asset.clone()))
                                    .padding(DEFAULT_PADDING)
                                    .width(Length::Units(250))
                                    .style(style::Button::ReleaseSelected),
                                ),
                            None => install_bar.push(Text::new(
                                "No assets are available for download for this device",
                            )),
                        };

                        Column::new()
                            .padding(DEFAULT_PADDING)
                            .spacing(DEFAULT_PADDING / 2)
                            .height(Length::Fill)
                            .width(Length::Fill)
                            .push(Text::new(selected.name.clone().unwrap_or_default()))
                            .push(Rule::horizontal(1))
                            .push(
                                Scrollable::new(&mut self.detail_scroll)
                                    .height(Length::Fill)
                                    .push(Text::new(selected.body.clone().unwrap_or_default())),
                            )
                            .push(Rule::horizontal(1))
                            .push(install_bar)
                            .into()
                    }
                    None => Container::new(Text::new("Please select a release"))
                        .center_x()
                        .center_y()
                        .height(Length::Fill)
                        .width(Length::Fill)
                        .into(),
                };

                // build our row
                Row::new()
                    .padding([0, DEFAULT_PADDING])
                    .push(Scrollable::new(&mut self.version_scroll).push(release_selection_column))
                    .push(Rule::vertical(1))
                    .push(release_selected_detail)
                    .into()
            } else {
                // loading message
                Row::new()
                    .align_items(Alignment::Center)
                    .padding(DEFAULT_PADDING)
                    .height(Length::Fill)
                    .width(Length::Fill)
                    .push(Space::new(Length::Fill, Length::Shrink))
                    .push(Text::new("Loading..."))
                    .push(Space::new(Length::Fill, Length::Shrink))
                    .into()
            }
        };

        Container::new(version_row).width(Length::Fill).into()
    }
}
