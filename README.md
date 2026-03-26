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

## Bind

The program will be installed in `$HOME/.cargo/bin/`, which should be taken into account when configuring keyboard shortcuts.

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
# Path to config. Usually `~/.config/screenland/config.yaml`
config_path: <PATH>
# The placement of the color channels in the screenshot
color_format:
  r: <USIZE(0..3)>
  g: <USIZE(0..3)>
  b: <USIZE(0..3)>
  a: <USIZE(0..3)>
# Path to the folder where screenshots will be saved
path: <PATH>
# File name format. To add the date and time, use https://docs.rs/chrono/latest/chrono/format/strftime/index.html
format: <CHRONO_FORMAT>
# Complete the screenshot immediately after selection
base_end: (null; Save; Copy)
# Disables overlay mode
disables_overlay: <BOOL>
# Default color and size settings for all objects
edit_object_base_settings:
  color:
    r: <F32>
    g: <F32>
    b: <F32>
    a: <F32>
  size: <F32>
# Custom objects that can either replace the entire screenshot or add something to it
custom_objects:
- 
  # Object name; it must be unique among the added objects
  name: <STRING>
  # Button icon. You can specify an icon using the `iced_font_awesome` library via `Name` and `SolidName`, or specify a direct path to the image using `Path`
  icon: !Name <STRING>
  params:
  # Parameters that the user can set and that will be passed to the shader
  - 
    # Parameter name
    name: <STRING>
    # Parameter type
    shader_type: (!F32)
      # Default value
      num_input: 1.0
  # The body of the shader function. The following parameters will be passed to the function: `pixel_color: vec4<f32>, pixel_pos: vec2<f32>, data: Data`. The `data` variable contains all parameters requested from the user, as well as `edit_object_base_settings` and `points_format(cube)`. To view the entire shader, use `-o` or, better yet, `-o | bat -l wgsl`
  shader: <WGSL_CODE>
  # Point format for modifying an object (`Cube` - rectangular area)
  points_format: (Cube)

```

A standard configuration file can be created using `screenland -g`.

## CLI

```
Screenland is a program for creating and editing screenshots

Usage: screenland [OPTIONS]

Options:
  -s, --support-config <SUPPORT_CONFIG>
          Generate config for the supported system (hypr | hyprland)
  -c, --color-format <COLOR_FORMAT>
          The placement of the color channels in the screenshot (rgba -> 0123; bgra -> 2103)
  -g, --generate-config
          Generate config
  -o, --output-shader
          Displays the shader with the current settings. Best used in conjunction with `bat`, for example: `-o | bat -l wgsl`
      --output-shader-and-run
          Displays the shader with the current settings and run screenland
      --config <CONFIG>
          Path to config. By default: `~/.config/screenland/config.yaml`
      --format <FORMAT>
          File name format. To add the date and time, use https://docs.rs/chrono/latest/chrono/format/strftime/index.html
      --path <PATH>
          Path to the folder where screenshots will be saved
  -e, --end <END>
          Complete the screenshot immediately after selection (s | save | Save; c | copy | Copy)
      --disables-overlay
          Disables overlay mode
  -h, --help
          Print help
```

# Basic features:

- [X]  Screenshot of an area
- [ ]  Recording of an area
- [ ]  Magnifying glass
- [X]  Manual selection of an area with the ability to change it before clicking the save button
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
