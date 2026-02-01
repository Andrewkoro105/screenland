use std::time::{Duration, Instant};

use iced::{
    Event, Subscription, event,
    keyboard::{
        self, Key, Modifiers,
        key::{self, Named},
    },
    mouse, window,
};

use crate::app::{START_TIME, Screenland, end::End, update::Message};

impl Screenland {
    pub fn subscription(&self) -> Subscription<Message> {
        Subscription::batch(vec![
            event::listen_raw(move |event, status, id| Self::event_handler(id, status, event)),
        ])
    }

    fn event_handler(id: window::Id, status: event::Status,  event: Event) -> Option<Message> {
        match event {
            Event::Keyboard(keyboard::Event::KeyPressed {
                key: Key::Named(Named::Escape),
                ..
            }) => Some(Message::Exit),
            Event::Keyboard(keyboard::Event::KeyPressed {
                physical_key: key::Physical::Code(key::Code::KeyS),
                modifiers: Modifiers::CTRL,
                ..
            }) => Some(Message::End(End::Save)),
            Event::Keyboard(keyboard::Event::KeyPressed {
                physical_key: key::Physical::Code(key::Code::KeyC),
                modifiers: Modifiers::CTRL,
                ..
            }) => Some(Message::End(End::Copy)),
            Event::Mouse(mouse::Event::CursorMoved { position }) => {
                Some(Message::MoveMouse(position, id))
            }
            Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Left)) => {
                (status == event::Status::Ignored).then_some(Message::TouchStart)
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
            Event::Window(window::Event::Closed) => Some(Message::AutoExit),
            Event::Window(window::Event::Moved(_)) => START_TIME.get().and_then(|start_time| {
                (start_time.elapsed() > Duration::new(1, 0)).then_some(Message::AutoExit)
            }),
            Event::Window(window::Event::Resized(_)) => START_TIME.get().and_then(|start_time| {
                (start_time.elapsed() > Duration::new(1, 0)).then_some(Message::AutoExit)
            }),
            _ => None,
        }
    }
}
