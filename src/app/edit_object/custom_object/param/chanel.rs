use bytemuck::{Pod, Zeroable};

#[derive(Clone)]
pub enum Message {
    F32(f32)
}

#[derive(Hash, PartialEq)]
pub enum ChanelType {
    F32
}

impl Eq for ChanelType {}

#[repr(C)]
#[derive(Copy, Clone, Debug, Pod, Zeroable, Default)]
pub struct ChannelIndex {
    pub f32: u32,
}

#[derive(Debug, Clone, Default)]
pub struct Chanel {
    f32: Vec<f32>,
}

impl Chanel {
    pub fn get_index(&self) -> ChannelIndex {
        ChannelIndex {
            f32: self.f32.len() as _,
        }
    }

    pub fn add_f32(&mut self, mut data: Vec<f32>) {
        self.f32.append(&mut data);
    }

    fn set_f32(&mut self, chanel_index: ChannelIndex, index: usize, value: f32) {
        self.f32[chanel_index.f32 as usize + index] = value;
    }

    pub fn get_f32(&self) -> &Vec<f32> {
        &self.f32
    }

    pub fn update(&mut self, message: Message, chanel_index: ChannelIndex, index: usize) {
        match message {
            Message::F32(value) => self.set_f32(chanel_index, index, value),
        }
    }
}