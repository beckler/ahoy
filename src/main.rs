use std::{process::exit, time::Duration};

use crate::cli::{Args, Commands};
use clap::Parser;
use log::info;

mod cli;
mod command;
mod gui;
mod usb;

// GLOBALS
const USB_BAUD_RATE: u32 = 9600;
const USB_TIMEOUT: Duration = Duration::from_secs(1);
const USB_MANUFACTURER: &str = "Pirate MIDI";
const GITHUB_ORG: &str = "Pirate-MIDI";
const GITHUB_REPO: &str = "Pirate-MIDI-Features-Bug-Tracking";

fn main() {
    // parse the arguments
    let args = Args::parse();

    // configure logging
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
