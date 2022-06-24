use clap::{Parser, Subcommand};

/// Update the firmware for Pirate MIDI devices
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct Args {
    /// Verbose mode (-v, -vv, -vvv, etc.)
    #[clap(global = true, short, long, parse(from_occurrences))]
    pub verbose: usize,

    /// Source
    #[clap(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// List all available releases
    List,
    /// Install the latest or specific release
    Install,
}
