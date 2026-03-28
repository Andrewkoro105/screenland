pub mod edit_object_base_settings;
use clap::Parser;
use iced_helper::ui_elements::num_input::NumInput;
use serde::{Deserialize, Serialize};
use serde_yaml::from_reader;
use std::{
    fs::{self, OpenOptions},
    path::PathBuf,
    str::FromStr,
};
use xdg::BaseDirectories;

use crate::{
    Args,
    app::{
        edit_object::custom_object::{
            icon::Icon,
            param::{Param, ShaderType},
            points::PointsFormat,
            settings::{
                CustomIndexedObjectSettings, CustomObjectSettings,
                serde_help::{add_type_id, add_type_id_deserialize, remove_type_id_serialize},
            },
        },
        end::End,
        settings::edit_object_base_settings::{ColorInput, EditObjectBaseSettings},
    },
    screenshots::ColorFormat,
};

#[derive(Clone, Serialize, Deserialize)]
pub struct Settings {
    #[serde(skip)]
    #[serde(default = "Settings::get_default_path")]
    config_path: PathBuf,

    #[serde(skip)]
    #[serde(default = "Settings::get_xdg_dir")]
    xdg_dirs: BaseDirectories,

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
    pub fn new(config_path: PathBuf, xdg_dirs: BaseDirectories) -> Self {
        Self {
            config_path,
            xdg_dirs,
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

    pub fn get_xdg_dir() -> BaseDirectories {
        xdg::BaseDirectories::with_prefix(env!("CARGO_PKG_NAME"))
    }

    fn get_default_path() -> PathBuf {
        Self::get_path(None)
    }

    pub fn get_path(xdg_dirs: Option<&BaseDirectories>) -> PathBuf {
        xdg_dirs
        .map(|xdg_dirs| xdg_dirs.create_config_directory(""))
            .unwrap_or_else(|| Self::get_xdg_dir().create_config_directory(""))
            .unwrap_or(".".into())
            .join("config.yaml")
    }

    pub fn load(
        args: Option<Args>,
        config_path: Option<PathBuf>,
        xdg_dirs: Option<BaseDirectories>,
    ) -> Self {
        let args = args.unwrap_or(Args::parse());
        let xdg_dirs = xdg_dirs.unwrap_or_else(Self::get_xdg_dir);
        let config_path = config_path.unwrap_or(Self::get_path(
            Some(&xdg_dirs),
        ));

        let mut result = fs::OpenOptions::new()
            .read(true)
            .open(&config_path)
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
            .unwrap_or_else(|| Settings::new(config_path, xdg_dirs));

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
        let base_save_data = Self::new(self.config_path.clone(), self.xdg_dirs.clone());
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
