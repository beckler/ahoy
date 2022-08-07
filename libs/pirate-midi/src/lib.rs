use std::{fmt::Display, io::ErrorKind, time::Duration};

pub use models::*;
// use models::Response;
use anyhow::anyhow;
use serialport::{available_ports, SerialPortBuilder, SerialPortType};

pub mod models;

pub const VENDOR_ID: u16 = 0x0483;
pub const PRODUCT_ID: u16 = 0x5740;
pub const USB_TIMEOUT: Duration = Duration::from_secs(1);
pub const USB_BAUD_RATE: u32 = 9600;

/// Arguments for Data Transmit Requests (DTXR)
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum DataTransmitRequestArgs {
    ProfileID(String),
    GlobalSettings,
    BankSettings(u8),
}

impl Display for DataTransmitRequestArgs {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let output = match self {
            DataTransmitRequestArgs::ProfileID(s) => format!("profileId,{s}"),
            DataTransmitRequestArgs::GlobalSettings => "globalSettings".to_string(),
            DataTransmitRequestArgs::BankSettings(x) => format!("bankSettings,{x}"),
        };
        write!(f, "{output}")
    }
}

// Arguments for Data Requests (DREQ)
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum DataRequestArgs {
    GlobalSettings,
    BankSettings(i8),
}

impl Display for DataRequestArgs {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let output = match self {
            DataRequestArgs::GlobalSettings => "globalSettings".to_string(),
            DataRequestArgs::BankSettings(x) => format!("bankSettings,{x}"),
        };
        write!(f, "{output}")
    }
}

// Arguments for Control Requests (CTRL)
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum ControlArgs {
    BankUp,
    BankDown,
    GoToBank(i8),
    ToggleFootswitch(i8),
    DeviceRestart,
    EnterBootloader,
    FactoryReset,
}

impl Display for ControlArgs {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let output = match self {
            ControlArgs::BankUp => "bankUp".to_string(),
            ControlArgs::BankDown => "bankDown".to_string(),
            ControlArgs::GoToBank(x) => format!("goToBank,{x}"),
            ControlArgs::ToggleFootswitch(x) => format!("toggleFootswitch,{x}"),
            ControlArgs::DeviceRestart => "deviceRestart".to_string(),
            ControlArgs::EnterBootloader => "enterBootloader".to_string(),
            ControlArgs::FactoryReset => "factoryReset".to_string(),
        };
        write!(f, "{output}")
    }
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum Command {
    /// Check Command (CHCK)
    /// This queries the device about what type of device it is, what firmware version is it running, and other essential information.
    Check,
    /// Control Command (CTRL)
    /// Provides control over basic device functions such as footswitch behaviour, bank navigation, and reset modes.
    Control(ControlArgs),
    /// Data Request Command (DREQ)
    /// Requested by the host application to the device. This prompts the device for global, bank, or configuration data.
    DataRequest(DataRequestArgs),
    /// Data Transmit Request (DTXR)
    /// Informs the device that the host appliccation wishes to transmit new data.
    DataTransmitRequest(DataTransmitRequestArgs),
    /// Reset Command (RSET)
    /// Resets the communication state of the device to exit a command state if required.
    Reset,
}

impl Command {
    pub fn format(&self) -> Vec<String> {
        match self {
            Command::Check | Command::Reset => vec![format!("{self}")],
            Command::Control(args) => match args {
                _ => vec![format!("{self}"), format!("{args}")],
            },
            Command::DataRequest(args) => match args {
                _ => vec![format!("{self}"), format!("{args}")],
            },
            Command::DataTransmitRequest(args) => match args {
                _ => vec![format!("{self}"), format!("{args}")],
            },
        }
    }
}

impl Display for Command {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let output = match self {
            Command::Check => "CHCK",
            Command::Control(_) => "CTRL",
            Command::DataRequest(_) => "DREQ",
            Command::DataTransmitRequest(_) => "DTXR",
            Command::Reset => "RSET",
        };
        write!(f, "{output}")
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PirateMIDIDevice {
    vid: u16,
    pid: u16,
    timeout: Duration,
    baud_rate: u32,
}

impl Default for PirateMIDIDevice {
    fn default() -> Self {
        Self {
            vid: VENDOR_ID,
            pid: PRODUCT_ID,
            timeout: USB_TIMEOUT,
            baud_rate: USB_BAUD_RATE,
        }
    }
}

impl PirateMIDIDevice {
    /// Creates a new Pirate MIDI serial client
    pub fn new() -> PirateMIDIDevice {
        Self::default()
    }

    /// By default we attempt to connect with devices with the constant VENDOR_ID
    /// However, this method exists for if this changes in the future.
    pub fn with_vendor_id(&self, vid: u16) -> PirateMIDIDevice {
        Self {
            vid: vid,
            pid: self.pid,
            timeout: self.timeout,
            baud_rate: self.baud_rate,
        }
    }

    /// By default we attempt to connect with devices with the constant PRODUCT_ID
    /// However, this method exists for if this changes in the future.
    pub fn with_product_id(&self, pid: u16) -> PirateMIDIDevice {
        Self {
            vid: self.vid,
            pid: pid,
            timeout: self.timeout,
            baud_rate: self.baud_rate,
        }
    }

