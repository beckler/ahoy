use std::env::temp_dir;
use std::fs::File;
use std::io::{copy, Cursor};
use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};
use std::{error::Error, fmt::Display};

use log::*;

use octocrab::models::repos::{Asset, Release};
use serialport::SerialPortInfo;

use crate::{GITHUB_ORG, GITHUB_REPO};

use crate::usb::{
    observer::UsbDevice,
    serial::{
        commands::{Command, ControlArgs},
        models::DeviceDetails,
        PirateMIDISerialDevice,
    },
};

/// maintains connection info for the usb device
#[derive(Clone)]
pub struct UsbConnection {
    pub port: SerialPortInfo,
    pub device: UsbDevice,
    pub details: DeviceDetails,
}

impl UsbConnection {
    pub fn new(port: SerialPortInfo, device: UsbDevice, details: DeviceDetails) -> UsbConnection {
        UsbConnection {
            port,
            device,
            details,
        }
    }

    pub fn install(&self) -> Result<(), CommandError> {
        Ok(())
    }
}

pub async fn enter_bootloader(conn: UsbConnection, path: PathBuf) -> Result<PathBuf, CommandError> {
    //-> Result<DeviceDetails, CommandError>
    match PirateMIDISerialDevice::send(&conn.port, Command::Control(ControlArgs::EnterBootloader)) {
        Ok(result) => {
            info!("RESULT FROM BOOTLOADER: {}", result)
        }
        Err(err) => error!("BOOTLOADER ERROR: {}", err),
    }
    Ok(path) // literally just pass it to the next message
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
                // create temp filepath
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

#[derive(Debug, Clone)]
pub enum CommandError {
    IOError(String),
    JSONError(String),
    DeviceError(String),
    RetievalError(String),
}

impl Error for CommandError {}

impl Display for CommandError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
