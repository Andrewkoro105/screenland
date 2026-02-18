use glam::Vec2;
use iced::{
    Alignment, Border, Element, Length, Theme, widget::{Shader, container, row, stack, text_input}, window
};

use crate::app::{
    Mode, Screenland, edit_object::EditObject, settings::edit_object_base_settings, shader,
    update::Message,
};

impl Screenland {
    pub fn view(&self, id: window::Id) -> Element<'_, Message> {
        let window_data = self.windows_data.get(&id).unwrap();
        let monitor_pos = Vec2::new(window_data.pos.0 as _, window_data.pos.1 as _);
        stack![
            Shader::new(shader::Program {
                monitor_pos,
                edit_object_base_settings: self.settings.edit_object_base_settings.clone().into(),
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
                    Mode::Transparency => vec![
                        shader::Command::Selection(self.selection.add(100000.)),
                        shader::Command::Points(vec![]),
                    ],
                },
            })
            .width(Length::Fill)
            .height(Length::Fill),
            if window_data.pos == (0, 0) && self.mode != Mode::Transparency {
                self.view_up_menu()
            } else {
                row![].into()
            }
        ]
        .into()
    }

    fn view_up_menu(&self) -> Element<'_, Message> {
        container(
            container(
                row![
                    "color:",
                    container(
                        self.settings
                            .edit_object_base_settings
                            .color
                            .r
                            .view("", |a| {
                                Message::EditObjectBaseSettings(
                                    edit_object_base_settings::Message::SetColorR(a),
                                )
                            })
                    )
                    .width(40),
                    container(
                        self.settings
                            .edit_object_base_settings
                            .color
                            .g
                            .view("", |a| {
                                Message::EditObjectBaseSettings(
                                    edit_object_base_settings::Message::SetColorG(a),
                                )
                            })
                    )
                    .width(40),
                    container(
                        self.settings
                            .edit_object_base_settings
                            .color
                            .b
                            .view("", |a| {
                                Message::EditObjectBaseSettings(
                                    edit_object_base_settings::Message::SetColorG(a),
                                )
                            })
                    )
                    .width(40),
                    container(
                        self.settings
                            .edit_object_base_settings
                            .color
                            .a
                            .view("", |a| {
                                Message::EditObjectBaseSettings(
                                    edit_object_base_settings::Message::SetColorA(a),
                                )
                            })
                    )
                    .width(40),
                    "size: ",
                    container(self.settings.edit_object_base_settings.size.view("", |a| {
                        Message::EditObjectBaseSettings(
                            edit_object_base_settings::Message::SetSize(a),
                        )
                    }))
                    .width(40),
                ]
                .spacing(10.)
                .align_y(Alignment::Center),
            )
            .padding(10)
            .style(|theme| {
                let mut result =
                    container::Catalog::style(theme, &<Theme as container::Catalog>::default());
                result.background = Some(
                    text_input::Catalog::style(
                        theme,
                        &<Theme as text_input::Catalog>::default(),
                        text_input::Status::Active,
                    )
                    .background,
                );
                result.border = Border::default().rounded(5);
                result
            }),
        )
        .padding(10)
        .align_x(Alignment::Center)
        .width(Length::Fill)
        .into()
    }
}
