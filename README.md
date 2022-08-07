# Ahoy

Ahoy is an easy-to-use cross-platform executable designed to update the firmware for [Pirate MIDI](https://www.piratemidi.com) Bridge devices.
 
It's written in [Rust](https://www.rust-lang.org), and offers a CLI and GUI. For the GUI, it uses the [iced-rs](https://github.com/iced-rs/iced) framework.

## Status

**THIS SOFTWARE SHOULD BE CONSIDERED ALPHA, AND AS SUCH IS POSSIBLY UNSTABLE.**
 
Luckily if you believe you may have bricked your device, there is a path to reapply the update! The creators of the Bridge devices had great foresight for this exact issue, and you should do the following:

- Hold FS6 while plugging in the USB cable.
  - For the Bridge4, hold FS4 (_Note: I have not personally verified this, as I have the Bridge6_) 
- Wait about 10-15 seconds.
- Use the `dfu-util` command as [laid out here](https://learn.piratemidi.com/software/downloads) (click "Details & Instructions").

## Getting Started

### Pre-built Binaries

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

This software is not notorized or signed, so it will be likely throw a trust error when running.
If this happens, right click the executable, and then explicitly click "Open".

You can [read more about this here](https://support.apple.com/en-us/HT202491).

### Building Locally

- Open your preferred terminal/console/shell
- Install [Rustup](https://rustup.rs/).
- Clone this repo: `git clone https://github.com/beckler/ahoy.git` (if you don't have git, you can [download an archive](https://github.com/beckler/ahoy/archive/refs/heads/main.zip))
- Navigate to where you saved this repo and run the command: `cargo run`

