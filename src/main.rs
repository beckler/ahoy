use std::{process::exit, time::Duration};

use crate::{
    cli::{Args, Commands},
    command::{enter_bootloader, install_binary},
};
use clap::Parser;
use log::{error, info};

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
            // Commands::List => todo!(),
            Commands::Install(args) => tokio::runtime::Builder::new_current_thread()
                .enable_all()
                .build()
                .unwrap()
                .block_on(async {
                    // get file size
                    let file_size = match args.file.metadata() {
                        Ok(meta) => meta.len(),
                        Err(err) => {
                            error!("unable to retrieve file size: {}", err);
                            std::process::exit(0x0200);
                        }
                    };
                    info!("binary size: {}", file_size);

                    // enter bootloader
                    println!("entering bootloader mode...");
                    match enter_bootloader().await {
                        Ok(_) => (), // continue
                        Err(err) => {
                            error!("device unable to enter bootloader mode: {}", err);
                            std::process::exit(0x0300);
                        }
                    };

                    // sleep to wait for bootloader mode
                    println!("pausing thread for 3 seconds to wait for bootloader mode...");
                    std::thread::sleep(Duration::from_secs(3));

                    // attempt install
                    println!("installing...");

                    // create progress bar
                    let bar = indicatif::ProgressBar::new(file_size as u64);
                    bar.set_style(
                        indicatif::ProgressStyle::default_bar()
                            .template(
                                "{spinner:.green} [{elapsed_precise}] [{bar:27.cyan/blue}] \
                            {bytes}/{total_bytes} ({bytes_per_sec}) ({eta}) {msg:10}",
                            )
                            .unwrap()
                            .progress_chars("#>-"),
                    );

                    let install_result = install_binary(
                        args.file,
                        Some({
                            let bar = bar.clone();
                            move |count| {
                                bar.inc(count as u64);
                            }
                        }),
                    )
                    .await;

                    // handle results
                    match install_result {
                        Ok(_) => (),
                        Err(err) => {
                            error!("unable to install: {:?}", err);
                            std::process::exit(0x0400);
                        }
                    }

                    // finish progress bar
                    bar.finish();
                }),
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
