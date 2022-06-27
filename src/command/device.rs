use serialport::SerialPortInfo;

use crate::usb::serial::{commands::Command, models::DeviceDetails, PirateMIDISerialDevice};

use super::CommandError;

pub fn get_device_details(port: SerialPortInfo) -> Result<DeviceDetails, CommandError> {
    match PirateMIDISerialDevice::send(&port, Command::Check) {
        Ok(result) => match serde_json::from_str::<DeviceDetails>(result.as_str()) {
            Ok(mut details) => {
                // set manufacturer because we don't get it from the serial port
                details.manufacturer = match port.port_type {
                    serialport::SerialPortType::UsbPort(usb) => {
                        usb.manufacturer.unwrap_or_default()
                    }
                    _ => String::new(),
                };

                Ok(details)
            }
            Err(err) => Err(CommandError::JSONError(err.to_string())),
        },
        Err(err) => Err(CommandError::DeviceError(err.to_string())),
    }
}
