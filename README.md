# Screenland

This is a program for creating and editing screenshots, as well as recording videos for Linux (Wayland) (support for Linux (X11), Windows, and Mac is planned for the future).

# Installation

The program is written in Rust, and until version 1 is released, you will need to compile it yourself to install it.

## Preparation

To compile the program, install the Rust compiler. This can be done with the following command:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

or you can learn how to do it on their website https://rust-lang.org/ru/tools/install/

## Cloning, building, and installing

Execute:

```bash
cd ~/Downloads
git clone https://github.com/Andrewkoro105/screenland.git
cd screenland
cargo install --path .
```

## Features

### Linux (Wayland)

To run the program, you need to add settings to your compositor's configuration file to open “screenland-{monitor_name}” windows on the corresponding monitors.

For supported compositors, this can be done with a single command

#### hyprland

```bash
screenland -s hyprland >> ~/.config/hypr/hyprland.conf
```

# Settings

## Config

The configuration file is written in YAML format and is located by default at `~/.config/screenland/config.yaml` (it will not be there if the settings have not been changed).

The structure of config.yaml looks like this:

```yaml
# path to config. Usually `~/.config/screenland/config.yaml`
config_path: <PATH>
# path to the folder where screenshots will be saved
path: <PATH>
# file name format. To add the date and time, use https://docs.rs/chrono/latest/chrono/format/strftime/index.html
format: <CHRONO_FORMAT>
# complete the screenshot immediately after selection
base_end: (null | Save | Copy)

```

A standard configuration file can be created using `screenland -g`.

## CLI

```
❯ screenland --help
Screenland is a program for creating and editing screenshots

Usage: screenland [OPTIONS]

Options:
  -s, --support-config <SUPPORT_CONFIG>
          generate config for the supported system (hypr | hyprland)
  -g, --generate-config
          generate config
  -c, --config <CONFIG>
          path to config. Usually `~/.config/screenland/config.yaml`
  -f, --format <FORMAT>
          file name format. To add the date and time, use https://docs.rs/chrono/latest/chrono/format/strftime/index.html
  -p, --path <PATH>
          path to the folder where screenshots will be saved
  -e, --end <END>
          complete the screenshot immediately after selection (s | save | Save; c | copy | Copy)
  -h, --help
          Print help
```

# Basic features:

- [X]  Screenshot of an area
- [ ]  Recording of an area
- [ ]  Magnifying glass
- [ ]  Manual selection of an area with the ability to change it before clicking the save button
- [ ]  Interface for all configurations
- [ ]  Ability to get configurations simply by link

## There are 3 modular systems:

### 1 Auto selection

Generated areas

- [ ]  Rectangle search
- [ ]  Plain text search
- [ ]  Ability to expand via configuration files
  Script that returns an array of regions in cbor/yaml/json format to the output stream

You can either select one area or hold down Shift to combine the desired areas (of course, there is also the classic manual selection).

### 2 Image editing objects

You can add objects to the screenshot that will change it

- [ ]  Line
- [ ]  Straight line (with the ability to curve and change the tip and line type)
- [ ]  Rectangle (with a choice of line and fill type)
- [ ]  Circle (with a choice of line type and fill)
- [ ]  Blur
- [ ]  Text
  Only the settings that are in the library for this will be implemented.
  Settings:

  - [ ]  Outline: Presence, size, color
  - [ ]  Under/overlining
  - [ ]  Font selection
  - [ ]  Background
  - [ ]  Line spacing
  - [ ]  Alignment
- [ ]  Counter
- [ ]  Image
- [ ]  More objects added via config
  All new objects have a name and an importance parameter (placing them either in the main menu or in the additional menu)
  Types:
- [ ]  Picture/video
- [ ]  Shader + shader settings interface configuration

Color and size selection are global settings!

### 3 What to do with the screenshot

- [X]  Save
- [X]  Copy
- [ ]  Pin
- [ ]  Auto-download with link retrieval, services are added via settings
- [ ]  Saving and transferring control of a file to a script
