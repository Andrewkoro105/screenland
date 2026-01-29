mod end;
mod selection;
mod shader;
mod subscription;
mod update;
mod view;

use crate::{
    app::{selection::Selection, update::Message},
    screenshots::{MonitorData, get_outputs},
};
use glam::Vec2;
use iced::{
    Task,
    window::{self, Settings, settings::PlatformSpecific},
};
use std::{collections::HashMap, sync::OnceLock, time::Instant};

pub static START_TIME: OnceLock<Instant> = OnceLock::new();

pub enum SelectionMode {
    Start,
    End,
}

#[derive(Default)]
pub enum Mode {
    #[default]
    Base,
    Selection(SelectionMode),
}

pub struct Screenland {
    transparency: bool,
    auto_exit: bool,
    windows_data: HashMap<window::Id, MonitorData>,
    selection: Selection,
    mode: Mode,
    mouse_pos: Vec2,
    focus_id: Option<window::Id>,
}

impl Screenland {
    pub fn new() -> (Self, Task<Message>) {
        let mut windows_task = Task::none();
        let mut windows_data = HashMap::new();

        for monitor_data in get_outputs() {
            let (id, window_task) = window::open(Settings {
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
            Self {
                windows_data,
                selection: Default::default(),
                mode: Default::default(),
                mouse_pos: Default::default(),
                transparency: false,
                auto_exit: true,
                focus_id: None
            },
            windows_task,
        )
    }

    pub fn theme(&self, _id: window::Id) -> iced::Theme {
        iced::Theme::Dark
    }

    pub fn title(&self, id: window::Id) -> String {
        format!("screenland-{}", self.windows_data.get(&id).unwrap().name)
    }
}
