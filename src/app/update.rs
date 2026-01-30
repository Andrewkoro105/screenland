use glam::Vec2;
use iced::{Point, Task, exit, window};

use crate::app::{Mode, Screenland, SelectionMode, end::End};

#[derive(Clone)]
pub enum Message {
    Exit,
    AutoExit,
    Transparency(bool),
    FocusId(Option<window::Id>),
    MoveMouse(Point, Option<window::Id>),
    TouchStart,
    TouchEnd,
    End(End),
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
            Message::FocusId(id) => {
                self.focus_id = id;
                Task::none()
            }
            Message::MoveMouse(point, id) => {
                self.mouse_pos = Vec2 {
                    x: point.x,
                    y: point.y,
                } + id
                    .and_then(|id| self.windows_data.get(&id))
                    .map(|window_data| Vec2 {
                        x: window_data.pos.0 as _,
                        y: window_data.pos.1 as _,
                    })
                    .unwrap_or(Vec2 { x: 0., y: 0. });

                match &self.mode {
                    Mode::Base => Task::none(),
                    Mode::Selection(mode) => {
                        match mode {
                            SelectionMode::Start => self.selection.end = self.mouse_pos,
                            SelectionMode::End => {}
                        }
                        Task::none()
                    }
                }
            }
            Message::TouchStart => match self.mode {
                Mode::Base => {
                    self.mode = Mode::Selection(SelectionMode::Start);
                    self.selection.start = self.mouse_pos;
                    self.selection.end = self.mouse_pos;
                    Task::none()
                }
                Mode::Selection(_) => {
                    self.mode = Mode::Selection(SelectionMode::Start);
                    self.selection.start = self.mouse_pos;
                    self.selection.end = self.mouse_pos;
                    Task::none()
                }
            },
            Message::TouchEnd => match self.mode {
                Mode::Base => Task::none(),
                Mode::Selection(_) => {
                    if let Some(end) = &self.settings.base_end {
                        Task::done(Message::End(end.clone()))
                    } else {
                        self.mode = Mode::Selection(SelectionMode::End);
                        Task::none()
                    }
                }
            },
            Message::Transparency(transparency) => {
                self.transparency = transparency;
                Task::none()
            }
            Message::End(end) => {
                self.auto_exit = false;
                let selection = self.selection;
                let windows_data = self.windows_data.clone();
                let settings = self.settings.clone();
                Task::done(Message::Transparency(true)).chain(
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
                    .chain(exit())
                    ,
                )
            }
        }
    }
}
