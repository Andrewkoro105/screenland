use crate::app::edit_object::{
    self,
    points_system::{PointsSystem, Reload},
    ui_point::{UIMessages, UIPoint},
    ui_utils::cube::{self, Cube},
};
use bytemuck::{Pod, Zeroable};
use glam::Vec2;
use iced::Task;

pub type Message = edit_object::ui_utils::cube::Message;

#[repr(C)]
#[derive(Copy, Clone, Debug, Pod, Zeroable, Default)]
pub struct Selection {
    pub cube: Cube,
}

impl Selection {
    pub fn normalize(&self) -> Self {
        Self {
            cube: self.cube.normalize(),
        }
    }

    pub fn add(&self, value: f32) -> Self {
        let mut result = *self;

        result.cube.start.x -= value;
        result.cube.start.y -= value;
        result.cube.end.x += value;
        result.cube.end.y += value;

        result
    }

    pub fn get_ui_point(&self) -> Vec<UIPoint> {
        <Cube as PointsSystem<cube::Message>>::view(&self.cube)
            .into_iter()
            .map(Into::into)
            .collect()
    }

    pub fn update(&mut self, mouse_pos: Vec2, message: Option<Message>) -> Reload<Task<Message>> {
        self.cube.update(&mouse_pos, message)
    }

    pub fn get_messages(&mut self, position: &Vec2) -> Option<Reload<UIMessages<Message>>> {
        let messages = self.cube.get_message(position);
        if messages.is_none() {
            self.cube.init = 0;
            self.cube.get_message(position)
        } else {
            messages
        }
    }

    pub fn get_data(&self) -> Vec<u8> {
        <Cube as PointsSystem<cube::Message>>::get_data(&self.cube)
            .into_iter()
            .map(|(_, a)| a)
            .collect::<Vec<_>>()
            .concat()
            .concat()
    }
}
