pub mod chanel;

use std::{collections::HashMap, hash::Hash};

use iced::widget::{row, text};
use iced_helper::ui_elements::num_input::{NumInput, base_value::ConstF32};
use serde::{Deserialize, Serialize};

use crate::app::edit_object::custom_object::Message;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ShaderType {
    F32 {
        num_input: NumInput<f32, ConstF32<0>>,
    },
}

impl Hash for ShaderType {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        match self {
            ShaderType::F32 { .. } => 0.hash(state),
        }
    }
}

impl PartialEq for ShaderType {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::F32 { .. }, Self::F32 { .. }) => true,
            _ => false,
        }
    }
}

impl Eq for ShaderType {}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Param {
    name: String,
    pub shader_type: ShaderType,
}

impl Param {
    pub fn new(name: String, shader_type: ShaderType) -> Self {
        Self { name, shader_type }
    }

    pub fn get_menu(&self, i: usize) -> iced::Element<'_, Message> {
        match &self.shader_type {
            ShaderType::F32 { num_input } => row![
                text!("{}: ", self.name),
                num_input.view("", move |str| Message::SetF32(i, str))
            ]
            .into(),
        }
    }

    pub fn get_name(&self) -> String {
        self.name.clone()
    }

    pub fn get_str_field(&self) -> String {
        format!("{}: {},", self.name, self.shader_type.get_type_name())
    }

    pub fn get_str_init_field(&self, i: usize) -> String {
        let shader_type_name = 
            self.shader_type.get_type_name();
        format!(
            "{shader_type_name}_channel.{shader_type_name}_channel[channel_index.{shader_type_name}_index + {i}],",
        )
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

impl ShaderType {
    pub fn get_type_name(&self) -> String {
        match self {
            ShaderType::F32 { .. } => "f32".into(),
        }
    }
}
