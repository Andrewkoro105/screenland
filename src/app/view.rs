use glam::Vec2;
use iced::{
    Element, Length,
    widget::{Shader, stack},
    window,
};

use crate::app::{Mode, Screenland, edit_object::EditObject, shader, update::Message};

impl Screenland {
    pub fn view(&self, id: window::Id) -> Element<'_, Message> {
        let window_data = self.windows_data.get(&id).unwrap();
        let monitor_pos = Vec2::new(window_data.pos.0 as _, window_data.pos.1 as _);
        stack![
            Shader::new(shader::Program {
                monitor_pos,
                commands: match &self.mode {
                    Mode::Base => vec![shader::Command::None],
                    Mode::Move(_) => vec![
                        shader::Command::Selection(self.selection),
                        shader::Command::Points(self.selection.get_ui_point()),
                    ],
                    Mode::Selection => vec![
                        shader::Command::Selection(self.selection),
                        shader::Command::Points(self.selection.get_ui_point()),
                    ],
                    Mode::Transparency => {
                        vec![
                            shader::Command::Selection(self.selection.add(100000.)),
                            shader::Command::Points(vec![]),
                        ]
                    }
                },
            })
            .width(Length::Fill)
            .height(Length::Fill),
        ]
        .into()
    }
}
