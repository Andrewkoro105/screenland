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
        let id = self.focus_id;
        Subscription::batch(vec![
            window::events().filter_map(move |(id, event)| Self::focused_event_handler(id, event)),
            event::listen()
                .with(id)
                .filter_map(move |(id, event)| Self::event_handler(id, event)),
        ])
    }

    fn focused_event_handler(id: window::Id, event: window::Event) -> Option<Message> {
        match event {
            window::Event::Focused => Some(Message::FocusId(Some(id))),
            _ => None,
        }
    }

    fn event_handler(id: Option<window::Id>, event: Event) -> Option<Message> {
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
