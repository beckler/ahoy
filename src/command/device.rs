use std::{
    io::{self, Seek},
    path::PathBuf,
};

use dfu_libusb::DfuLibusb;
use pirate_midi_rs::{check::CheckResponse, Command, ControlArgs, PirateMIDIDevice};

use crate::{usb::observer::UsbDevice, USB_PRODUCT_DFU_ID, USB_VENDOR_ID};

use super::CommandError;

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
    let context = rusb::Context::new()
        .map_err(|e| CommandError::Device(format!("unable to create usb context: {}", e)))?;
    // open the binary file and get the file size
    let mut file = std::fs::File::open(binary_path)
        .map_err(|e| CommandError::IO(format!("could not open firmware file: {}", e)))?;
    let file_size = u32::try_from(
        file.seek(io::SeekFrom::End(0))
            .map_err(|e| CommandError::IO(e.to_string()))?,
    )
    .map_err(|e| CommandError::IO(format!("the firmware file is too big: {}", e)))?;
    file.seek(io::SeekFrom::Start(0))
        .map_err(|e| CommandError::IO(e.to_string()))?;

    // open the DFU interface
    let mut dfu_iface = DfuLibusb::open(&context, USB_VENDOR_ID, USB_PRODUCT_DFU_ID, 0, 0)
        .map_err(|e| CommandError::Dfu(e.to_string()))?;

    // setup our progress bar - if available
    if progress.is_some() {
        dfu_iface.with_progress(progress.unwrap());
    }

    // PERFORM THE INSTALL
    dfu_iface
        .download(file, file_size)
        .map_err(|e| CommandError::Dfu(e.to_string()))
}

pub async fn enter_bootloader() -> Result<(), CommandError> {
    match PirateMIDIDevice::new().send(Command::Control(ControlArgs::EnterBootloader)) {
        Ok(_) => Ok(()),
        Err(err) => Err(CommandError::Device(format!(
            "UNABLE TO ENTER BOOTLOADER: {}",
            err
        ))),
    }
}
