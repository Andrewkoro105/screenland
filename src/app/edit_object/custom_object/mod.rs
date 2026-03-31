pub mod data_type;
pub mod icon;
pub mod param;
pub mod points;
pub mod settings;
use std::{mem, ops::Not};

use glam::Vec2;
use iced::{Task, widget::Column};
use std::fmt::Debug;
use strum::EnumCount;

use crate::app::{
    self,
    edit_object::{
        self, EditObject,
        custom_object::{
            param::{
                Param,
                channel::{ChannelIndex, ChannelType, Channels},
            },
            points::{CustomObjectPointSystem, PointsMessage},
        },
        ui_point::{PointsSystem, UIPoint},
    },
    settings::edit_object_base_settings::EditObjectBaseSettingsFromShader,
};

#[derive(Clone, Debug)]
pub enum Message {
    SetParam(usize, param::Message),
    Point(Option<PointsMessage>),
}

#[derive(Debug)]
pub struct CustomObject {
    type_id: u32,
    i: usize,
    edit_object_base_settings: EditObjectBaseSettingsFromShader,
    points_data: Option<Box<dyn CustomObjectPointSystem>>,
    params: Vec<Param>,
}

#[repr(C)]
#[derive(Clone, Debug, Default)]
pub struct CustomObjectFromShader {
    pub edit_object_base_settings: EditObjectBaseSettingsFromShader,
    pub custom_object_type: u32,
    pub channel_index: ChannelIndex,
}

impl CustomObjectFromShader {
    const PADDING_SIZE: usize = 12;

    pub fn get_size() -> usize {
        std::mem::size_of::<EditObjectBaseSettingsFromShader>()
            + std::mem::size_of::<u32>()
            + (ChannelType::COUNT * std::mem::size_of::<u32>())
            + (std::mem::size_of::<u8>() * Self::PADDING_SIZE)
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        let result = [
            bytemuck::bytes_of(&self.edit_object_base_settings).to_vec(),
            bytemuck::bytes_of(&self.custom_object_type).to_vec(),
            self.channel_index.to_bytes(),
            vec![0; Self::PADDING_SIZE],
        ]
        .concat();
        result
    }
}

impl EditObject for CustomObject {
    fn set_index(&mut self, i: usize) {
        self.i = i;
    }

    fn get_menu(&self) -> Option<iced::Element<'_, app::Message>> {
        self.params.is_empty().not().then_some(
            Column::from_iter(self.params.iter().enumerate().map(|(i, param)| {
                Param::get_menu(param)
                    .map(move |message| Message::SetParam(i, message))
                    .map(|message| {
                        app::Message::UpdateEditObject((
                            self.i,
                            edit_object::Message::Custom(message),
                        ))
                    })
            }))
            .spacing(5)
            .into(),
        )
    }

    fn get_ui_point(&self) -> Vec<UIPoint> {
        self.points_data
            .as_ref()
            .map(|points_data| points_data.get_ui_points())
            .unwrap_or(vec![])
    }

    fn get_messages(&mut self, position: &glam::Vec2) -> Option<app::Message> {
        self.points_data
            .as_mut()
            .map(|points_data| {
                points_data
                    .as_mut()
                    .get_message(position)
                    .map(Some)
                    .map(Message::Point)
                    .map(|message| {
                        app::Message::UpdateEditObject((
                            self.i,
                            edit_object::Message::Custom(message),
                        ))
                    })
            })
            .unwrap_or_default()
    }

    fn update(
        &mut self,
        muse_position: glam::Vec2,
        message: edit_object::Message,
    ) -> iced::Task<app::Message> {
        if let edit_object::Message::Custom(message) = message {
            match message {
                Message::SetParam(index, message) => {
                    let current_discriminant = mem::discriminant(&self.params[index].shader_type);
                    let mut shader_index = 0;
                    for param in &self.params[0..index] {
                        if mem::discriminant(&param.shader_type) == current_discriminant {
                            shader_index += 1;
                        }
                    }

                    self.params[index].shader_type.update(message);
                    Task::done(app::Message::CustomObjectsChannelUpdate {
                        i: self.i,
                        index: shader_index,
                        channel_type: self.params[index].shader_type.clone().into(),
                        data: self.params[index].shader_type.get_data(),
                    })
                }
                Message::Point(points_message) => {
                    let i = self.i;
                    self.points_data
                        .as_mut()
                        .map(move |points_data| {
                            points_data.custom_object_update(i, muse_position, points_message)
                        })
                        .unwrap_or(Task::none())
                }
            }
        } else {
            unreachable!(
                "In CustomObject({}), a message for the wrong object was sent.",
                self.i
            )
        }
    }

    fn get_shader_object(&self, channel: &mut Channels) -> edit_object::ShaderObjects {
        let result = edit_object::ShaderObjects::Custom(CustomObjectFromShader {
            channel_index: channel.get_index(),
            custom_object_type: self.get_type_id(),
            edit_object_base_settings: self.edit_object_base_settings,
        });

        self.params
            .iter()
            .map(|param| {
                (
                    ChannelType::from(param.shader_type.clone()),
                    param.shader_type.get_data(),
                )
            })
            .chain(
                self.points_data
                    .iter()
                    .flat_map(|points_data| points_data.get_data())
                    .map(|(channel_type, data)| (channel_type, data.concat())),
            )
            .for_each(|(channel_type, data)| channel.add(channel_type, data));
        result
    }

    fn in_object(&self, muse_position: Vec2) -> bool {
        self.points_data
            .as_ref()
            .map(|points_data| points_data.in_object(&muse_position))
            .unwrap_or(false)
    }
}

impl CustomObject {
    pub fn get_type_id(&self) -> u32 {
        self.type_id
    }
}

impl TryFrom<super::Message> for Message {
    type Error = String;

    fn try_from(value: super::Message) -> Result<Self, Self::Error> {
        if let super::Message::Custom(message) = value {
            Ok(message)
        } else {
            Err("".into())
        }
    }
}

impl From<Message> for super::Message {
    fn from(value: Message) -> Self {
        super::Message::Custom(value)
    }
}
