use glam::Vec2;
use iced::{Element, Length, widget::Shader, window};

use crate::app::{Message, Mode, Screenland, shader};

impl Screenland {
    pub fn view(&self, id: window::Id) -> Element<'_, Message> {
        let window_data = self.windows_data.get(&id).unwrap();
        let monitor_pos = Vec2::new(window_data.pos.0 as _, window_data.pos.1 as _);

        let result = Shader::new(shader::Program {
            monitor_pos,
            command: match &self.mode {
                Mode::Base => shader::Command::None,
                Mode::Selection(_) => shader::Command::Selection(self.selection),
            },
        })
        .width(Length::Fill)
        .height(Length::Fill)
        .into();
        result
    }
}
