# Screenland

This is a program for creating and editing screenshots, as well as video recording for Linux (Wayland) (support for Linux (X11), Windows, and Mac is planned for the future).

# Installation

The program is written in Rust, and until version 1 is released, you will need to compile it yourself to install it.

## Preparation

To compile the program, install the Rust compiler. This can be done with the following command:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

or follow the instructions on the official website https://rust-lang.org/ru/tools/install/.

## Cloning, building, and installing

Execute:

```bash
cd ~/Downloads
git clone https://github.com/Andrewkoro105/screenland.git
cd screenland
cargo install --path .
```

## Key bindings

The program will be installed in `$HOME/.cargo/bin/`, which should be taken into account when configuring keyboard shortcuts.

# Basic features:

- [X]  Screenshot of an area
- [ ]  Area video recording
- [ ]  Magnifying glass
- [X]  Manual selection of an area with the ability to change it before clicking the save button
- [ ]  Configuration GUI
- [ ]  Ability to import configurations via a link

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

- [X]  Line

  - [X]  Straight
  - [X]  Polyline
  - [ ]  Curve
  - [X]  Arrowhead type
  - [ ]  Line type
- [X]  Rectangle (with a choice of line and fill type)
- [X]  Circle (with a choice of line type and fill)
- [X]  Blur
- [ ]  Text (LaTeX)
- [ ]  Numbering
- [ ]  Image
- [ ]  Additional objects added via configuration files
  
  All new objects have a name and a priority parameter (placing them either in the main menu or in the submenu)

Color and size selection are global settings!

### 3 What to do with the screenshot

- [X]  Save
- [X]  Copy
- [ ]  Pin
- [ ]  Auto-upload and get a shareable link, services can be added via settings
- [ ]  Save and pass file control to a script

# Settings

## CLI

```
Screenland is a program for creating and editing screenshots

Usage: screenland [OPTIONS]

Options:
  -g, --generate-config              Generate config
  -f, --format <FORMAT>              File name format. To add the date and time, use https://docs.rs/chrono/latest/chrono/format/strftime/index.html
  -p, --path <PATH>                  Path to the folder where screenshots will be saved
  -e, --end <END>                    Complete the screenshot immediately after selection (s | save | Save; c | copy | Copy)
  -c, --color-format <COLOR_FORMAT>  The placement of the color channels in the screenshot (rgba -> 0123; bgra -> 2103)
      --config <CONFIG>              Path to config. By default: `~/.config/screenland`
      --disables-overlay             Disable overlay mode and force the screen capture application to open a full-screen window for each monitor
  -o, --output-shader                Displays the shader with the current settings. Best used in conjunction with `bat`, for example: `-o | bat -l wgsl`
      --output-shader-and-run        Displays the shader with the current settings and then runs screenland
      --input-log                    Enable input logging
  -h, --help                         Print help
```

## Config

All settings are located in the `$XDG_CONFIG_HOME/screenland` folder; this is usually `~/.config/screenland`, though this path can also be overridden via the command line.
A standard configuration file can be created using `screenland -g`.
The file structure in this directory is as follows:

```
screenland
- config.yaml
- custom_objects
  - "name_object"
    - object.yaml
  - "name_object2"
    - object.yaml
  - "name_object3"
    - object.yaml
  ...
```

`config.yaml` is the main configuration file; it should have the following structure:

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
# Disable overlay mode and force the screen capture application to open a full-screen window for each monitor
disables_overlay: <BOOL>
# Default color and size settings for all objects
edit_object_base_settings:
  color:
    r: <F32>
    g: <F32>
    b: <F32>
    a: <F32>
  size: <F32>
```

`object.yaml` files describe objects that can be added to a screenshot. The WGSL language is used to describe objects, and YAML is used to specify object settings.
The structure of the `object.yaml` file is shown below (a description of the `<ICON>` type will follow):

```yaml
# Object name; it must be unique among the added objects
name: <STRING>
# Button icon. You can specify an icon using the `iced_font_awesome` library via `Name` and `SolidName`, or specify a direct path to the image using `Path`
icon: 
params:
# Parameters that the user can set and that will be passed to the shader
- 
  # Parameter name
  name: <STRING>
  # Parameter type
  shader_type: 
    F32:
      # Default value
      num_input: <F32>
    # or
    U32: 
      # Default value
      num_input: <U32>
    # or
    I32: 
      # Default value
      num_input: <I32>
    Enum:
      enums:
        - - variant_name
          - <ICON>
# List of functions for this object. This field accepts a `HashMap` whose keys are function names (the function name is prefixed with the object name and “_”), and whose values are the function's arguments and body.
functions: <MAP<STRING, WGSL_CODE>>
# The body of the shader function. The following parameters will be passed to the function: `pixel_color: vec4<f32>, pixel_pos: vec2<f32>, data: Data`. The `data` variable contains all parameters requested from the user, as well as `edit_object_base_settings` and `points_format(cube; bezier_points)`. To view the entire shader, use `-o` or, better yet, `-o | bat -l wgsl`
shader: <WGSL_CODE>
# Point format for modifying an object (`Cube` - rectangular area; BezierPoints - control points for the Bézier curve (not yet finalized and currently linear))
points_format: (Cube; BezierPoints)
```

In the `<ICON>` tag, you can specify an icon using the `iced_font_awesome` library via the `Name` and `SolidName` parameters, or specify a direct path to the image using the `Path` parameter.
Below is the format for how these options should be specified in YAML.

```yaml
Name: <STRING>
# or
SolidName: <STRING>
# or 
Path: <PATH>
```

To view the available icons from `iced_font_awesome`, you can use this script.

```bash
(
    cd ~/Downloads || exit 1
    git clone https://github.com/danielmbomfim/iced_font_awesome.git
    cd iced_font_awesome
    cargo run --example explorer
)
```
