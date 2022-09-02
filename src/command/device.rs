use std::path::PathBuf;

use dfu_libusb::DfuLibusb;
use log::error;
use pirate_midi_rs::{Command, ControlArgs, PirateMIDIDevice};
use rusb::{Context, Device};

use crate::{USB_PRODUCT_DFU_ID, USB_VENDOR_ID};

use super::CommandError;

pub async fn install_binary(
    binary_path: PathBuf,
    progress: Option<impl FnMut(usize) + 'static>,
    raw_device: Option<Device<Context>>,
) -> Result<(), CommandError> {
    // open the binary file and get the file size
    let file = std::fs::File::open(binary_path)
        .map_err(|e| CommandError::IO(format!("could not open firmware file: {}", e)))?;

    let mut dfu_iface = match raw_device {
        Some(device) => {
            // get device descriptor
            let (vid, pid) = match device.device_descriptor() {
                Ok(desc) => (desc.product_id(), desc.vendor_id()),
                Err(err) => panic!(
                    "unable to get device descriptors from usb device! - error: {}",
                    err
                ),
            };
            // open the DFU interface
            DfuLibusb::open(device.context(), vid, pid, 0, 0)
                .map_err(|e| CommandError::Dfu(e.to_string()))?
        }
        // if we didn't pass in a device, just try to guess via VID and PID
        None => {
            // create new usb context
            let context = rusb::Context::new().map_err(|e| {
                CommandError::Device(format!("unable to create usb context: {}", e))
            })?;
            // open the DFU interface
            DfuLibusb::open(&context, USB_VENDOR_ID, USB_PRODUCT_DFU_ID, 0, 0)
                .map_err(|e| CommandError::Dfu(e.to_string()))?
        }
    };

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
