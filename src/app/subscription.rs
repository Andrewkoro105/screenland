use std::time::{Duration, Instant};

use iced::{
    Event, Subscription, event,
    keyboard::{self, Key, key::Named},
    mouse, window,
};

use crate::app::{Message, START_TIME, Screenland};

impl Screenland {
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
}
