mod update;
mod view;

use log::*;
use octocrab::models::repos::{Asset, Release};
use std::path::PathBuf;

use iced::Element;
use iced_lazy::{self, Component};

use crate::command::CommandError;

use self::update::handle_message;

use super::element::{
    controls::ControlsView, install::InstallView, modal::ConfirmModal, version::VersionList,
};

// use self::{
//     element::controls::ControlsView,
//     element::{
//         device::DeviceView, install::InstallView, modal::ConfirmModal, version::VersionList,
//     },
//     firmware::{update::handle_message, view::handle_view},
// };

#[derive(Debug, Clone)]
pub enum Event {
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
}

#[derive(Default)]
pub(crate) struct Firmware {
    filter: Filter,
    controls: ControlsView,
    releases: Option<Vec<Release>>,
    versions: VersionList,
    installer: InstallView,
    installing: bool,
    post_install: bool,
    confirm_modal: ConfirmModal,
    selected_version: Option<Release>,
    installable_asset: Option<PathBuf>,
}

impl<Message, Renderer> Component<Message, Renderer> for Firmware {
    type Event = Event;

    fn update(&mut self, event: Event) -> Option<Event> {
        handle_message(event);
    }

    fn view(&self) -> Element<Self::Event> {
        // self.handle_view()
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
