pub mod channel;

use std::{collections::HashMap, hash::Hash, mem};

use iced_helper::ui_elements::{
    ParamSettings,
    num_input::{
        NumInput,
        base_value::{ConstF32, ConstU32},
    },
};
use serde::{Deserialize, Serialize};

use crate::app::edit_object::custom_object::{
    data_type::DataType, param::channel::ChannelType
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ShaderType {
    F32 {
        num_input: NumInput<f32, ConstF32<0>>,
    },
    U32 {
        num_input: NumInput<u32, ConstU32<0>>,
    },
}

#[derive(Clone, Debug)]
pub enum Message {
    SetF32(String),
    SetU32(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Param {
    name: String,
    pub shader_type: ShaderType,
}

impl Param {
    pub fn new(name: String, shader_type: ShaderType) -> Self {
        Self { name, shader_type }
    }

    pub fn get_menu(&self) -> iced::Element<'_, Message> {
        let param_settings = ParamSettings { name_size: 60 };
        match &self.shader_type {
            ShaderType::F32 { num_input } => param_settings.create_param(
                format!("{}: ", self.name),
                num_input.view("", move |str| Message::SetF32(str)),
            ),
            ShaderType::U32 { num_input } => param_settings.create_param(
                format!("{}: ", self.name),
                num_input.view("", move |str| Message::SetU32(str)),
            ),
        }
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
}

impl DataType for Param {
    fn get_name(&self) -> String {
        self.name.clone()
    }

    fn get_type_name(&self) -> String {
        match self.shader_type {
            ShaderType::F32 { .. } => "f32".into(),
            ShaderType::U32 { .. } => "u32".into(),
        }
    }
}

impl ShaderType {
    pub fn get_data(&self) -> Vec<u8> {
        match self {
            ShaderType::F32 { num_input } => bytemuck::bytes_of(&num_input.get()).to_vec(),
            ShaderType::U32 { num_input } => bytemuck::bytes_of(&num_input.get()).to_vec(),
        }
    }

    pub fn update(&mut self, message: Message) {
        match self {
            ShaderType::F32 { num_input } => {
                num_input.update(
                    if let Message::SetF32(value) = message {
                        value
                    } else {
                        panic!("The `Message::SetF32` call is not for `ShaderType::F32`")
                    }
                    .as_str(),
                );
            }
            ShaderType::U32 { num_input } => {
                num_input.update(
                    if let Message::SetU32(value) = message {
                        value
                    } else {
                        panic!("The `Message::SetU32` call is not for `ShaderType::U32`")
                    }
                    .as_str(),
                );
            },
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
        }
    }
}
