mod shader;

use crate::{
    app::{shader::ScreenlandShaderProgram},
    screenshots::{MonitorData, get_outputs},
};
use glam::Vec2;
use iced::{
    Element, Event, Length, Subscription, Task,
    event, exit,
    keyboard::{self, Key, key::Named},
    mouse,
    widget::Shader,
    window::{self, Settings},
};
use std::{
    collections::HashMap, sync::OnceLock, time::{Duration, Instant}
};

pub static START_TIME: OnceLock<Instant> = OnceLock::new();

#[derive(Clone)]
pub enum Message {
    Close,
}


pub struct Screenland {
    windows_data: HashMap<window::Id, MonitorData>,
}

impl Screenland {
    pub fn new() -> (Self, Task<Message>) {
        let mut windows_task = Task::none();
        let mut windows_data = HashMap::new();

        for monitor_data in get_outputs() {
            let (id, window_task) = window::open(Settings {
                fullscreen: true,
                ..Default::default()
            });
            windows_task = windows_task.chain(window_task.discard());

            windows_data.insert(id, monitor_data);
        }

        (
            Self {
                windows_data,
            },
            windows_task,
        )
    }

    pub fn theme(&self, _id: window::Id) -> iced::Theme {
        iced::Theme::Dark
    }

    pub fn title(&self, id: window::Id) -> String {
        format!(
            "screenland-{}",
            self.windows_data.get(&id).unwrap().name
        )
    }

    pub fn subscription(&self) -> Subscription<Message> {
        Subscription::batch(vec![event::listen().filter_map(|event| {
            match event {
                Event::Keyboard(keyboard::Event::KeyPressed {
                    key: Key::Named(Named::Escape),
                    ..
                }) => Some(Message::Close),
                Event::Mouse(mouse::Event::CursorMoved { position: _ }) => {
                    None // Some(Message::MoveMouse(position))
                }
                Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Left)) => {
                    None // Some(Message::TouchStart)
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
            }
        })])
    }

    pub fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::Close => exit(),
        }
    }

    pub fn view(&self, id: window::Id) -> Element<'_, Message> {
        let window_data = self.windows_data.get(&id).unwrap();
        let result = Shader::new(ScreenlandShaderProgram{monitor_pos: Vec2::new(window_data.pos.0 as _, window_data.pos.1 as _) })
            .width(Length::Fill)
            .height(Length::Fill)
            .into();
        result
    }
}
