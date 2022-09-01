mod element;
mod style;
mod update;
mod usb;
mod view;

/* STATIC RESOURCES */
// images for light mode
// pub static IMAGE_USB_CABLE_LIGHT: &[u8] = include_bytes!("../../resources/usb-light.svg");
// pub static IMAGE_BRIDGE_6_LIGHT: &[u8] = include_bytes!("../../resources/bridge6-light.svg");
// pub static IMAGE_BRIDGE_4_LIGHT: &[u8] = include_bytes!("../../resources/bridge4-light.svg");

// images for dark mode
pub static IMAGE_USB_CABLE_DARK: &[u8] = include_bytes!("../../resources/usb-dark.svg");
pub static IMAGE_BRIDGE_6_DARK: &[u8] = include_bytes!("../../resources/bridge6-dark.svg");
pub static IMAGE_BRIDGE_4_DARK: &[u8] = include_bytes!("../../resources/bridge4-dark.svg");

// images for both
pub static IMAGE_PIRATE_MIDI_LOGO: &[u8] = include_bytes!("../../resources/pirate-midi-pink.png");

// DEFAULTS
pub static DEFAULT_FONT: &[u8] = include_bytes!("../../resources/OpenSans-Regular.ttf");
pub static SECONDARY_FONT: Font = Font::External {
    name: "RobotoMono",
    bytes: include_bytes!("../../resources/RobotoMono-Regular.ttf"),
};
pub static DEFAULT_PADDING: u16 = 10;
pub static DEFAULT_FONT_SIZE: u16 = 20;
pub static SECONDARY_FONT_SIZE: u16 = 18;
pub static DEFAULT_BORDER_RADIUS: f32 = 6.0;
pub static DEFAULT_HEADING_FONT_SIZE: u16 = 24;
pub static DEFAULT_FONT_COLOR: Color = Color::WHITE;
// Color {
//     r: 0.29,
//     g: 0.29,
//     b: 0.29,
//     a: 1.0,
// };

use iced::{button, window, Application, Color, Command, Element, Font, Settings, Subscription};
use log::*;
use pirate_midi_rs::check::CheckResponse;
use rusb::Device;
use std::path::PathBuf;

use crate::{
    cli::{self, Args},
    command::{
        github::{Asset, Release},
        CommandError,
    },
};

use self::{
    element::controls::ControlsView,
    element::{
        device::DeviceView, install::InstallView, modal::ConfirmModal, version::VersionList,
    },
    update::handle_message,
    view::handle_view,
};

pub fn run(args: Args) -> iced::Result {
    let settings = Settings::<cli::Args> {
        window: window::Settings {
            size: (800, 800),
            resizable: true,
            decorations: true,
            ..Default::default()
        },
        // antialiasing: true,
        exit_on_close_request: true,
        default_font: Some(DEFAULT_FONT),
        default_text_size: DEFAULT_FONT_SIZE,
        flags: args,
        ..Default::default()
    };

    Ahoy::run(settings)
}

#[derive(Debug, Clone)]
pub enum Message {
    // global device
    DeviceChangedAction(usb::Event),

    // release specific
    FetchReleases,
    SelectedRelease(Box<Release>),
    RetrievedReleases(Result<Vec<Release>, CommandError>),
    ReleaseFilterChanged(Filter),

    // prompt
    Cancel,
    EnterBootloader,
    WaitForBootloader(Result<(), CommandError>),
    Install,
    AttemptReset,
    PostInstallResult(Result<(), CommandError>),

    // install specific
    Download(Box<Asset>),
    Downloaded(Result<PathBuf, CommandError>),
}

#[derive(Default)]
pub(crate) struct Ahoy {
    debug: bool,
    error: Option<Error>,
    filter: Filter,
    device: DeviceState,
    status: DeviceView,
    controls: ControlsView,
    releases: Option<Vec<Release>>,
    versions: VersionList,
    installer: InstallView,
    confirm_modal: ConfirmModal,
    selected_version: Option<Release>,
    installable_asset: Option<PathBuf>,
    reset_button: button::State,
}

#[derive(Default, PartialEq)]
pub(crate) enum DeviceState {
    #[default]
    Disconnected,
    Connected {
        device: Device<rusb::Context>,
        details: CheckResponse,
    },
    DFU(Option<Device<rusb::Context>>),
    PostInstall,
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
        String::from("AHOY! - Pirate MIDI Firmware Updater")
    }

    fn subscription(&self) -> Subscription<Self::Message> {
        // subscription::batch([
        usb::listener().map(Message::DeviceChangedAction)
        // ])
    }

    fn update(&mut self, message: Self::Message) -> iced::Command<Self::Message> {
        handle_message(self, message)
    }

    fn view(&mut self) -> Element<Message> {
        handle_view(self)
    }
}

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
pub enum Filter {
    #[default]
    Stable,
    PreRelease,
}

impl Filter {
    fn matches(&self, release: &Release) -> bool {
        match self {
            Filter::Stable => !release.prerelease,
            Filter::PreRelease => release.prerelease,
        }
    }
}

#[derive(thiserror::Error, Debug, Clone)]
pub enum Error {
    #[error("Error reaching Github!\nRun with `-v` flag to see more details. Reason: {0}")]
    RemoteApi(String),
    #[error("Unable to install update! Reason: {0}")]
    Install(String),
}

impl From<surf::Error> for Error {
    fn from(error: surf::Error) -> Error {
        debug!("ERROR: {}", error);
        Error::RemoteApi(error.to_string())
    }
}
