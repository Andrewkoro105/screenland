use bytemuck::{Pod, Zeroable};
use glam::Vec2;

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
        let mut result = self.clone();

        result.start.x -= value;
        result.start.y -= value;
        result.end.x += value;
        result.end.y += value;

        result
    }
}
