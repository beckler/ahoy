// mod components;
mod element;
mod style;
mod update;
mod usb;

/* STATIC RESOURCES */
// Default values used on multiple elements.
pub static DEFAULT_HEADING_FONT_SIZE: u16 = 24;
pub static DEFAULT_FONT_SIZE: u16 = 18;
pub static DEFAULT_PADDING: u16 = 10;

// fonts
pub static FONT_SOURCE_CODE_PRO: &[u8] =
    include_bytes!("../../resources/SourceCodePro-Regular.ttf");

// for light mode
pub static IMAGE_USB_CABLE_LIGHT: &[u8] = include_bytes!("../../resources/usb-light.svg");
pub static IMAGE_BRIDGE_6_LIGHT: &[u8] = include_bytes!("../../resources/bridge6-light.svg");
pub static IMAGE_BRIDGE_4_LIGHT: &[u8] = include_bytes!("../../resources/bridge4-light.svg");

// for dark mode
// pub static IMAGE_USB_CABLE_DARK: &[u8] = include_bytes!("../../resources/usb-dark.svg");
// pub static IMAGE_BRIDGE_6_DARK: &[u8] = include_bytes!("../../resources/bridge6-dark.svg");
// pub static IMAGE_BRIDGE_4_DARK: &[u8] = include_bytes!("../../resources/bridge4-dark.svg");

use log::*;

// mod element;
use iced::{
    svg::Handle, window, Alignment, Application, Color, Column, Command, Container, Element,
    Length, Row, Rule, Settings, Space, Subscription, Svg, Text,
};
use octocrab::models::repos::Release;

use crate::{
    cli::{self, Args},
    command::CommandError,
    usb::serial::models::DeviceDetails,
};

use self::{
    element::controls::ControlsView,
    element::{device::DeviceView, modal::InstallModal, version::VersionList},
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
        default_font: Some(FONT_SOURCE_CODE_PRO),
        default_text_size: DEFAULT_FONT_SIZE,
        flags: args,
        ..Default::default()
    };

    Ahoy::run(settings)
}

#[derive(Debug, Clone)]
pub enum Message {
    Fetch,
    Reset,
    Cancel,
    Prompt,
    Install,
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
    install_modal: InstallModal,
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
            /* WHEN A DEVICE IS CONNECTED */
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
            /* WHEN A DEVICE IS NOT CONNECTED */
            let usb_cable_image = Svg::new(Handle::from_memory(IMAGE_USB_CABLE_LIGHT.clone()))
                .height(Length::Units(400));

            let bridge6 = Svg::new(Handle::from_memory(IMAGE_BRIDGE_6_LIGHT.clone()))
                .width(Length::Units(100));

            let bridge4 = Svg::new(Handle::from_memory(IMAGE_BRIDGE_4_LIGHT.clone()))
                .width(Length::Units(100));

            Column::new()
                .align_items(Alignment::Center)
                .spacing(DEFAULT_PADDING)
                .width(Length::Fill)
                .push(usb_cable_image)
                .push(Space::with_height(Length::Units(DEFAULT_PADDING * 2)))
                .push(Text::new("Please connect either a:").size(DEFAULT_HEADING_FONT_SIZE))
                .push(
                    Row::new()
                        .align_items(Alignment::Center)
                        .spacing(DEFAULT_PADDING)
                        .push(bridge6)
                        .push(Text::new("or").size(DEFAULT_HEADING_FONT_SIZE))
                        .push(bridge4),
                )
                .into()
        };

        // wrap modal around the inner content
        let modal_wrapped_content = self.install_modal.view(inner_content);

        // setup graphical debugging
        let output = if *&mut self.debug {
            modal_wrapped_content.explain(Color::BLACK)
        } else {
            modal_wrapped_content
        };

        // finally wrap everything in a container.
        Container::new(output)
            .height(Length::Fill)
            .width(Length::Fill)
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
