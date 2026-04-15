use std::{
    path::{Path, PathBuf}, str::FromStr,
};

use serde::{Deserialize, Serialize};

use crate::{Args, app::end::End, screenshots::ColorFormat};

#[derive(Clone, Serialize, Deserialize)]
pub struct Settings {
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
}

impl Default for Settings {
    fn default() -> Self {
        Self {
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
        }
    }
}

impl Settings {
    pub fn load(path: &Path, args: &Args) -> Option<Self> {
        let mut result: Option<Self> = super::base_load(path, "Settings");
        if let Some(result) = result.as_mut() {
            if let Some(path) = args.path.as_ref() {
                result.path = PathBuf::from(path);
                result.cli_path = true;
            }

            if let Some(format) = args.format.as_ref() {
                result.format = format.clone();
                result.cli_format = true;
            }

            if let Some(color_format) = args.color_format.as_ref() {
                result.color_format = ColorFormat::from_str(&color_format).unwrap();
                result.cli_format = true;
            }

            if args.disable_overlay {
                result.disables_overlay = true;
                result.cli_disables_overlay = true;
            }

            result.output_shader_and_run = args.output_shader_and_run;

            if let Some(end) = args.end.as_ref() {
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
        }
        result
    }

    pub(super) fn save(&self, path: &Path) {
        let mut save_data = self.clone();
        let base_save_data = Self::default();
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

        super::base_save(self, path, "Settings");
    }
}
