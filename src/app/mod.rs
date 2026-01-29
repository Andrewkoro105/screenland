mod object;
mod selection;
mod shader;

use crate::{
    app::{selection::Selection},
    screenshots::{MonitorData, get_outputs},
};
use glam::Vec2;
use iced::{
    Element, Event, Length, Point, Subscription, Task, event, exit,
    keyboard::{self, Key, key::Named},
    mouse,
    widget::Shader,
    window::{self, Settings, settings::PlatformSpecific},
};
use std::{
    collections::HashMap,
    sync::OnceLock,
    time::{Duration, Instant},
};

pub static START_TIME: OnceLock<Instant> = OnceLock::new();

#[derive(Clone)]
pub enum Message {
    Close,
    MoveMouse(Point),
    TouchStart,
    TouchEnd,
}

#[derive(Default)]
pub enum SelectionMode {
    #[default]
    None,
    Start,
    End,
}

pub enum Mode {
    Selection(SelectionMode),
}

impl Default for Mode {
    fn default() -> Self {
        Mode::Selection(Default::default())
    }
}

pub struct Screenland {
    windows_data: HashMap<window::Id, MonitorData>,
    selection: Selection,
    mode: Mode,
    mouse_pos: Vec2,
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

    pub fn subscription(&self) -> Subscription<Message> {
        Subscription::batch(vec![event::listen().filter_map(|event| match event {
            Event::Keyboard(keyboard::Event::KeyPressed {
                key: Key::Named(Named::Escape),
                ..
            }) => Some(Message::Close),
            Event::Mouse(mouse::Event::CursorMoved { position }) => {
                Some(Message::MoveMouse(position))
            }
            Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Left)) => {
                Some(Message::TouchStart)
            }
            Event::Mouse(mouse::Event::ButtonReleased(mouse::Button::Left)) => {
                Some(Message::TouchEnd)
            }
            Event::Window(window::Event::Opened { .. }) => {
                if START_TIME.get().is_none() {
                    let _ = START_TIME.set(Instant::now());
                }
                None
            }
            Event::Window(window::Event::Closed) => Some(Message::Close),
            Event::Window(window::Event::Moved(_)) => {
                if let Some(start_time) = START_TIME.get() {
                    (start_time.elapsed() > Duration::new(1, 0)).then_some(Message::Close)
                } else {
                    None
                }
            }
            Event::Window(window::Event::Resized(_)) => {
                if let Some(start_time) = START_TIME.get() {
                    (start_time.elapsed() > Duration::new(1, 0)).then_some(Message::Close)
                } else {
                    None
                }
            }
            _ => None,
        })])
    }

    pub fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::Close => exit(),
            Message::MoveMouse(point) => {
                self.mouse_pos = Vec2 {
                    x: point.x,
                    y: point.y,
                };

                match &self.mode {
                    Mode::Selection(mode) => {
                        match mode {
                            SelectionMode::None => {},
                            SelectionMode::Start => self.selection.end = self.mouse_pos,
                            SelectionMode::End => {},
                        }
                        Task::none()
                    }
                }
            }
            Message::TouchStart => match self.mode {
                Mode::Selection(_) => {
                    self.selection.start = self.mouse_pos;
                    self.selection.end = self.mouse_pos;
                    self.mode = Mode::Selection(SelectionMode::Start);
                    Task::none()
                }
            },
            Message::TouchEnd => match self.mode {
                Mode::Selection(_) => {
                    self.mode = Mode::Selection(SelectionMode::End);
                    Task::none()
                }
            },
        }
    }

    pub fn view(&self, id: window::Id) -> Element<'_, Message> {
        let window_data = self.windows_data.get(&id).unwrap();
        let monitor_pos = Vec2::new(window_data.pos.0 as _, window_data.pos.1 as _);

        let result = Shader::new(
            shader::Program {
                monitor_pos,
                command: match &self.mode {
                    Mode::Selection(_) => shader::Command::Selection(self.selection),
                }
            },
        )
        .width(Length::Fill)
        .height(Length::Fill)
        .into();
        result
    }
}
