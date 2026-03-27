use bytemuck::{Pod, Zeroable};
use strum_macros::EnumIter;

use crate::app::{
    edit_object::ui_utils::cube::Cube,
    shader::pipeline::base_storage_buffers::base_storage_buffer::BaseStorageBufferData,
};

#[derive(EnumIter, Default, Hash, PartialEq)]
pub enum ChannelType {
    #[default]
    Cube,
    F32,
}

// trait Channel {
//     const CHANEL_TYPE: ChannelType;
// }

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

impl Eq for ChannelType {}

#[repr(C)]
#[derive(Copy, Clone, Debug, Pod, Zeroable, Default)]
pub struct ChannelIndex {
    pub cube: u32,
    pub f32: u32,
}

#[derive(Debug, Clone, Default)]
pub struct Channels {
    cube: Vec<Cube>,
    f32: Vec<f32>,
}

impl ChannelType {
    pub fn get_storage_buffers_data(&self) -> BaseStorageBufferData {
        match self {
            ChannelType::Cube => {
                BaseStorageBufferData::new(std::mem::size_of::<Cube>(), 2, "cube_channel", "Cube")
            }
            ChannelType::F32 => {
                BaseStorageBufferData::new(std::mem::size_of::<f32>(), 1, "f32_channel", "f32")
            }
        }
    }
}

impl Channels {
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

    pub fn update(&mut self, message: Message, channel_index: ChannelIndex, index: usize) {
        match message {
            Message::F32(value) => self.f32[channel_index.f32 as usize + index] = value,

            Message::Cube(value) => self.cube[channel_index.cube as usize + index] = value,
        }
    }
}
