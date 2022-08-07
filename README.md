# Ahoy

Ahoy is an easy-to-use cross-platform executable designed to update the firmware for [Pirate MIDI](https://www.piratemidi.com) Bridge devices.
 
It's written in [Rust](https://www.rust-lang.org), and offers a CLI and GUI. For the GUI, it uses the [iced-rs](https://github.com/iced-rs/iced) framework.

## Getting Started

<!---
Download the [latest pre-built release](https://github.com/beckler/ahoy/releases/latest) for your machine.

Use this guide for your specific machine:
- Windows 
  - Intel/AMD
    - `x86_64-pc-windows-gnu`
- macOS (**will likely get a security warning**: [read more about this here](https://support.apple.com/en-us/HT202491))
  - Apple Silicon 
    - `aarch64-apple-darwin`
  - Intel
    - `x86_64-apple-darwin`
- Linux
  - Intel/AMD
    - `x86_64-unknown-linux-gnu`
    - `x86_64-unknown-linux-musl`
  - ARM
    - `aarch64-unknown-linux-gnu`
    - `aarch64-unknown-linux-musl`

### macOS Caviet
--->

### Building Locally

1. Open your preferred terminal/console/shell
2. Install [Rustup](https://rustup.rs/).
3. Clone this repo: `git clone https://github.com/beckler/ahoy.git` (if you don't have git, you can [download an archive](https://github.com/beckler/ahoy/archive/refs/heads/main.zip))
4. Navigate to where you saved this repo and run the command: `cargo run`
