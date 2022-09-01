use std::path::PathBuf;

use dfu_libusb::DfuLibusb;
use log::error;
use pirate_midi_rs::{Command, ControlArgs, PirateMIDIDevice};

use crate::{USB_PRODUCT_DFU_ID, USB_VENDOR_ID};

use super::CommandError;

pub async fn install_binary(
    // device: Device<Context>,
    binary_path: PathBuf,
    progress: Option<impl FnMut(usize) + 'static>,
) -> Result<(), CommandError> {
    // create new usb context
    let context = rusb::Context::new()
        .map_err(|e| CommandError::Device(format!("unable to create usb context: {}", e)))?;
    // open the binary file and get the file size
    let file = std::fs::File::open(binary_path)
        .map_err(|e| CommandError::IO(format!("could not open firmware file: {}", e)))?;

    // open the DFU interface
    let mut dfu_iface = DfuLibusb::open(&context, USB_VENDOR_ID, USB_PRODUCT_DFU_ID, 0, 0)
        .map_err(|e| CommandError::Dfu(e.to_string()))?;

    // setup our progress bar - if available
    if progress.is_some() {
        dfu_iface.with_progress(progress.unwrap());
    }

    // PERFORM THE INSTALL
    match dfu_iface.download_all(file) {
        Ok(_) => Ok(()),
        Err(dfu_libusb::Error::LibUsb(rusb::Error::Io)) => Ok(()),
        Err(err) => {
            error!("dfu download error: {}", err);
            Err(CommandError::Dfu(err.to_string()))
        }
    }
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
