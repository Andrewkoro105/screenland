use std::path::Path;

use crate::app::update::Message as AppMessage;
use bytemuck::{Pod, Zeroable};
use glam::Vec4;
use iced::{Color, Task};
use iced_helper::ui_elements::num_input::{NumInput, base_value::ConstF32};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug)]
pub enum Message {
    SetSize(String),
    SetColor(Color),
    SetColorR(String),
    SetColorG(String),
    SetColorB(String),
    SetColorA(String),
}

#[derive(Default, Clone, Serialize, Deserialize)]
pub struct ColorInput {
    pub r: NumInput<f32, ConstF32<0>>,
    pub g: NumInput<f32, ConstF32<0>>,
    pub b: NumInput<f32, ConstF32<0>>,
    pub a: NumInput<f32, ConstF32<1>>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct EditObjectBaseSettings {
    pub color: ColorInput,
    pub size: NumInput<f32, ConstF32<0>>,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Pod, Zeroable, Default)]
pub struct EditObjectBaseSettingsFromShader {
    pub color: Vec4,
    pub size: f32,
    _padding: [u8; 3 * 4],
}

impl Default for EditObjectBaseSettings {
    fn default() -> Self {
        Self {
            color: ColorInput {
                r: NumInput::new(1.),
                g: NumInput::new(0.),
                b: NumInput::new(0.),
                a: NumInput::new(1.),
            },
            size: NumInput::new(6.),
        }
    }
}

impl From<EditObjectBaseSettings> for EditObjectBaseSettingsFromShader {
    fn from(value: EditObjectBaseSettings) -> Self {
        Self {
            color: Vec4::new(
                value.color.r.get(),
                value.color.g.get(),
                value.color.b.get(),
                value.color.a.get(),
            ),
            size: value.size.get(),
            _padding: [0; _],
        }
    }
}

impl From<EditObjectBaseSettingsFromShader> for EditObjectBaseSettings {
    fn from(value: EditObjectBaseSettingsFromShader) -> Self {
        Self {
            color: ColorInput {
                r: NumInput::new(value.color.x),
                g: NumInput::new(value.color.y),
                b: NumInput::new(value.color.z),
                a: NumInput::new(value.color.w),
            },
            size: NumInput::new(value.size),
        }
    }
}

impl EditObjectBaseSettings {
    pub fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::SetSize(str) => {
                self.size.update(&str);
                Task::none()
            }
            Message::SetColor(color) => {
                self.color.r.set(color.r);
                self.color.g.set(color.g);
                self.color.b.set(color.b);
                self.color.a.set(color.a);
                Task::none()
            }
            Message::SetColorR(str) => {
                self.color.r.update(&str);
                Task::none()
            }
            Message::SetColorG(str) => {
                self.color.g.update(&str);
                Task::none()
            }
            Message::SetColorB(str) => {
                self.color.b.update(&str);
                Task::none()
            }
            Message::SetColorA(str) => {
                self.color.a.update(&str);
                Task::none()
            }
        }
    }

    pub(super) fn load(path: &Path) -> Option<Self> {
        super::base_load(path, "EditObjectBaseSettings")
    }

    pub(super) fn save(&self, path: &Path) {
        super::base_save(self, path, "EditObjectBaseSettings");
    }
}

impl From<Message> for AppMessage {
    fn from(message: Message) -> Self {
        Self::EditObjectBaseSettings(message)
    }
}