    /// By default we attempt to connect with devices with the constant BAUD_RATE
    /// However, this method exists for if this changes in the future.
    pub fn with_baud_rate(&self, baud_rate: u32) -> PirateMIDIDevice {
        Self {
            vid: self.vid,
            pid: self.pid,
            timeout: self.timeout,
            baud_rate: baud_rate,
        }
    }

    /// By default we attempt to connect with devices with the constant BAUD_RATE
    /// However, this method exists for if this changes in the future.
    pub fn with_timeout(&self, timeout: Duration) -> PirateMIDIDevice {
        Self {
            vid: self.vid,
            pid: self.pid,
            timeout: timeout,
            baud_rate: self.baud_rate,
        }
    }

    pub fn send(&self, command: Command) -> Result<Response, Error> {
        match self.find_device() {
            Ok(device) => match device.open() {
                Ok(mut port) => {
                    // setting up output
                    let mut buffer = String::new();
                    let mut err: Option<crate::Error> = None;

                    // turn our commands into a series of commands
                    for (i, sub_cmd) in command.format().iter().enumerate() {
                        // clear buffer before we iterate
                        if buffer.len() > 0 {
                            let _ = &buffer.clear();
                        }

                        // transmit command
                        match &port.write(format!("{i},{sub_cmd}~").as_bytes()) {
                            Ok(_) => (),
                            Err(ref e) if e.kind() == ErrorKind::TimedOut => (),
                            Err(e) => eprintln!("{:?}", e),
                        }

                        match port.read_to_string(&mut buffer) {
                            Ok(_) => (),
                            Err(e) if e.kind() == ErrorKind::TimedOut => (),
                            Err(e) if e.kind() == ErrorKind::BrokenPipe => {
                                err = match &command {
                                    Command::Control(sub) => match sub {
                                        // these commands will break the pipe on purpose so don't log it as an error
                                        ControlArgs::DeviceRestart
                                        | ControlArgs::EnterBootloader
                                        | ControlArgs::FactoryReset => None,
                                        _ => Some(Error::BrokenPipeError(e)),
                                    },
                                    _ => Some(Error::BrokenPipeError(e)),
                                };
                            }
                            Err(e) => err = Some(Error::ReadError(e)),
                        };
                    }

                    // if we have a broken pipe error, report it here.
                    match err {
                        Some(inner) => Err(inner),
                        None => {
                            // match our response to our request
                            let result = match command {
                                Command::Check => {
                                    match serde_json::from_str::<check::CheckResponse>(
                                        &trim_response(&buffer),
                                    ) {
                                        Ok(result) => Response::Check(result),
                                        Err(err) => return Err(Error::JsonError(err)),
                                    }
                                }
                                Command::Control(subreq) => match subreq {
                                    _ => {
                                        let result = if trim_response(&buffer) == "ok" {
                                            Ok(())
                                        } else {
                                            Err(anyhow!(trim_response(&buffer)))
                                        };
                                        Response::Control(result)
                                    }
                                },
                                Command::DataRequest(subreq) => match subreq {
                                    DataRequestArgs::GlobalSettings => Response::DataRequest(
                                        DataRequestResponse::GlobalSettings(trim_response(&buffer)),
                                    ),
                                    DataRequestArgs::BankSettings(_) => {
                                        match serde_json::from_str::<bank::BankSettings>(
                                            &trim_response(&buffer),
                                        ) {
                                            Ok(result) => Response::DataRequest(
                                                DataRequestResponse::BankSettings(result),
                                            ),
                                            Err(err) => return Err(Error::JsonError(err)),
                                        }
                                    } // Response::DataRequest(trim_response(&buffer))
                                },
                                Command::DataTransmitRequest(_) => {
                                    Response::DataTransmit(trim_response(&buffer))
                                }
                                Command::Reset => Response::Reset(trim_response(&buffer)),
                            };
                            Ok(result)
                        }
                    }
                }
                Err(err) => Err(Error::SerialError(err.to_string())),
            },
            Err(err) => Err(err),
        }
    }

    fn find_device(&self) -> Result<SerialPortBuilder, Error> {
        match available_ports() {
            Ok(ports) => {
                for p in ports {
                    if let SerialPortType::UsbPort(usb_info) = p.port_type {
                        if usb_info.vid == self.vid && usb_info.pid == self.pid {
                            return Ok(
                                serialport::new(p.port_name, self.baud_rate).timeout(self.timeout)
                            );
                        }
                    }
                }
                Err(Error::SerialError("unable to locate device".to_string()))
            }
            Err(err) => Err(Error::SerialError(err.to_string())),
        }
    }
}

fn trim_response(response: &String) -> String {
    response
        .trim_start_matches(char::is_numeric)
        .trim_start_matches(',')
        .trim_end_matches('~')
        .to_string()
}

#[derive(Debug)]
pub enum Error {
    ReadError(std::io::Error),
    JsonError(serde_json::Error),
    SerialError(String),
    InterfaceError(String),
    BrokenPipeError(std::io::Error),
}

impl std::error::Error for Error {}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
