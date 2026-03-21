pub mod icon;
pub mod param;
pub mod points;
use std::ops::Not;

use bytemuck::{Pod, Zeroable};
use iced::{Task, widget::Column};
use serde::{Deserialize, Serialize, Serializer};

use crate::app::{
    self,
    edit_object::{
        self, EditObject, EditObjectSettings,
        custom_object::{
            icon::Icon,
            param::{
                Param, ShaderType,
                chanel::{self, Chanel, ChannelIndex},
            },
            points::{PointsData, PointsFormat, PointsMessage},
        },
        ui_point::UIPoint,
    },
    settings::edit_object_base_settings::EditObjectBaseSettingsFromShader,
};

#[derive(Clone)]
pub enum Message {
    SetF32(usize, String),
    Point(PointsMessage),
}

#[derive(Clone, Serialize, Deserialize)]
pub struct CustomObjectSettings {
    name: String,
    icon: Icon,
    params: Vec<Param>,
    shader: String,
    points_format: Option<PointsFormat>,
}

#[derive(Clone)]
pub struct CustomIndexedObjectSettings {
    type_id: u32,
    name: String,
    icon: Icon,
    params: Vec<Param>,
    shader: String,
    points_format: Option<PointsFormat>,
}

#[derive(Debug, Clone)]
pub struct CustomObject {
    type_id: u32,
    i: usize,
    edit_object_base_settings: EditObjectBaseSettingsFromShader,
    points_data: Option<PointsData>,
    params: Vec<Param>,
}
#[repr(C)]
#[derive(Copy, Clone, Debug, Pod, Zeroable, Default)]
pub struct CustomObjectFromShader {
    pub custom_object_type: u32,
    pub channel_index: ChannelIndex,
}

impl CustomObjectSettings {
    pub fn new(
        name: String,
        icon: Icon,
        params: Vec<Param>,
        shader: String,
        points_format: Option<PointsFormat>,
    ) -> Self {
        Self {
            name,
            icon,
            params,
            shader,
            points_format,
        }
    }
}

impl CustomIndexedObjectSettings {
    pub fn new(
        type_id: u32,
        name: String,
        icon: Icon,
        params: Vec<Param>,
        shader: String,
        points_format: Option<PointsFormat>,
    ) -> Self {
        Self {
            type_id,
            name,
            icon,
            params,
            shader,
            points_format,
        }
    }
}

pub fn add_type_id(value: Vec<CustomObjectSettings>) -> Vec<CustomIndexedObjectSettings> {
    value
        .into_iter()
        .enumerate()
        .map(|(i, object)| {
            CustomIndexedObjectSettings::new(
                i as _,
                object.name,
                object.icon,
                object.params,
                object.shader,
                object.points_format,
            )
        })
        .collect()
}

pub fn add_type_id_deserialize<'de, D>(
    deserializer: D,
) -> Result<Vec<CustomIndexedObjectSettings>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let intermediate = Vec::<CustomObjectSettings>::deserialize(deserializer)?;
    Ok(add_type_id(intermediate))
}

pub fn remove_type_id(value: Vec<CustomIndexedObjectSettings>) -> Vec<CustomObjectSettings> {
    value
        .into_iter()
        .map(|object| {
            CustomObjectSettings::new(
                object.name,
                object.icon,
                object.params,
                object.shader,
                object.points_format,
            )
        })
        .collect()
}

pub fn remove_type_id_serialize<S>(
    value: &Vec<CustomIndexedObjectSettings>,
    serializer: S,
) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    remove_type_id(value.clone()).serialize(serializer)
}

impl EditObjectSettings for CustomIndexedObjectSettings {
    type Object = CustomObject;

