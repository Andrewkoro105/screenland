use glam::Vec2;
use iced::{
    Alignment, Border, Element, Length, Theme,
    widget::{Column, Row, Shader, button, container, row, stack, text_input},
    window,
};

use crate::app::{
    Mode, Screenland, edit_object::CreateObjects, settings::edit_object_base_settings, shader,
    update::Message,
};

use crate::app::edit_object::EditObjectSettings;

impl Screenland {
    pub fn view(&self, id: window::Id) -> Element<'_, Message> {
        let is_transparency = match self.mode {
            Mode::Transparency => true,
            _ => false,
        };
        if let Some(window_data) = self.windows_data.get(&id) {
            let monitor_pos = Vec2::new(window_data.pos.0 as _, window_data.pos.1 as _);
            stack![
                Shader::new(shader::Program {
                    monitor_pos,
                    commands: match &self.mode {
                        Mode::Base => vec![
                            shader::Command::Points(
                                [
                                    self.selection.get_ui_point(),
                                    self.current_object
                                        .map(|current_object| self.objects[current_object]
                                            .get_ui_point())
                                        .unwrap_or_default()
                                ]
                                .concat()
                            ),
                            shader::Command::UpdateEditObjects {
                                shader_objects: self.shader_objects.clone(),
                                custom_objects_channel: self.custom_objects_channel.clone(),
                            }
                        ],
                        Mode::Move(_) => vec![
                            shader::Command::Selection(self.selection),
                            shader::Command::Points(
                                [
                                    self.selection.get_ui_point(),
                                    self.current_object
                                        .map(|current_object| self.objects[current_object]
                                            .get_ui_point())
                                        .unwrap_or_default()
                                ]
                                .concat()
                            ),
                            shader::Command::UpdateEditObjects {
                                shader_objects: self.shader_objects.clone(),
                                custom_objects_channel: self.custom_objects_channel.clone(),
                            },
                        ],
                        Mode::Transparency => vec![
                            shader::Command::Selection(self.selection.add(100000.)),
                            shader::Command::Points(vec![]),
                        ],
                    },
                })
                .width(Length::Fill)
                .height(Length::Fill),
                if window_data.pos == (0, 0) && !is_transparency {
                    stack![self.view_up_menu(), self.view_left_menu()]
                        .height(Length::Fill)
                        .width(Length::Fill)
                } else {
                    stack![]
                }
            ]
            .into()
        } else {
            row![].into()
        }
    }

    fn view_left_menu(&self) -> Element<'_, Message> {
        if let Some(current_object) = self.current_object {
            container(
                container(self.objects[current_object].get_menu())
                    .width(Length::Fixed(200.))
                    .padding(10)
                    .style(Self::style_menu),
            )
            .padding(10)
            .align_x(Alignment::Start)
            .align_y(Alignment::Center)
            .height(Length::Fill)
            .width(Length::Fill)
            .into()
        } else {
            "".into()
        }
    }

    fn view_up_menu(&self) -> Element<'_, Message> {
        container(
            container(
                row![
                    "color: ",
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
                                    edit_object_base_settings::Message::SetColorB(a),
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
                    if self.settings.custom_objects.len() > 0 {
                        " | "
                    } else {
                        ""
                    },
                    Row::from_iter(self.settings.custom_objects.iter().enumerate().map(
                        |(i, object)| {
                            button(object.get_icon().map(|()| unreachable!()))
                                .on_press(Message::AddObject(CreateObjects::Custom(i)))
                                .into()
                        }
                    ),)
                    .spacing(10.),
                ]
                .spacing(10.)
                .align_y(Alignment::Center),
            )
            .padding(10)
            .style(Self::style_menu),
        )
        .padding(10)
        .align_x(Alignment::Center)
        .width(Length::Fill)
        .into()
    }

    fn style_menu(theme: &Theme) -> container::Style {
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
    }
}
