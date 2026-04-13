pub mod channel;

use std::{collections::HashMap, hash::Hash, mem};

use iced::{
    Theme,
    widget::{Row, button},
};
use iced_helper::ui_elements::{
    ParamSettings,
    num_input::{
        NumInput,
        base_value::{ConstF32, ConstI32, ConstU32},
    },
};
use serde::{Deserialize, Serialize};

use crate::app::edit_object::custom_object::{
    data_type::DataType, icon::Icon, param::channel::ChannelType,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ShaderType {
    F32 {
        num_input: NumInput<f32, ConstF32<0>>,
    },
    U32 {
        num_input: NumInput<u32, ConstU32<0>>,
    },
    I32 {
        num_input: NumInput<i32, ConstI32<0>>,
    },
    Enum {
        #[serde(skip)]
        #[serde(default)]
        current: u32,
        enums: Vec<(String, Icon)>,
    },
}

#[derive(Clone, Debug)]
pub enum Message {
    SetF32(String),
    SetU32(String),
    SetI32(String),
    SetEnum(u32),
}

#[derive(Debug, Clone, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub struct Param {
    name: String,
    pub shader_type: ShaderType,
}

impl Param {
    pub fn new(name: String, shader_type: ShaderType) -> Self {
        Self { name, shader_type }
    }

    pub fn get_menu(&self) -> iced::Element<'_, Message> {
        let param_settings = ParamSettings { name_size: 100 };
        param_settings
            .create_param(
                format!("{}: ", self.name),
                match &self.shader_type {
                    ShaderType::F32 { num_input } => {
                        num_input.view("", move |str| Message::SetF32(str))
                    }
                    ShaderType::U32 { num_input } => {
                        num_input.view("", move |str| Message::SetU32(str))
                    }
                    ShaderType::I32 { num_input } => {
                        num_input.view("", move |str| Message::SetI32(str))
                    }
                    ShaderType::Enum { current, enums } => {
                        Row::from_iter(enums.iter().enumerate().map(|(i, (_, icon))| {
                            button(icon.get_icon())
                                .on_press(Message::SetEnum(i as _))
                                .style(move |theme, _| {
                                    button::Catalog::style(
                                        theme,
                                        &<Theme as button::Catalog>::default(),
                                        if *current == i as u32 {
                                            button::Status::Active
                                        } else {
                                            button::Status::Disabled
                                        },
                                    )
                                })
                                .into()
                        }))
                        .into()
                    }
                },
            )
            .into()
    }

    pub fn indexing_params(params: &Vec<Self>) -> Vec<(usize, Self)> {
        let mut params_map: HashMap<_, Vec<_>> = HashMap::new();
        let mut result = vec![];
        for param in params {
            if let Some(buf) = params_map.get_mut(&param.shader_type) {
                result.push((buf.len(), param.clone()));
                buf.push(param);
            } else {
                params_map.insert(&param.shader_type, vec![param]);
                result.push((0, param.clone()));
            }
        }
        result
    }

    pub fn get_supporting_system(&self, object_name: &str) -> String {
        match &self.shader_type {
            ShaderType::Enum { enums, .. } => enums
                .iter()
                .enumerate()
                .map(|(i, (name, _))| format!("const {object_name}_{name} = {i};"))
                .collect::<Vec<_>>()
                .join("\n"),
            _ => "".to_string(),
        }
    }
}

impl DataType for Param {
    fn get_name(&self) -> String {
        self.name.clone()
    }

    fn base_get_type_name(&self) -> String {
        match self.shader_type {
            ShaderType::F32 { .. } => "f32".into(),
            ShaderType::U32 { .. } => "u32".into(),
            ShaderType::I32 { .. } => "i32".into(),
            ShaderType::Enum { .. } => "Enum".into(),
        }
    }
}

impl ShaderType {
    pub fn get_data(&self) -> Vec<u8> {
        match self {
            ShaderType::F32 { num_input } => bytemuck::bytes_of(&num_input.get()).to_vec(),
            ShaderType::U32 { num_input } => bytemuck::bytes_of(&num_input.get()).to_vec(),
            ShaderType::I32 { num_input } => bytemuck::bytes_of(&num_input.get()).to_vec(),
            ShaderType::Enum { current, .. } => bytemuck::bytes_of(current).to_vec(),
        }
    }

    pub fn update(&mut self, message: Message) {
        match self {
            ShaderType::F32 { num_input } => {
                num_input.update(
                    if let Message::SetF32(value) = message {
                        value
                    } else {
                        unreachable!("The `Message::SetF32` call is not for `ShaderType::F32`")
                    }
                    .as_str(),
                );
            }
            ShaderType::U32 { num_input } => {
                num_input.update(
                    if let Message::SetU32(value) = message {
                        value
                    } else {
                        unreachable!("The `Message::SetU32` call is not for `ShaderType::U32`")
                    }
                    .as_str(),
                );
            }
            ShaderType::I32 { num_input } => {
                num_input.update(
                    if let Message::SetI32(value) = message {
                        value
                    } else {
                        unreachable!("The `Message::SetI32` call is not for `ShaderType::I32`")
                    }
                    .as_str(),
                );
            }
            ShaderType::Enum { current, .. } => {
                if let Message::SetEnum(value) = message {
                    *current = value
                } else {
                    unreachable!("The `Message::SetEnum` call is not for `ShaderType::Enum`")
                }
            }
        }
    }
}

impl Hash for ShaderType {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        mem::discriminant(self).hash(state)
    }
}

impl PartialEq for ShaderType {
    fn eq(&self, other: &Self) -> bool {
        mem::discriminant(self) == mem::discriminant(other)
    }
}

impl Eq for ShaderType {}

impl From<ShaderType> for ChannelType {
    fn from(value: ShaderType) -> Self {
        match value {
            ShaderType::F32 { .. } => ChannelType::F32,
            ShaderType::U32 { .. } => ChannelType::U32,
            ShaderType::I32 { .. } => ChannelType::I32,
            ShaderType::Enum { .. } => ChannelType::Enum,
        }
    }
}
