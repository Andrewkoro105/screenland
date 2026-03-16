pub mod custom_object;
pub mod ui_point;
pub mod ui_utils;
use glam::Vec2;
use iced::{Element, Task};

use crate::app::{
    self, edit_object::ui_point::{UIPoint, UIPointElement}, settings::edit_object_base_settings::EditObjectBaseSettingsFromShader, update
};

#[derive(Clone)]
pub enum Message {
    Custom(custom_object::Message)
}

#[derive(Clone)]
pub enum CreateObjects {
    Custom(usize)
}

#[derive(Debug, Clone)]
pub enum Objects {
    Custom(custom_object::CustomObject)
}

#[derive(Debug, Clone)]
pub enum ShaderObjects {
    Custom(custom_object::CustomObjectFromShader)
}


pub trait EditObjectSettings {
    type Object;

    fn get_icon(&self) -> Element<'_, () >;

    fn get_name(&self) -> String;

    fn get_shader(&self) -> String;

    fn get_object(
        &self,
        i: usize,
        edit_object_base_settings: &EditObjectBaseSettingsFromShader,
    ) -> Self::Object;
}

pub trait EditObject<Message> {
    fn get_menu(&self) -> Option<Element<'_, Message>>;

    fn get_ui_point(&self) -> Vec<UIPoint>;

    fn get_messages(&self, position: &Vec2) -> Vec<Message>;

    fn update(&mut self, muse_position: Vec2, message: Message) -> Task<app::Message>;

    fn get_f32_data(&self) -> Vec<f32>;
}

impl<Message> From<UIPointElement<Message>> for UIPoint {
    fn from(value: UIPointElement<Message>) -> Self {
        value.point
    }
}

impl From<(usize, Message)> for update::Message {
    fn from(value: (usize, Message)) -> Self {
        Self::UpdateEditObject(value)
    }
}