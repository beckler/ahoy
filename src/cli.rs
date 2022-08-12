use std::path::PathBuf;

use clap::{Parser, Subcommand};

/// Update the firmware for Pirate MIDI devices
/// * Run with no commands to start the GUI *
#[derive(Default, Parser, Debug)]
#[clap(author, version, about, long_about = None, verbatim_doc_comment)]
pub struct Args {
    /// Verbose mode (-v, -vv, -vvv, etc.)
    #[clap(global = true, short, long, parse(from_occurrences))]
    pub verbose: usize,

    /// Graphical debug mode
    #[clap(global = true, short, long)]
    pub debug: bool,

    /// Source
    #[clap(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Parser, Debug)]
pub struct InstallArgs {
    /// Path to the binary file to install to the device
    pub file: PathBuf,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    // /// List all available releases
    // List,
    /// Install a specific binary/firmware file [bypasses GUI]
    Install(InstallArgs),
    /// Update this application to the latest available version
    Update,
}
