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
    button, pick_list, scrollable, window, Application, Color, Column, Command, Container, Element,
    Length, Row, Scrollable, Settings, Subscription,
};
use octocrab::{models::repos::Release, Page};

use crate::{
    cli::Args,
    command::{device::get_device_details, list::retrieve_releases, CommandError},
    usb::serial::models::DeviceDetails,
};

use self::{
    device::try_get_serial_port,
    element::{device_bar::device_bar, menu_bar::menu_bar, version_list::version_list},
};

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

#[derive(Debug, Clone)]
pub enum Message {
    Fetch,
    Loaded(Result<Page<Release>, CommandError>),
    FilterChanged(Filter),
    DeviceChangedAction(device::Event),
}

#[derive(Default)]
struct Ahoy {
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

impl Application for Ahoy {
    type Executor = iced::executor::Default;
    type Message = Message;
    type Flags = ();

    fn new(_flags: Self::Flags) -> (Self, iced::Command<Self::Message>) {
        (
            Ahoy {
                debug: true,
                selected_filter: Some(Filter::All),
                ..Default::default()
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        String::from("AHOY! - Pirate MIDI Firmware Updater")
    }

    fn subscription(&self) -> Subscription<Self::Message> {
        device::listener().map(Message::DeviceChangedAction)
    }

    fn update(&mut self, message: Self::Message) -> iced::Command<Self::Message> {
        match message {
            Message::Loaded(Ok(releases)) => {
                self.releases = Some(releases.clone());
            }
            Message::Loaded(Err(_)) => {
                self.error = Some(Error::FetchError);
            }
            Message::FilterChanged(filter) => {
                self.selected_filter = Some(filter.clone());
            }
            Message::Fetch => {
                self.error = None;
                self.releases = None;
                info!("refresh requested - attempt to fetch releases...");
                if let Some(_) = self.device {
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
                                self.device = Some(details);
                                return Command::perform(retrieve_releases(), Message::Loaded);
                            }
                            Err(err) => {
                                error!("error connecting to device: {:?}", err);
                                self.device = None;
                                self.releases = None;
                            }
                        }
                    }
                    None => (),
                },
                device::Event::Disconnect(device) => {
                    info!("DEVICE DISCONNECTED: {:?}", device);
                    self.device = None;
                    self.releases = None;
                }
            },
        }
        Command::none()
    }

    fn view(&mut self) -> Element<Message> {
        let inner_content: Element<Message> = {
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
                .push(
                    Scrollable::new(&mut self.version_scroll)
                        .push(version_list(&mut self.releases)),
                )
                .push(Scrollable::new(&mut self.detail_scroll).push(version_detail))
                .height(Length::Shrink)
                .into();

            let content: Element<Message> = Column::new()
                .padding(DEFAULT_PADDING)
                // .spacing(10)
                .push(device_bar(&mut self.device))
                .push(menu_bar(
                    &mut self.error,
                    &mut self.fetch,
                    &mut self.filter,
                    self.selected_filter.clone(),
                ))
                .push(version_browser)
                .into();

            // graphical debugging
            if *&mut self.debug {
                content.explain(Color::BLACK)
            } else {
                content
            }
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
