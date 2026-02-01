pub mod ui_utils;
use bytemuck::{Pod, Zeroable};
use glam::Vec2;
use iced::Task;

pub struct UIPointElement<Message> {
    pub point: UIPoint,
    pub message: Message,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Pod, Zeroable, Default)]
pub struct UIPoint {
    pub pos: Vec2,
    pub size: f32,
    _padding: f32,
}

impl UIPoint {
    pub fn new(pos: Vec2, size: f32) -> Self {
        Self {
            pos,
            size,
            _padding: 0.,
        }
    }

    pub fn in_point(&self, pos: &Vec2) -> bool{
        let r = ((self.pos.x - pos.x).powi(2) + (self.pos.y - pos.y).powi(2)).sqrt();
        r < self.size 
    }
}

pub trait EditObject<Message> {
    fn get_ui_point(&self) -> Vec<UIPoint>;

    fn get_messages(&self, position: &Vec2) -> Vec<Message>;

    fn update(&mut self, muse_position: Vec2, message: Message) -> Task<Message>;
}

impl<Message> From<UIPointElement<Message>> for UIPoint {
    fn from(value: UIPointElement<Message>) -> Self {
        value.point
    }
}