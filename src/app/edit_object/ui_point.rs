use bytemuck::{Pod, Zeroable};
use glam::Vec2;
use iced::Task;

use crate::app;

#[derive(Clone)]
pub struct UIMessages<M> {
    pub message: M,
    pub start_messages: Vec<M>,
}

#[derive(Clone)]
pub struct UIPointElement<M> {
    pub point: UIPoint,
    pub messages: UIMessages<M>,
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

    pub fn in_point(&self, pos: &Vec2) -> bool {
        let r = ((self.pos.x - pos.x).powi(2) + (self.pos.y - pos.y).powi(2)).sqrt();
        r < self.size
    }
}

impl<M: Clone> UIPointElement<M> {
    pub fn into_ui_point_element<M2: From<M> + Clone>(self) -> UIPointElement<M2> {
        UIPointElement {
            point: self.point,
            messages: self.messages.map(Into::into),
        }
    }
}

impl<M> UIMessages<M> {
    pub fn from_message(message: M) -> Self {
        Self { message, start_messages: vec![] }
    }
    
    pub fn map<M2>(self, into: impl Fn(M) -> M2) -> UIMessages<M2> {
        UIMessages {
            message: into(self.message),
            start_messages: self.start_messages.into_iter().map(into).collect(),
        }
    }
}



impl UIMessages<app::Message> {
    pub fn get_task(self, base_massage_into: impl Fn(app::Message) -> app::Message) -> Task<app::Message> {
        Task::batch(self.start_messages.into_iter().map(Task::done)).chain(Task::done(base_massage_into(self.message)))
    }
}
