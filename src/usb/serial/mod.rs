use std::fmt::Display;
use std::io::ErrorKind;

use serialport::SerialPortInfo;

use crate::{USB_BAUD_RATE, USB_TIMEOUT};

use self::commands::{Command, ControlArgs};

pub mod commands;
pub mod models;

#[derive(Clone, Default)]
pub struct PirateMIDISerialDevice {}

impl PirateMIDISerialDevice {
    /* PUBLIC API */
    pub fn send(port: &SerialPortInfo, command: Command) -> Result<String, SerialError> {
        // have to keep a copy here since we're gonna move into our own thread.
        let cmd = command.clone();

        // setup up serial port
        let serial_port = serialport::new(port.port_name.clone(), USB_BAUD_RATE)
            .timeout(USB_TIMEOUT)
            .open();

        // yolo
        match serial_port {
            Ok(mut port) => {
                // setting up output
                let mut buffer = String::new();
                let mut err: Option<SerialError> = None;

                for (i, sub_cmd) in cmd.format().iter().enumerate() {
                    // clear buffer before we iterate
                    if buffer.len() > 0 {
                        let _ = &buffer.clear();
                    }

                    // transmit command
                    match &port.write(format!("{i},{sub_cmd}~").as_bytes()) {
                        Ok(_written) => {
                            // println!("bytes written: {}", &written);
                        }
                        Err(ref e) if e.kind() == ErrorKind::TimedOut => (),
                        Err(e) => eprintln!("{:?}", e),
                    }

                    match port.read_to_string(&mut buffer) {
                        Ok(_) => (),
                        Err(e) if e.kind() == std::io::ErrorKind::TimedOut => (),
                        Err(e) if e.kind() == std::io::ErrorKind::BrokenPipe => {
                            err = match &cmd {
                                Command::Control(sub) => match sub {
                                    ControlArgs::DeviceRestart => None,
                                    ControlArgs::EnterBootloader => None,
                                    ControlArgs::FactoryReset => None,
                                    _ => Some(SerialError::BrokenPipeError(e)),
                                },
                                _ => Some(SerialError::BrokenPipeError(e)),
                            };
                        }
                        Err(e) => err = Some(SerialError::ReadError(e)),
                    };
                }

                // if we have a broken pipe error, report it here.
                match err {
                    Some(inner) => Err(inner),
                    None => Ok(trim_response(&buffer)),
                }
            }
            Err(e) => Err(SerialError::InterfaceError(format!(
                "serialport error: {:?}",
                e
            ))),
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
pub enum SerialError {
    InterfaceError(String),
    ReadError(std::io::Error),
    BrokenPipeError(std::io::Error),
}

impl std::error::Error for SerialError {}

impl Display for SerialError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
