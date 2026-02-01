use bytemuck::{Pod, Zeroable};
use glam::Vec2;
use iced::{Task};

use crate::app::edit_object::{self, EditObject, UIPoint, ui_utils::cube::{self, update}};

pub type Message = edit_object::ui_utils::cube::Message;

#[repr(C)]
#[derive(Copy, Clone, Debug, Pod, Zeroable, Default)]
pub struct Selection {
    pub start: Vec2,
    pub end: Vec2,
}

impl Selection {
    pub fn normalize(&self) -> Self {
        Self {
            start: Vec2 {
                x: self.start.x.min(self.end.x),
                y: self.start.y.min(self.end.y),
            },
            end: Vec2 {
                x: self.start.x.max(self.end.x),
                y: self.start.y.max(self.end.y),
            },
        }
    }

    pub fn add(&self, value: f32) -> Self {
        let mut result = *self;

        result.start.x -= value;
        result.start.y -= value;
        result.end.x += value;
        result.end.y += value;

        result
    }
}

impl EditObject<Message> for Selection {
    fn get_ui_point(&self) -> Vec<UIPoint> {
        let new_self = self.normalize();
        cube::view(&new_self.start, &new_self.end).into_iter().map(Into::into).collect()
    }

    fn update(
        &mut self,
        mouse_pos: Vec2,
        message: Message,
    ) -> Task<Message> {
        let mut new_self = self.normalize();

        let result = update(
            &mut new_self.start,
            &mut new_self.end,
            &mouse_pos,
            message
        ); 
        
        *self = new_self;

        result
    }
    
    fn get_messages(&self, position: &Vec2) -> Vec<Message> {
        cube::get_message(&self.start, &self.end, position)
    }
}
