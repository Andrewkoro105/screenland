pub mod edit_object_base_settings;
use clap::Parser;
use directories::UserDirs;
use iced_helper::ui_elements::num_input::NumInput;
use serde::{Deserialize, Serialize};
use serde_yaml::from_reader;
use std::{
    fs::{self, OpenOptions},
    path::PathBuf,
    str::FromStr,
};

use crate::{
    Args,
    app::{
        edit_object::custom_object::{
            CustomIndexedObjectSettings, CustomObjectSettings, add_type_id,
            add_type_id_deserialize,
            icon::Icon,
            param::{Param, ShaderType},
            points::PointsFormat,
            remove_type_id_serialize,
        },
        end::End,
        settings::edit_object_base_settings::{ColorInput, EditObjectBaseSettings},
    },
    screenshots::ColorFormat,
};

#[derive(Clone, Serialize, Deserialize)]
pub struct Settings {
    config_path: PathBuf,

    #[serde(skip)]
    #[serde(default)]
    pub cli_color_format: bool,
    pub color_format: ColorFormat,
    #[serde(skip)]
    #[serde(default)]
    pub cli_path: bool,
    pub path: PathBuf,
    #[serde(skip)]
    #[serde(default)]
    pub cli_format: bool,
    pub format: String,
    #[serde(skip)]
    #[serde(default)]
    pub cli_base_end: bool,
    pub base_end: Option<End>,

    #[serde(skip)]
    #[serde(default)]
    pub cli_disables_overlay: bool,
    pub disables_overlay: bool,

    #[serde(skip)]
    #[serde(default)]
    pub output_shader_and_run: bool,

    pub edit_object_base_settings: EditObjectBaseSettings,
    #[serde(deserialize_with = "add_type_id_deserialize")]
    #[serde(serialize_with = "remove_type_id_serialize")]
    pub custom_objects: Vec<CustomIndexedObjectSettings>,
}

impl Settings {
    pub fn new(config_path: PathBuf) -> Self {
        Self {
            config_path,
            cli_color_format: false,
            color_format: ColorFormat {
                r: 0,
                g: 1,
                b: 2,
                a: 3,
            },
            path: PathBuf::from("./"),
            cli_path: false,
            format: String::from("screenshot_%Y-%m-%d_%H:%M:%S.png"),
            cli_format: false,
            base_end: None,
            cli_base_end: false,
            output_shader_and_run: false,
            cli_disables_overlay: false,
            disables_overlay: false,
            edit_object_base_settings: EditObjectBaseSettings {
                color: ColorInput {
                    r: NumInput::new(1.),
                    g: NumInput::new(0.),
                    b: NumInput::new(0.),
                    a: NumInput::new(1.),
                    hide_color: false,
                },
                size: NumInput::new(6.),
            },
            custom_objects: add_type_id(vec![CustomObjectSettings::new(
                "rectangle".into(),
                Icon::Name("square".into()),
                vec![
                    Param::new(
                        "filter_r".into(),
                        ShaderType::F32 {
                            num_input: NumInput::new(1.),
                        },
                    ),
                    Param::new(
                        "filter_g".into(),
                        ShaderType::F32 {
                            num_input: NumInput::new(1.),
                        },
                    ),
                    Param::new(
                        "filter_b".into(),
                        ShaderType::F32 {
                            num_input: NumInput::new(1.),
                        },
                    ),
                ],
                r"
    if in(pixel_pos, data.cube.start, data.cube.end) {
        return vec4(pixel_color.r * data.filter_r, pixel_color.g * data.filter_g, pixel_color.b * data.filter_b, pixel_color.a);
    } else {
        return pixel_color;
    }".into(),
                Some(PointsFormat::Cube),
            )]),
        }
    }

    pub fn get_path(path: Option<PathBuf>) -> PathBuf {
        path.unwrap_or(if let Some(user_dirs) = UserDirs::new() {
            user_dirs.home_dir().join(".config/screenland/config.yaml")
        } else {
            PathBuf::from("./config.yaml")
        })
    }

    pub fn load(args: Option<Args>, arg_config: Option<PathBuf>) -> Self {
        let args = args.unwrap_or(Args::parse());
        let arg_config =
            arg_config.unwrap_or(Settings::get_path(args.path.clone().map(Into::into)));

        let mut result = fs::OpenOptions::new()
            .read(true)
            .open(&arg_config)
            .map(|file| {
                let result = from_reader::<_, Settings>(file);
                if let Err(err) = &result {
                    eprintln!("Configuration parsing error:\n{err}");
                }
                result.ok()
            })
            .inspect_err(|err| {
                if let Some(path) = args.config {
                    eprintln!("Unable to open file: {path}. Error: {err}")
                }
            })
            .ok()
            .flatten()
            .unwrap_or_else(|| Settings::new(arg_config));

        if let Some(path) = args.path {
            result.path = PathBuf::from(path);
            result.cli_path = true;
        }

        if let Some(format) = args.format {
            result.format = format;
            result.cli_format = true;
        }

        if let Some(color_format) = args.color_format {
            result.color_format = ColorFormat::from_str(&color_format).unwrap();
            result.cli_format = true;
        }

        if args.disables_overlay {
            result.disables_overlay = true;
            result.cli_disables_overlay = true;
        }

        result.output_shader_and_run = args.output_shader_and_run;

        if let Some(end) = args.end {
            match end.as_str() {
                "s" | "save" | "Save" => {
                    result.base_end = Some(End::Save);
                    result.cli_base_end = true;
                }
                "c" | "copy" | "Copy" => {
                    result.base_end = Some(End::Copy);
                    result.cli_base_end = true;
                }
                _ => eprintln!("{end} unsupported termination method"),
            };
        }

        result
    }

    pub fn save(&self) {
        let mut save_data = self.clone();
        let base_save_data = Self::new(self.config_path.clone());
        if !save_data.cli_color_format {
            save_data.color_format = base_save_data.color_format;
        }
        if !save_data.cli_path {
            save_data.path = base_save_data.path;
        }
        if !save_data.cli_format {
            save_data.format = base_save_data.format;
        }
        if !save_data.cli_base_end {
            save_data.base_end = base_save_data.base_end;
        }
        if !save_data.cli_disables_overlay {
            save_data.disables_overlay = base_save_data.disables_overlay;
        }

        if let Some(parent) = self.config_path.parent()
            && !parent.exists()
        {
            fs::create_dir_all(parent).unwrap();
        }
        serde_yaml::to_writer(
            OpenOptions::new()
                .write(true)
                .create(true)
                .truncate(true)
                .open(&self.config_path)
                .unwrap_or_else(|err| {
                    panic!("Unable to open file: {:?}. Error: {err}", self.config_path)
                }),
            &save_data,
        )
        .unwrap();
    }
}
