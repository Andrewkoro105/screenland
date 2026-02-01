use glam::Vec2;
use iced::{Point, Task, exit, window};

use crate::app::{Mode, Screenland, edit_object::EditObject, end::End, selection};

#[derive(Clone)]
pub enum Message {
    Exit,
    AutoExit,
    Transparency,
    MoveMouse(Point, window::Id),
    TouchStart,
    TouchEnd,
    End(End),
    SelectionUpdate(selection::Message),
}

impl Screenland {
    pub fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::Exit => exit(),
            Message::AutoExit => {
                if self.auto_exit {
                    Task::done(Message::Exit)
                } else {
                    Task::none()
                }
            }
            Message::MoveMouse(point, id) => {
                self.mouse_pos = Vec2 {
                    x: point.x,
                    y: point.y,
                } + self
                    .windows_data
                    .get(&id)
                    .map(|window_data| Vec2 {
                        x: window_data.pos.0 as _,
                        y: window_data.pos.1 as _,
                    })
                    .unwrap_or(Vec2 { x: 0., y: 0. });

                match &self.mode {
                    Mode::Base => Task::none(),
                    Mode::Move(message) => {
                        self.update(Message::SelectionUpdate(message.clone()))
                    }
                    Mode::Selection => {
                        self.selection.end = self.mouse_pos;
                        Task::none()
                    }
                    Mode::Transparency => Task::none(),
                }
            }
            Message::TouchStart => match self.mode {
                Mode::Base => {
                    let select_message = self.selection.get_messages(&self.mouse_pos);
                    if !select_message.is_empty() {
                        self.mode = Mode::Move(select_message[0].clone());
                    } else {
                        self.mode = Mode::Selection;
                        self.selection.start = self.mouse_pos;
                        self.selection.end = self.mouse_pos;
                    }
                    Task::none()
                }
                Mode::Move(_) => Task::none(),
                Mode::Selection => {
                    self.mode = Mode::Selection;
                    self.selection.start = self.mouse_pos;
                    self.selection.end = self.mouse_pos;
                    Task::none()
                }
                Mode::Transparency => Task::none(),
            },
            Message::TouchEnd => match self.mode {
                Mode::Base => Task::none(),
                Mode::Move(_) => {
                    self.mode = Mode::Base;
                    Task::none()
                }
                Mode::Selection => {
                    if let Some(end) = &self.settings.base_end {
                        Task::done(Message::End(end.clone()))
                    } else {
                        self.mode = Mode::Base;
                        Task::none()
                    }
                }
                Mode::Transparency => Task::none(),
            },
            Message::Transparency => {
                self.mode = Mode::Transparency;
                Task::none()
            }
            Message::End(end) => {
                self.auto_exit = false;
                let selection = self.selection;
                let windows_data = self.windows_data.clone();
                let settings = self.settings.clone();
                Task::done(Message::Transparency).chain(
                    Task::future(async move {
                        let screen = Self::screenshot(selection);

                        let mut windows_task = Task::<Message>::none();

                        for (id, _) in windows_data.iter() {
                            windows_task = windows_task.chain(window::close(*id));
                        }
                        windows_task.chain(
                            Task::future(async move {
                                end.end(&settings, screen);
                            })
                            .discard(),
                        )
                    })
                    .then(|task| task)
                    .chain(exit()),
                )
            }
            Message::SelectionUpdate(message) => self
                .selection
                .update(self.mouse_pos, message)
                .map(Message::SelectionUpdate),
        }
    }
}
