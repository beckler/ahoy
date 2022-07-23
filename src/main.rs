use std::{process::exit, time::Duration};

use crate::cli::{Args, Commands};
use clap::Parser;
use log::info;

mod cli;
mod command;
mod gui;
mod usb;

// GLOBALS
const USB_VENDOR_ID: u16 = 0x0483;
const USB_PRODUCT_ID: u16 = 0x5740;
const USB_PRODUCT_DFU_ID: u16 = 0xDF11;
const USB_TIMEOUT: Duration = Duration::from_secs(1);
const GITHUB_ORG: &str = "Pirate-MIDI";
const GITHUB_REPO: &str = "Pirate-MIDI-Features-Bug-Tracking";

fn main() {
    // parse the arguments
    let args = Args::parse();

    // configure logging for libusb
    match args.verbose {
        0 => rusb::set_log_level(rusb::LogLevel::None),
        1 => rusb::set_log_level(rusb::LogLevel::Error),
        2 => rusb::set_log_level(rusb::LogLevel::Warning),
        3 => rusb::set_log_level(rusb::LogLevel::Info),
        _ => rusb::set_log_level(rusb::LogLevel::Debug),
    }

    // configure std logging
    stderrlog::new()
        .module(module_path!())
        .verbosity(args.verbose)
        .timestamp(stderrlog::Timestamp::Second)
        .init()
        .unwrap();

    info!("ahoy mateys - starting up...");

    // execute!
    match args.command {
        Some(cmd) => match cmd {
            Commands::List => todo!(),
            Commands::Install => todo!(),
        },
        None => {
            // Start the GUI
            match gui::run(args) {
                Ok(_) => exit(0),
                Err(e) => println!("{:?}", e),
            }
        }
    }
}
