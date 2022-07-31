use dfu_libusb::DfuLibusb;
use log::*;
use pirate_midi::check::CheckResponse;
use std::env::temp_dir;
use std::fs::File;
use std::io::{self, copy, Cursor, Seek};
use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};

use octocrab::models::repos::{Asset, Release};

use crate::{GITHUB_ORG, GITHUB_REPO, USB_PRODUCT_DFU_ID, USB_VENDOR_ID};

use crate::usb::observer::UsbDevice;
use pirate_midi::{Command, ControlArgs, PirateMIDIDevice};

/// maintains connection info for the usb device
#[derive(Clone)]
pub struct UsbConnection {
    pub device: UsbDevice,
    pub details: CheckResponse,
}

impl UsbConnection {
    pub fn new(device: UsbDevice, details: CheckResponse) -> UsbConnection {
        UsbConnection { device, details }
    }
}

pub async fn install_binary(
    binary_path: PathBuf,
    progress: Option<impl FnMut(usize) + 'static>,
) -> Result<(), CommandError> {
    // create new usb context
    let context = rusb::Context::new().map_err(|e| {
        CommandError::DeviceError(format!("unable to create usb context: {}", e.to_string()))
    })?;
    // open the binary file and get the file size
    let mut file = std::fs::File::open(binary_path).map_err(|e| {
        CommandError::IOError(format!("could not open firmware file: {}", e.to_string()))
    })?;
    let file_size = u32::try_from(
        file.seek(io::SeekFrom::End(0))
            .map_err(|e| CommandError::IOError(e.to_string()))?,
    )
    .map_err(|e| {
        CommandError::IOError(format!("the firmware file is too big: {}", e.to_string()))
    })?;
    file.seek(io::SeekFrom::Start(0))
        .map_err(|e| CommandError::IOError(e.to_string()))?;

    // open the DFU interface
    let mut dfu_iface = DfuLibusb::open(&context, USB_VENDOR_ID, USB_PRODUCT_DFU_ID, 0, 0)
        .map_err(|e| CommandError::DfuError(e.to_string()))?;

    // setup our progress bar - if available
    if progress.is_some() {
        dfu_iface.with_progress(progress.unwrap());
    }

    // PERFORM THE INSTALL
    dfu_iface
        .download(file, file_size)
        .map_err(|e| CommandError::DfuError(e.to_string()))
}

pub async fn enter_bootloader() -> Result<(), CommandError> {
    match PirateMIDIDevice::new().send(Command::Control(ControlArgs::EnterBootloader)) {
        Ok(_) => Ok(()),
        Err(err) => Err(CommandError::DeviceError(format!(
            "UNABLE TO ENTER BOOTLOADER: {}",
            err
        ))),
    }
}

/// retrieve all available github releases
pub async fn fetch_releases() -> Result<Vec<Release>, CommandError> {
    let fetch_all = async {
        info!("fetching releases from github...");
        // create crab instance
        let octocrab = octocrab::instance();
        // grab first page
        let page = octocrab
            .repos(GITHUB_ORG, GITHUB_REPO)
            .releases()
            .list()
            .per_page(50)
            .send()
            .await?;

        trace!("{:?}", page);

        // grab all pages. be warned there is no rate limiting here...
        octocrab.all_pages(page).await
    };

    match fetch_all.await {
        Ok(releases) => Ok(releases),
        Err(err) => Err(CommandError::RetievalError(err.to_string())),
    }
}

pub async fn fetch_asset(asset: Asset) -> Result<PathBuf, CommandError> {
    // download the binary
    info!("fetching asset from github...");
    match reqwest::get(asset.browser_download_url).await {
        Ok(response) => match response.bytes().await {
            Ok(content) => {
                // create timestamp
                let time = SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .unwrap()
                    .as_millis();
                // create temp file
                let temp_file_path = temp_dir().join(format!("{time}-{}", asset.name));
                info!("downloading file to: {}", temp_file_path.display());
                // create temp file
                match File::create(&temp_file_path) {
                    Ok(mut file) => {
                        let mut content = Cursor::new(content);
                        match copy(&mut content, &mut file) {
                            Ok(written) => {
                                info!("successfully downloaded - total bytes written: {}", written);
                                return Ok(temp_file_path);
                            }
                            Err(err) => return Err(CommandError::IOError(err.to_string())),
                        }
                    }
                    Err(err) => return Err(CommandError::IOError(err.to_string())),
                }
            }
            Err(err) => return Err(CommandError::RetievalError(err.to_string())),
        },
        Err(err) => return Err(CommandError::RetievalError(err.to_string())),
    }
}

#[derive(thiserror::Error, Debug, Clone)]
pub enum CommandError {
    #[error("unable to retrieve file")]
    IOError(String),
    #[error("unable to perform install")]
    DfuError(String),
    #[error("unable to send command to device")]
    DeviceError(String),
    #[error("unable to fetch releases")]
    RetievalError(String),
}
