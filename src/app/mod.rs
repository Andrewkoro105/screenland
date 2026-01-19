use crate::{
    screenshots::{MonitorData, screenshots},
};
use iced::{
    Element, Event, Subscription, Task, event, exit, keyboard::{self, Key, key::Named}, window::{self, Settings}
};
use image::RgbaImage;

#[derive(Clone)]
pub enum Message {
    Close
}

pub struct Screenland {
    imgs: Vec<(MonitorData, RgbaImage, window::Id)>,
}

impl Screenland {
    pub fn new() -> (Self, Task<Message>) {
        let mut windows_task = Task::none();
        let screenshots = screenshots();
        let mut windows = vec![];

        for _ in 0..screenshots.len() {
            let (id, window_task) = window::open(Settings {
                fullscreen: true,
                ..Default::default()
            });
            windows_task = windows_task.chain(window_task.discard());

            windows.push(id);
        }

        (
            Self {
                imgs: screenshots
                    .into_iter()
                    .zip(windows)
                    .map(|((a, b), c)| (a, b, c))
                    .collect(),
            },
            windows_task,
        )
    }

    pub fn theme(&self, _id: window::Id) -> iced::Theme {
        iced::Theme::Dark
    }

    pub fn titel(&self, current_id: window::Id) -> String {
        format!(
            "screenland-{}",
            self.imgs
                .iter()
                .find_map(|(monitor_data, _, id)| {
                    (current_id == *id).then_some(monitor_data.name.clone())
                })
                .unwrap()
        )
    }

    pub fn subscription(&self) -> Subscription<Message> {
        Subscription::batch(vec![
            window::close_events().map(|_| Message::Close),
            event::listen().filter_map(|event| {
                if let Event::Keyboard(keyboard::Event::KeyPressed {
                    key: Key::Named(key),
                    ..
                }) = event {
                    match key {
                        Named::Escape => Some(Message::Close),
                        _ => None,
                    }
                } else {
                    None
                }
            }),
        ])
    }

    pub fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::Close => exit(),
        }
    }

    pub fn view(&self, current_id: window::Id) -> Element<'_, Message> {
        let (_, img, _) = self.imgs.iter().find(|(_, _, id)| current_id == *id).unwrap();

        iced::widget::image(iced::widget::image::Handle::from_rgba(
            img.width(),
            img.height(),
            img.to_vec()
        )).into()
    }
}