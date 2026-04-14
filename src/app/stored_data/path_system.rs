use std::path::PathBuf;

use xdg::BaseDirectories;

use crate::Args;

pub enum PathType {
    Settings,
    CustomObjects,
    EditObjectBaseSettings,
    Log,
}

#[derive(Clone)]
pub struct PathSystem {
    settings_path_dir: PathBuf,
    xdg: BaseDirectories,
}

impl PathSystem {
    pub fn from_args(args: &Args) -> Self {
        let xdg = xdg::BaseDirectories::with_prefix(env!("CARGO_PKG_NAME"));
        Self {
            settings_path_dir: args
                .config
                .clone()
                .map(Into::into)
                .unwrap_or(xdg.create_config_directory("").unwrap_or(".".into())),
            xdg,
        }
    }

    pub fn get(&self, path_type: PathType) -> PathBuf {
        match path_type {
            PathType::Settings => self.settings_path_dir.join("config.yaml"),
            PathType::CustomObjects => self
                .xdg
                .create_config_directory("custom_objects")
                .expect("Unable to create or open config dir"),
            PathType::EditObjectBaseSettings => self
                .xdg
                .create_state_directory("")
                .expect("Unable to create or open state dir")
                .join("edit_objects_base_settings.yaml"),
            PathType::Log => self
                .xdg
                .create_state_directory("log")
                .expect("Unable to create or open state dir"),
        }
    }
}
