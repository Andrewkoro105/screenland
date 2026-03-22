use crate::app::edit_object::{self, ui_point::UIPoint, ui_utils::cube};
use bytemuck::{Pod, Zeroable};
use glam::Vec2;
use iced::Task;

pub type Message = edit_object::ui_utils::cube::Message;

#[repr(C)]
#[derive(Copy, Clone, Debug, Pod, Zeroable, Default)]
pub struct Selection {
    pub start: Vec2,
    pub end: Vec2,
}

impl Selection {
    pub fn normalize(&self) -> Self {
        let mut result = *self;
        cube::normalize(&mut result.start, &mut result.end);
        result
    }

    pub fn add(&self, value: f32) -> Self {
        let mut result = *self;

        result.start.x -= value;
        result.start.y -= value;
        result.end.x += value;
        result.end.y += value;

        result
    }
    
    pub fn get_ui_point(&self) -> Vec<UIPoint> {
        let new_self = self.normalize();
        cube::view(&new_self.start, &new_self.end)
            .into_iter()
            .map(Into::into)
            .collect()
    }

    pub fn update(&mut self, mouse_pos: Vec2, message: Message) -> Task<Message> {
        cube::update(&mut self.start, &mut self.end, &mouse_pos, message)
    }

    pub fn get_messages(&self, position: &Vec2) -> Vec<Message> {
        cube::get_message(&self.start, &self.end, position)
    }
}
