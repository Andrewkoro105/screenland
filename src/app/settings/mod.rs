use serde::{Deserialize, Serialize};
use std::{fs::{self, OpenOptions}, path::PathBuf};

use crate::app::end::End;

#[derive(Clone, Serialize, Deserialize)]
pub struct Settings {
    config_path: PathBuf,

    pub path: PathBuf,
    #[serde(skip_serializing)]
    #[serde(default)]
    pub cli_path: bool,
    pub format: String,
    #[serde(skip_serializing)]
    #[serde(default)]
    pub cli_format: bool,
    pub base_end: Option<End>,
    #[serde(skip_serializing)]
    #[serde(default)]
    pub cli_base_end: bool,
}

impl Settings {
    pub fn new(config_path: PathBuf) -> Self {
        Self {
            path: PathBuf::from("./"),
            cli_path: false,
            format: String::from("screenshot_%Y-%m-%d_%H:%M:%S.png"),
            cli_format: false,
            base_end: None,
            cli_base_end: false,
            config_path
        }
    }

    pub fn save(&self) {
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
                .unwrap_or_else(|err| panic!("Unable to open file: {:?}. Error: {err}", self.config_path)),
            &self,
        )
        .unwrap();
    }
}
