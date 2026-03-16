mod edit_object;
pub mod end;
mod selection;
pub mod settings;
pub mod shader;
mod subscription;
mod update;
mod view;

use crate::{
    app::{
        edit_object::{
            EditObject,
            custom_object::{self, CustomObjectFromShader},
        },
        selection::Selection,
        settings::Settings,
        update::Message,
    },
    screenshots::{MonitorData, get_outputs},
};
use glam::Vec2;
use iced::{
    Task,
    application::BootFn,
    window::{self, settings::PlatformSpecific},
};
use std::{collections::HashMap, sync::OnceLock, time::Instant};

pub static START_TIME: OnceLock<Instant> = OnceLock::new();

/// `Mode` indicates the basic state of the program that affects how objects are drawn and updated, as well as what data is sent to the shader.
#[derive(Default, Clone, PartialEq, Eq)]
pub enum Mode {
    /// Select the point to move, and if you click outside the points, switch to `Selection` mode.
    #[default]
    Base,
    /// Mode for moving the specified point
    Move(selection::Message),
    /// First allocation mode
    Selection,
    /// Mode in which the interface is not visible
    Transparency,
}

pub struct Screenland {
    auto_exit: bool,
    windows_data: HashMap<window::Id, MonitorData>,
    selection: Selection,
    mode: Mode,
    mouse_pos: Vec2,
    settings: Settings,
    objects: Vec<edit_object::Objects>,
    shader_objects: Vec<edit_object::ShaderObjects>,
    custom_objects_chenel: custom_object::param::Chanel,
}

impl BootFn<Screenland, Message> for Settings {
    fn boot(&self) -> (Screenland, Task<Message>) {
        let mut windows_task = Task::none();
        let mut windows_data = HashMap::new();

        for monitor_data in get_outputs() {
            let (id, window_task) = window::open(window::Settings {
                fullscreen: true,
                platform_specific: PlatformSpecific {
                    application_id: "screenland".into(),
                    ..Default::default()
                },
                ..Default::default()
            });
            windows_task = windows_task.chain(window_task.discard());

            windows_data.insert(id, monitor_data);
        }

        (
            Screenland {
                windows_data,
                selection: Default::default(),
                mode: Default::default(),
                mouse_pos: Default::default(),
                auto_exit: true,
                settings: self.clone(),
                objects: vec![],
                shader_objects: vec![],
                custom_objects_chenel: Default::default(),
            },
            windows_task,
        )
    }
}

impl Screenland {
    pub fn theme(&self, _id: window::Id) -> iced::Theme {
        iced::Theme::Dark
    }

    pub fn title(&self, id: window::Id) -> String {
        format!("screenland-{}", self.windows_data.get(&id).unwrap().name)
    }

    pub fn reload_shader_objects(&mut self) {
        self.shader_objects.clear();
        self.shader_objects.reserve(self.objects.len());
        for object in &self.objects {
            match object {
                edit_object::Objects::Custom(custom_object) => {
                    self.shader_objects.push(edit_object::ShaderObjects::Custom(
                        CustomObjectFromShader {
                            channel_index: self.custom_objects_chenel.get_index(),
                            custom_object_type: custom_object.get_type_id(),
                        },
                    ));

                    self.custom_objects_chenel.add_f32(custom_object.get_f32_data());
                }
            }
        }
    }
}
