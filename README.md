# Ahoy

Ahoy is an easy-to-use cross-platform executable designed to update the firmware for [Pirate MIDI](https://www.piratemidi.com) Bridge devices.
 
It's written in [Rust](https://www.rust-lang.org), and offers a CLI and GUI. For the GUI, it uses the [iced-rs](https://github.com/iced-rs/iced) framework.

## Status

**THIS SOFTWARE SHOULD BE CONSIDERED ALPHA, AND AS SUCH IS POSSIBLY UNSTABLE.**
 
### Device Recovery 

Luckily if you're concerned, or believe you may have bricked your device, there is a path to reapply the update! The creators of the Bridge devices had great foresight for this exact issue, and you should do the following:

1. **DON'T PANIC**
2. Hold FS6 while plugging in a USB cable.
    1. For the Bridge4, hold FS4 (**Note**: _I have not personally verified this, as I have the Bridge6, please submit an issue or PR if you're able to confirm_) 
3. Wait about 10-15 seconds, as the device won't appear to do anything.
4. As a backup method, use the `dfu-util` command as [laid out here](https://learn.piratemidi.com/software/downloads) (click "Details & Instructions").

## Getting Started

### Pre-built Binaries

Download the [latest pre-built release](https://github.com/beckler/ahoy/releases/latest) for your machine.

All pre-built executables listed are 64-bit, unless otherwise listed. If you need additional archtecture or 32-bit support, I recommend [building locally](#building-locally).

Use this guide for your specific machine:
- Windows
  - Intel/AMD
    -  [***untested**] `x86_64-pc-windows-msvc`
  - ARM
    - [***untested**] `aarch64-pc-windows-msvc`
- macOS
  - Apple Silicon 
    - `aarch64-apple-darwin`
  - Intel
    - [***untested**] `x86_64-apple-darwin`
- Linux
  - Intel/AMD
    - [***untested**] `x86_64-unknown-linux-gnu`

_If you're able to verify these work on the specificed OS/Arch, please let me know via an Issue or PR, as I'm not readily able to verify them myself!_

### macOS Caviet

This software is not notorized or signed, so it will be likely throw a trust error when running.
If this happens, right click the executable, and then explicitly click "Open".

You can [read more about this here](https://support.apple.com/en-us/HT202491).

### Building Locally

- Open your preferred terminal/console/shell
- Install [Rustup](https://rustup.rs/).
- Clone this repo: `git clone https://github.com/beckler/ahoy.git` (if you don't have git, you can [download an archive](https://github.com/beckler/ahoy/archive/refs/heads/main.zip))
- Navigate to where you cloned/downloaded this repo and run the command: `cargo run`
- (**Possibly Required**) If you're having issues with a missing library, this does require `libusb` and any additional requirements that library requires. You can download and install it [from here](https://libusb.info/), or via a package manager:
  - (macOS): `brew install libusb`
  - (linux): `apt install pkg-config libudev-dev libusb-1.0-0-dev`

