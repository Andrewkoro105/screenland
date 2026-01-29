use glam::Vec2;
use iced::{Task, exit};

use crate::app::{Message, Mode, Screenland, SelectionMode};

impl Screenland {
    pub fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::Close => exit(),
            Message::MoveMouse(point) => {
                self.mouse_pos = Vec2 {
                    x: point.x,
                    y: point.y,
                };

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
                    self.mode = Mode::Selection(SelectionMode::End);
                    Task::none()
                }
            },
        }
    }
}
