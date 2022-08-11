mod element;
mod style;
mod update;
mod usb;
mod view;

/* STATIC RESOURCES */
// images for light mode
pub static IMAGE_USB_CABLE_LIGHT: &[u8] = include_bytes!("../../resources/usb-light.svg");
pub static IMAGE_BRIDGE_6_LIGHT: &[u8] = include_bytes!("../../resources/bridge6-light.svg");
pub static IMAGE_BRIDGE_4_LIGHT: &[u8] = include_bytes!("../../resources/bridge4-light.svg");

// images for dark mode
// pub static IMAGE_USB_CABLE_DARK: &[u8] = include_bytes!("../../resources/usb-dark.svg");
// pub static IMAGE_BRIDGE_6_DARK: &[u8] = include_bytes!("../../resources/bridge6-dark.svg");
// pub static IMAGE_BRIDGE_4_DARK: &[u8] = include_bytes!("../../resources/bridge4-dark.svg");

pub static DEFAULT_FONT: &[u8] = include_bytes!("../../resources/RobotoMono-Regular.ttf");
pub static DEFAULT_PADDING: u16 = 10;
pub static DEFAULT_FONT_SIZE: u16 = 18;
pub static DEFAULT_BORDER_RADIUS: f32 = 6.0;
pub static DEFAULT_HEADING_FONT_SIZE: u16 = 24;
pub static DEFAULT_FONT_COLOR: Color = Color {
    r: 0.29,
    g: 0.29,
    b: 0.29,
    a: 1.0,
};

use iced::{window, Application, Color, Command, Element, Settings, Subscription};
use log::*;
use octocrab::models::repos::{Asset, Release};
use std::path::PathBuf;

use crate::{
    cli::{self, Args},
    command::{CommandError, UsbConnection},
    usb::observer::UsbDevice,
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
        antialiasing: true,
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
    PostInstall(Result<(), CommandError>),

    // install specific
    Download(Box<Asset>),
    Downloaded(Result<PathBuf, CommandError>),
    // DownloadProgressed((usize, download::Progress)),
    // InstallProgressed((usize, download::Progress)),
}

// #[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
// pub enum Status {
//     #[default]
//     Initial,
// InBootloader,
// Installing,
// Installed,
// }

#[derive(Default)]
pub(crate) struct Ahoy {
    debug: bool,
    error: Option<Error>,
    filter: Filter,
    status: DeviceView,
    controls: ControlsView,
    releases: Option<Vec<Release>>,
    versions: VersionList,
    connection: Option<UsbConnection>,
    installer: InstallView,
    installing: bool,
    post_install: bool,
    confirm_modal: ConfirmModal,
    dfu_connection: Option<UsbDevice>,
    selected_version: Option<Release>,
    installable_asset: Option<PathBuf>,
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
        // match self.
        usb::listener().map(Message::DeviceChangedAction)
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

#[derive(thiserror::Error, Debug, Clone)]
pub enum Error {
    #[error("Error reaching Github!\nRun with `-v` flag to see more details. Reason: {0}")]
    RemoteApi(String),
    #[error("Unable to install update! Reason: {0}")]
    Install(String),
}

impl From<reqwest::Error> for Error {
    fn from(error: reqwest::Error) -> Error {
        debug!("ERROR: {}", error);
        Error::RemoteApi(error.to_string())
    }
}
