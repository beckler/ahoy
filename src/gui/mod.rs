// mod components;
mod element;
mod style;
mod update;
mod usb;

// Default values used on multiple elements.
pub static DEFAULT_HEADING_FONT_SIZE: u16 = 24;
pub static DEFAULT_FONT_SIZE: u16 = 18;
pub static DEFAULT_PADDING: u16 = 10;

use log::*;

// mod element;
use iced::{
    window, Alignment, Application, Color, Column, Command, Container, Element, Length, Rule,
    Settings, Space, Subscription, Svg, Text,
};
use octocrab::models::repos::Release;

use crate::{
    cli::{self, Args},
    command::CommandError,
    usb::serial::models::DeviceDetails,
};

use self::{
    element::controls::ControlsView,
    element::{device::DeviceView, version::VersionList},
    update::handle_message,
};

pub fn run(args: Args) -> iced::Result {
    let settings = Settings::<cli::Args> {
        window: window::Settings {
            size: (600, 600),
            resizable: true,
            decorations: true,
            ..Default::default()
        },
        // antialiasing: true,
        default_font: Some(include_bytes!("../../resources/SourceCodePro-Regular.ttf")),
        default_text_size: DEFAULT_FONT_SIZE,
        flags: args,
        ..Default::default()
    };

    Ahoy::run(settings)
}

#[derive(Debug, Clone)]
pub enum Message {
    Fetch,
    Retrieved(Result<Vec<Release>, CommandError>),
    FilterChanged(Filter),
    ReleaseSelected(Release),
    DeviceChangedAction(usb::Event),
}

#[derive(Default)]
pub(crate) struct Ahoy {
    debug: bool,
    error: Option<Error>,
    device: Option<DeviceDetails>,
    filter: Filter,
    status: DeviceView,
    controls: ControlsView,
    versions: VersionList,
    releases: Option<Vec<Release>>,
    selected_version: Option<Release>,
}

impl Application for Ahoy {
    type Executor = iced::executor::Default;
    type Message = Message;
    type Flags = cli::Args;

    fn new(flags: Self::Flags) -> (Self, iced::Command<Self::Message>) {
        (
            Ahoy {
                debug: flags.debug,
                ..Default::default()
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        String::from("AHOY! - [UNOFFICIAL] Pirate MIDI Firmware Updater")
    }

    fn subscription(&self) -> Subscription<Self::Message> {
        usb::listener().map(Message::DeviceChangedAction)
    }

    fn update(&mut self, message: Self::Message) -> iced::Command<Self::Message> {
        handle_message(self, message)
    }

    fn view(&mut self) -> Element<Message> {
        // BUILD PRIMARY VIEW
        let inner_content: Element<Message> = if let Some(device) = &self.device {
            /* BUILD PRIMARY COLUMN */
            Column::new()
                .padding(DEFAULT_PADDING)
                .push(self.status.view(&device))
                .push(Rule::horizontal(1))
                // .push(error_message)
                .push(self.controls.view(&self.filter))
                .push(Rule::horizontal(1))
                .push(self.versions.view(
                    &self.error,
                    &self.filter,
                    &self.releases,
                    &self.selected_version,
                ))
                .into()
        } else {
            // if we have no device connect, display the plug icon
            let usb_cable_image = Svg::from_path(format!(
                "{}/resources/usb-light.svg",
                env!("CARGO_MANIFEST_DIR"),
            ))
            .height(Length::Units(400));

            Column::new()
                .align_items(Alignment::Center)
                .push(usb_cable_image)
                .push(Space::new(
                    Length::Units(DEFAULT_PADDING),
                    Length::Units(32),
                ))
                .push(Text::new("Please connect a device").size(DEFAULT_HEADING_FONT_SIZE))
                .into()
        };

        // graphical debugging
        let output = if *&mut self.debug {
            inner_content.explain(Color::BLACK)
        } else {
            inner_content
        };

        // Finally wrap everything in a container.
        Container::new(output)
            .height(Length::Fill)
            .width(Length::Fill)
            // .style(style::Container::Test)
            .into()
    }
}

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
pub enum Filter {
    All,
    #[default]
    Stable,
    PreRelease,
}

impl Filter {
    fn matches(&self, release: &Release) -> bool {
        match self {
            Filter::All => true,
            Filter::Stable => !release.prerelease,
            Filter::PreRelease => release.prerelease,
        }
    }
}

#[derive(Debug, Clone)]
pub enum Error {
    APIError(String),
}

impl From<reqwest::Error> for Error {
    fn from(error: reqwest::Error) -> Error {
        debug!("ERROR: {}", error);

        Error::APIError(error.to_string())
    }
}
impl Error {
    fn to_string(&self) -> String {
        match self {
            Error::APIError(message) => {
                error!("Unable to reach Github. Details: {}", message);
                format!("Error reaching Github!\nRun with `-v` flag to see more details.")
            }
        }
    }
}