    fn get_icon(&self) -> iced::Element<'_, ()> {
        self.icon.get_icon()
    }

    fn get_name(&self) -> String {
        self.name.clone()
    }

    fn get_shader(&self) -> String {
        let name = &self.name;
        let shader = &self.shader;
        let params = self
            .params
            .iter()
            .map(Param::get_str_field)
            .collect::<Vec<_>>()
            .join("\n\t");
        let init_params = Param::indexing_params(&self.params)
            .iter()
            .map(|(i, param)| param.get_str_init_field(*i))
            .collect::<Vec<_>>()
            .join("\n\t");

        format!(
            r"
struct Data_{name} {{
    {params}
}}

fn get_data_{name}(channel_index: ChannelIndex) -> Data_{name} {{
    return Data_{name} (
        {init_params}
    );
}}
        
fn draw_{name}(pixel_color: vec4<f32>, pixel_pos: vec2<f32>, data: Data_{name}) -> vec4<f32> {{
{shader}
}}
"
        )
    }

    fn get_object(
        &self,
        i: usize,
        edit_object_base_settings: &EditObjectBaseSettingsFromShader,
    ) -> CustomObject {
        CustomObject {
            type_id: self.type_id,
            i,
            edit_object_base_settings: *edit_object_base_settings,
            points_data: self.points_format.clone().map(Into::into),
            params: self.params.clone(),
        }
    }
}

impl EditObject for CustomObject {
    fn get_menu(&self) -> Option<iced::Element<'_, app::Message>> {
        self.params.is_empty().not().then_some(
            Column::from_iter(self.params.iter().enumerate().map(|(i, param)| {
                Param::get_menu(param, i).map(|message| {
                    app::Message::UpdateEditObject((self.i, edit_object::Message::Custom(message)))
                })
            }))
            .spacing(5)
            .into(),
        )
    }

    fn get_ui_point(&self) -> Vec<UIPoint> {
        self.points_data
            .as_ref()
            .map(PointsData::get_ui_point)
            .unwrap_or(vec![])
    }

    fn get_messages(&self, position: &glam::Vec2) -> Vec<app::Message> {
        self.points_data
            .as_ref()
            .map(|points_data| {
                points_data
                    .get_messages(position)
                    .into_iter()
                    .map(Message::Point)
                    .map(|message| {
                        app::Message::UpdateEditObject((
                            self.i,
                            edit_object::Message::Custom(message),
                        ))
                    })
                    .collect()
            })
            .unwrap_or(vec![])
    }

    fn update(
        &mut self,
        muse_position: glam::Vec2,
        message: edit_object::Message,
    ) -> iced::Task<app::Message> {
        if let edit_object::Message::Custom(message) = message {
            match message {
                Message::SetF32(index, value) => {
                    if let ShaderType::F32 { num_input } = &mut self.params[index].shader_type {
                        Task::done(app::Message::CustomObjectsChenelUpdate {
                            i: self.i,
                            index,
                            message: chanel::Message::F32(num_input.update(&value)),
                        })
                    } else {
                        panic!("")
                    }
                }
                Message::Point(points_message) => {
                    let i = self.i;
                    self.points_data
                        .as_mut()
                        .map(move |points_data| {
                            points_data
                                .update(muse_position, points_message)
                                .map(Message::Point)
                                .map(move |message| {
                                    app::Message::UpdateEditObject((
                                        i.clone(),
                                        edit_object::Message::Custom(message),
                                    ))
                                })
                        })
                        .unwrap_or(Task::none())
                }
            }
        } else {
            panic!(
                "In CustomObject({}), a message for the wrong object was sent.",
                self.i
            )
        }
    }

    fn get_f32_data(&self) -> Vec<f32> {
        self.params
            .iter()
            .filter_map(|param| {
                if let ShaderType::F32 { num_input } = &param.shader_type {
                    Some(num_input.get())
                } else {
                    None
                }
            })
            .collect()
    }

    fn get_shader_object(&self, chanel: &mut Chanel) -> edit_object::ShaderObjects {
        let result = edit_object::ShaderObjects::Custom(CustomObjectFromShader {
            channel_index: chanel.get_index(),
            custom_object_type: self.get_type_id(),
        });

        chanel.add_f32(self.get_f32_data());
        result
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
