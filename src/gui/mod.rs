mod device;
mod element;

// Default values used on multiple elements.
// pub static DEFAULT_FONT_SIZE: u16 = 16;
// pub static DEFAULT_HEADER_FONT_SIZE: u16 = 19;
pub static DEFAULT_PADDING: u16 = 10;

use log::*;
use std::fmt::Display;

// mod element;
use iced::{
    button, pick_list, scrollable, window, Alignment, Application, Color, Column, Command,
    Container, Element, Length, Row, Scrollable, Settings, Subscription, Text,
};
use octocrab::{models::repos::Release, Page};

use crate::{
    cli::Args,
    command::{
        device::{get_device_details, try_get_serial_port},
        list::retrieve_releases,
        CommandError,
    },
    usb::serial::models::DeviceDetails,
};

use self::element::{device_bar::device_bar, menu_bar::menu_bar};

pub fn run(_args: Args) -> iced::Result {
    let settings = Settings::<()> {
        window: window::Settings {
            size: (600, 400),
            resizable: true,
            decorations: true,
            ..Default::default()
        },
        antialiasing: true,
        default_text_size: 16,
        ..Default::default()
    };

    Ahoy::run(settings)
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Filter {
    All,
    Stable,
    Beta,
}

impl Filter {
    const ALL: [Filter; 3] = [Filter::All, Filter::Stable, Filter::Beta];
}

impl Default for Filter {
    fn default() -> Filter {
        Filter::All
    }
}

impl Display for Filter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", &self)
    }
}

#[derive(Default)]
struct State {
    debug: bool,
    error: Option<Error>,
    fetch: button::State,
    device: Option<DeviceDetails>,
    filter: pick_list::State<Filter>,
    releases: Option<Page<Release>>,
    detail_scroll: scrollable::State,
    version_scroll: scrollable::State,
    selected_filter: Option<Filter>,
}

#[derive(Debug, Clone)]
pub enum Message {
    // Selected(String),
    // Install(Release),
    Fetch,
    Loaded(Result<Page<Release>, CommandError>),
    FilterChanged(Filter),
    DeviceChangedAction(device::Event),
    // LanguageSelected(Language),
    // Loaded(Result<SavedState, LoadError>),
    // Saved(Result<(), SaveError>),
    // InputChanged(String),
    // CreateTask,
    // FilterChanged(Filter),
    // TaskMessage(usize, TaskMessage),
}

enum Ahoy {
    Loading,
    Loaded(State),
    // Errored,
    // Errored {
    //     // error: Error,
    //     try_again: button::State,
    // },
}

impl Default for Ahoy {
    fn default() -> Self {
        Ahoy::Loading
    }
}

impl Application for Ahoy {
    type Executor = iced::executor::Default;
    type Message = Message;
    type Flags = ();

    // async fn device_listener(monitor: UsbMonitor)

    fn new(_flags: Self::Flags) -> (Self, iced::Command<Self::Message>) {
        (Ahoy::Loaded(State::default()), Command::none())
    }

    fn title(&self) -> String {
        match self {
            Ahoy::Loading => String::from("AHOY! - Fetching available versions..."),
            Ahoy::Loaded(_) => String::from("AHOY! - Pirate MIDI Firmware Updater"),
            // Ahoy::Errored => String::from("AHOY - ARRRRG MATEY, SOMETHING WENT WRONG..."),
        }
    }

    fn subscription(&self) -> Subscription<Self::Message> {
        device::listener().map(Message::DeviceChangedAction)
    }

    fn update(&mut self, message: Self::Message) -> iced::Command<Self::Message> {
        match self {
            Ahoy::Loading => {
                match message {
                    Message::Loaded(Ok(releases)) => {
                        debug!("LOADING::RELEASES: {:?}", releases);
                        *self = Ahoy::Loaded(State {
                            debug: true,
                            releases: Some(releases),
                            selected_filter: Some(Filter::All),
                            ..Default::default()
                        });
                    }
                    _ => (),
                }
                Command::none()
            }
            Ahoy::Loaded(state) => {
                match message {
                    Message::Loaded(Ok(releases)) => {
                        state.releases = Some(releases.clone());
                    }
                    Message::Loaded(Err(_)) => {
                        state.error = Some(Error::FetchError);
                    }
                    Message::FilterChanged(filter) => {
                        state.selected_filter = Some(filter.clone());
                    }
                    Message::Fetch => {
                        state.error = None;
                        state.releases = None;
                        info!("refresh requested - attempt to fetch releases...");
                        if let Some(_) = state.device {
                            return Command::perform(retrieve_releases(), Message::Loaded);
                        }
                    }
                    Message::DeviceChangedAction(event) => match event {
                        device::Event::Connect(device) => match try_get_serial_port(&device) {
                            Some(port) => {
                                info!("DEVICE CONNECTED: {:?}", device);
                                match get_device_details(port) {
                                    Ok(details) => {
                                        info!("DEVICE DETAILS: {:?}", details);
                                        state.device = Some(details);
                                        return Command::perform(
                                            retrieve_releases(),
                                            Message::Loaded,
                                        );
                                    }
                                    Err(err) => {
                                        error!("error connecting to device: {:?}", err);
                                        state.device = None;
                                        state.releases = None;
                                    }
                                }
                            }
                            None => (),
                        },
                        device::Event::Disconnect(device) => {
                            info!("DEVICE DISCONNECTED: {:?}", device);
                            state.device = None;
                            state.releases = None;
                        }
                    },
                }
                Command::none()
            } // Ahoy::Errored => Command::none(),
        }
    }

