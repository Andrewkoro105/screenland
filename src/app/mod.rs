pub mod settings;
pub mod end;
mod edit_object;
mod selection;
mod shader;
mod subscription;
mod update;
mod view;

use crate::{
    app::{selection::{Selection}, settings::Settings, update::Message},
    screenshots::{MonitorData, get_outputs},
};
use glam::Vec2;
use iced::{
    Task, application::BootFn, window::{self, settings::PlatformSpecific}
};
use std::{collections::HashMap, sync::OnceLock, time::Instant};

pub static START_TIME: OnceLock<Instant> = OnceLock::new();

#[derive(Default)]
pub enum Mode {
    #[default]
    Base,
    Move(selection::Message),
    Selection,
    Transparency,
}

pub struct Screenland {
    auto_exit: bool,
    windows_data: HashMap<window::Id, MonitorData>,
    selection: Selection,
    mode: Mode,
    mouse_pos: Vec2,
    settings: Settings,
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
                settings: self.clone()
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
}
