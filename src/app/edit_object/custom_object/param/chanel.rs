use bytemuck::{Pod, Zeroable};

use crate::app::edit_object::ui_utils::cube::Cube;

#[derive(Clone, Debug)]
pub enum Message {
    Cube(Cube),
    F32(f32),
}

#[derive(Clone, Debug)]
pub enum AddMessage {
    Cube(Vec<Cube>),
    F32(Vec<f32>),
}

#[derive(Hash, PartialEq)]
pub enum ChanelType {
    Cube,
    F32,
}

impl Eq for ChanelType {}

#[repr(C)]
#[derive(Copy, Clone, Debug, Pod, Zeroable, Default)]
pub struct ChannelIndex {
    pub cube: u32,
    pub f32: u32,
}

#[derive(Debug, Clone, Default)]
pub struct Chanel {
    cube: Vec<Cube>,
    f32: Vec<f32>,
}

impl Chanel {
    pub fn get_index(&self) -> ChannelIndex {
        ChannelIndex {
            cube: self.cube.len() as _,
            f32: self.f32.len() as _,
        }
    }

    pub fn add(&mut self, mut data: AddMessage) {
        match &mut data {
            AddMessage::Cube(data) => self.cube.append(data),
            AddMessage::F32(data) => self.f32.append(data),
        }
    }

    pub fn get_f32(&self) -> &Vec<f32> {
        &self.f32
    }

    pub fn get_cube(&self) -> &Vec<Cube> {
        &self.cube
    }

    pub fn update(&mut self, message: Message, chanel_index: ChannelIndex, index: usize) {
        match message {
            Message::F32(value) => self.f32[chanel_index.f32 as usize + index] = value,

            Message::Cube(value) => self.cube[chanel_index.cube as usize + index] = value,
        }
    }
}