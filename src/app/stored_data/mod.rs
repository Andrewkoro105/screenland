pub mod default_stored_data;
pub mod edit_object_base_settings;
pub mod path_system;
pub mod settings;
use clap::Parser;
use image::RgbaImage;
use serde::{Serialize, de::DeserializeOwned};
use std::{
    fs::{self, DirEntry},
    path::{Path, PathBuf},
    sync::{Arc, Mutex},
};

use crate::{
    Args,
    app::{
        edit_object::custom_object::settings::{
            CustomIndexedObjectSettings, CustomObjectSettings,
            serde_help::{add_type_id, remove_type_id},
        },
        end::End,
        stored_data::{
            edit_object_base_settings::EditObjectBaseSettings,
            path_system::{PathSystem, PathType},
            settings::Settings,
        },
    },
};

#[derive(Clone)]
pub struct StoredData {
    path_system: PathSystem,

    pub result: Arc<Mutex<Option<(End, RgbaImage)>>>,

    pub settings: Settings,

    pub edit_object_base_settings: EditObjectBaseSettings,

    pub custom_objects: Vec<CustomIndexedObjectSettings>,
}

impl StoredData {
    pub fn load(
        args: Option<Args>,
        path_system: Option<PathSystem>,
        result_app: Arc<Mutex<Option<(End, RgbaImage)>>>,
    ) -> Self {
        let args = args.unwrap_or_else(|| Args::parse());
        let path_system = path_system.unwrap_or_else(|| PathSystem::from_args(&args));

        let settings = Settings::load(&path_system.get(PathType::Settings), &args);
        let edit_object_base_settings =
            EditObjectBaseSettings::load(&path_system.get(PathType::EditObjectBaseSettings))
                .unwrap_or_default();
        let custom_objects = Self::custom_objects_load(&path_system);

        if let Some(settings) = settings {
            Self {
                path_system,
                result: result_app,
                settings,
                edit_object_base_settings,
                custom_objects,
            }
        } else {
            let mut result = Self::new(path_system, result_app);
            result.edit_object_base_settings = edit_object_base_settings;
            result
        }
    }

    pub fn custom_objects_load(path_system: &PathSystem) -> Vec<CustomIndexedObjectSettings> {
        add_type_id(
            fs::read_dir(path_system.get(PathType::CustomObjects))
                .expect("Unable to read dir for custom_objects")
                .filter_map(|dir_entry| {
                    dir_entry
                        .ok()
                        .as_ref()
                        .map(DirEntry::path)
                        .as_ref()
                        .map(PathBuf::as_path)
                        .filter(|path| path.is_dir())
                        .map(|dir| {
                            let name = dir.file_name().unwrap().to_str().unwrap().to_string();
                            let mut result: CustomObjectSettings =
                                base_load(&dir.join("object.yaml"), "custom_objects").expect(
                                    &format!(
                                        "The “{}” object does not have an “object.yaml” file.",
                                        name
                                    ),
                                );
                            result.name = name;
                            result
                        })
                })
                .collect(),
        )
    }

    pub fn save_edit_objects_base_settings(&self) {
        self.edit_object_base_settings
            .save(&self.path_system.get(PathType::EditObjectBaseSettings));
    }

    pub fn save(&self) {
        self.save_edit_objects_base_settings();
        self.settings
            .save(&self.path_system.get(PathType::Settings));
        for custom_object in remove_type_id(self.custom_objects.clone()).iter() {
            base_save(
                custom_object,
                &self
                    .path_system
                    .get(PathType::CustomObjects)
                    .join(&custom_object.name)
                    .join("object.yaml"),
                "custom_object",
            );
        }
    }
}

pub(self) fn base_load<T: DeserializeOwned>(path: &Path, name: &str) -> Option<T> {
    fs::OpenOptions::new()
        .read(true)
        .open(&path)
        .map(|file| {
            let result = serde_saphyr::from_reader(file);
            if let Err(err) = &result {
                panic!("{name} parsing error in file {path:?}:\n{err}");
            }
            result.ok()
        })
        .inspect_err(|err| match err.kind() {
            std::io::ErrorKind::NotFound => {}
            _ => panic!("Unable to open file: {path:?} for parsing {name}. Error: {err}"),
        })
        .ok()
        .flatten()
}
pub(self) fn base_save<T: Serialize>(data: &T, path: &Path, name: &str) {
    if let Some(parent) = path.parent()
        && !parent.exists()
    {
        fs::create_dir_all(parent).unwrap();
    }
    serde_saphyr::to_io_writer(
        &mut fs::OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(path)
            .unwrap_or_else(|err| {
                panic!("Unable to open file: {path:?} for save {name}. Error: {err}")
            }),
        data,
    )
    .unwrap();
}