    fn view(&mut self) -> Element<Self::Message> {
        let inner_content: Element<Message> = match self {
            Ahoy::Loading => Text::new("Loading...").into(),
            Ahoy::Loaded(State {
                debug,
                error,
                fetch,
                device,
                filter,
                releases,
                detail_scroll,
                version_scroll,
                selected_filter,
            }) => {
                /* VERSION SELECTOR */
                let version_list: Element<Message> = if let Some(release_page) = releases {
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
                        .push(Text::new("Loading Releases..."))
                        .padding(DEFAULT_PADDING)
                        .height(Length::Fill)
                        .width(Length::Fill)
                        .align_items(Alignment::Center)
                        .into()
                };

                /* VERSION DETAIL */
                let version_detail: Element<Message> = Column::new()
                    // .push(Text::new(format!("{:?} {:?}", releases, error)))
                    .padding(DEFAULT_PADDING)
                    .height(Length::Shrink)
                    .width(Length::Fill)
                    .spacing(5)
                    .into();

                /* VERSION BROWSER (SELECTOR + DETAIL) */
                let version_browser: Element<Message> = Row::new()
                    .padding(DEFAULT_PADDING)
                    .push(Scrollable::new(version_scroll).push(version_list))
                    .push(Scrollable::new(detail_scroll).push(version_detail))
                    .height(Length::Shrink)
                    .into();

                let content: Element<Message> = Column::new()
                    .padding(DEFAULT_PADDING)
                    // .spacing(10)
                    .push(device_bar(device))
                    .push(menu_bar(error, fetch, filter, selected_filter.clone()))
                    .push(version_browser)
                    .into();

                // graphical debugging
                if *debug {
                    content.explain(Color::BLACK)
                } else {
                    content
                }
            } // Ahoy::Errored => Text::new("ERROR").into(),
        };

        // Finally wrap everything in a container.
        Container::new(inner_content)
            .width(Length::Fill)
            .height(Length::Fill)
            // .style(style::NormalBackgroundContainer(color_palette))
            .into()
    }
}

#[derive(Debug, Clone)]
pub enum Error {
    FetchError,
    APIError,
    // LanguageError,
}

impl From<reqwest::Error> for Error {
    fn from(error: reqwest::Error) -> Error {
        dbg!(error);

        Error::APIError
    }
}
impl Error {
    fn to_string(&self) -> String {
        match self {
            Error::FetchError => String::from("ERROR FETCHING RELEASES"),
            Error::APIError => String::from("ERROR REACHING GITHUB"),
        }
    }
}

// #[derive(Debug, Default, Clone)]
// pub struct Controls {
//     all_button: button::State,
//     active_button: button::State,
//     completed_button: button::State,
// }

// impl Controls {
//     // fn view(&mut self, tasks: &[Task], current_filter: Filter) -> Row<Message> {
//     //     let Controls {
//     //         all_button,
//     //         active_button,
//     //         completed_button,
//     //     } = self;

//     //     let tasks_left = tasks.iter().filter(|task| !task.completed).count();

//     //     let filter_button = |state, label, filter, current_filter| {
//     //         let label = Text::new(label).size(16);
//     //         let button = Button::new(state, label).style(style::Button::Filter {
//     //             selected: filter == current_filter,
//     //         });

//     //         button.on_press(Message::FilterChanged(filter)).padding(8)
//     //     };

//     //     Row::new()
//     //         .spacing(20)
//     //         .align_items(Alignment::Center)
//     //         .push(
//     //             Text::new(&format!(
//     //                 "{} {} left",
//     //                 tasks_left,
//     //                 if tasks_left == 1 { "task" } else { "tasks" }
//     //             ))
//     //             .width(Length::Fill)
//     //             .size(16),
//     //         )
//     //         .push(
//     //             Row::new()
//     //                 .width(Length::Shrink)
//     //                 .spacing(10)
//     //                 .push(filter_button(
//     //                     all_button,
//     //                     "All",
//     //                     Filter::All,
//     //                     current_filter,
//     //                 ))
//     //                 .push(filter_button(
//     //                     active_button,
//     //                     "Active",
//     //                     Filter::Active,
//     //                     current_filter,
//     //                 ))
//     //                 .push(filter_button(
//     //                     completed_button,
//     //                     "Completed",
//     //                     Filter::Completed,
//     //                     current_filter,
//     //                 )),
//     //         )
//     // }
// }
