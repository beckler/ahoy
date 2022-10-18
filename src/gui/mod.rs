mod element;
mod style;
mod update;
mod usb;
mod view;

use async_std::sync::Mutex;
use futures::{
    channel::mpsc::{Receiver, Sender},
    StreamExt,
};
use iced::{
    button, image, svg, window, Application, Color, Command, Element, Font, Settings, Subscription,
};
use iced_native::subscription;
use lazy_static::lazy_static;
use log::*;
use pirate_midi_rs::check::CheckResponse;
use rusb::Device;
use std::{path::PathBuf, sync::Arc};

use crate::{
    cli::{self, Args},
    command::{
        github::{Asset, Release},
        update::update_available,
        CommandError,
    },
};

use self::{
    element::controls::ControlsView,
    element::{
        confirm_modal::ConfirmModal, device::DeviceView, install::InstallView,
        update_modal::UpdateModal, version::VersionList,
    },
    update::handle_message,
    view::handle_view,
};

/* STATIC RESOURCES */
// images are lazy loaded
lazy_static! {
    pub static ref IMAGE_USB_CABLE_DARK: svg::Handle =
        svg::Handle::from_memory(include_bytes!("../../resources/usb-dark.svg").to_vec());
    pub static ref IMAGE_BRIDGE_6_DARK: svg::Handle =
        svg::Handle::from_memory(include_bytes!("../../resources/bridge6-dark.svg").to_vec());
    pub static ref IMAGE_BRIDGE_4_DARK: svg::Handle =
        svg::Handle::from_memory(include_bytes!("../../resources/bridge4-dark.svg").to_vec());
    pub static ref IMAGE_FLEXI_BRIDGE: svg::Handle =
        svg::Handle::from_memory(include_bytes!("../../resources/wire-bridge.svg").to_vec());
    pub static ref IMAGE_PIRATE_MIDI_LOGO: image::Handle =
        image::Handle::from_memory(include_bytes!("../../resources/pirate-midi-pink.png").to_vec());
}

// DEFAULTS
pub static DEFAULT_PADDING: u16 = 10;
pub static DEFAULT_FONT_SIZE: u16 = 20;
pub static SECONDARY_FONT_SIZE: u16 = 18;
pub static DEFAULT_BORDER_RADIUS: f32 = 6.0;
pub static DEFAULT_HEADING_FONT_SIZE: u16 = 24;
pub static DEFAULT_FONT_COLOR: Color = Color::WHITE;
pub static DEFAULT_FONT: &[u8] = include_bytes!("../../resources/OpenSans-Regular.ttf");
pub static SECONDARY_FONT: Font = Font::External {
    name: "RobotoMono",
    bytes: include_bytes!("../../resources/RobotoMono-Regular.ttf"),
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
    // self-update
    UpdateAvailable(Result<Option<String>, CommandError>),
    UpdateApplication,
    IgnoreUpdate,
    Exit(Result<(), CommandError>),

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
    InstallProgress(f32),
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
    update_modal: UpdateModal,
    install_progress: f32,
    selected_version: Option<Release>,
    installable_asset: Option<PathBuf>,
    reset_button: button::State,
}

#[derive(Default)]
pub(crate) enum DeviceState {
    #[default]
    Disconnected,
    Connected(CheckResponse),
    DFU(
        Option<Device<rusb::Context>>,
        Option<Sender<f32>>,
        Option<Arc<Mutex<Receiver<f32>>>>,
    ),
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
            Command::perform(update_available(), Self::Message::UpdateAvailable),
        )
    }

    fn title(&self) -> String {
        String::from("AHOY! - Pirate MIDI Firmware Updater")
    }

    fn subscription(&self) -> Subscription<Self::Message> {
        let progress_subscription: Subscription<f32> = match &self.device {
            DeviceState::DFU(_, _, channel_recv) => match channel_recv.clone() {
                Some(receiver) => subscription::unfold(
                    std::any::TypeId::of::<Self>(),
                    receiver,
                    |recv| async move {
                        let value = recv.lock().await.next().await;
                        (value.clone(), recv)
                    },
                ),
                None => Subscription::none(),
            },
            _ => Subscription::none(),
        };

        Subscription::batch([
            usb::listener().map(Message::DeviceChangedAction),
            progress_subscription.map(Message::InstallProgress),
        ])
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
