use std::collections::HashMap;

use crate::app::{
    edit_object::ui_utils::cube::Cube,
    shader::pipeline::base_storage_buffers::base_storage_buffer::BaseStorageBufferData,
};
use strum::{EnumCount, EnumIter, IntoEnumIterator};

#[derive(EnumIter, EnumCount, Clone, Debug, Default, Hash, PartialEq)]
pub enum ChannelType {
    #[default]
    Cube,
    F32,
}

impl Eq for ChannelType {}

#[derive(Clone, Debug, Default)]
pub struct ChannelIndex {
    channels: HashMap<ChannelType, u32>,
}

#[derive(Debug, Clone, Default)]
pub struct Channels {
    null_data: [u8; 0],
    channels: HashMap<ChannelType, Vec<u8>>,
}

impl ChannelType {
    pub fn get_storage_buffers_data(&self) -> BaseStorageBufferData {
        match self {
            ChannelType::Cube => self.get_storage_buffer_data(2, "cube_channel", "Cube"),
            ChannelType::F32 => self.get_storage_buffer_data(1, "f32_channel", "f32"),
        }
    }

    fn get_storage_buffer_data(
        &self,
        len_padding: usize,
        name: &'static str,
        type_name: &'static str,
    ) -> BaseStorageBufferData {
        BaseStorageBufferData::new(self.get_size(), len_padding, name, type_name)
    }

    pub fn get_size(&self) -> usize {
        match self {
            ChannelType::Cube => std::mem::size_of::<Cube>(),
            ChannelType::F32 => std::mem::size_of::<f32>(),
        }
    }
}

impl ChannelIndex {
    pub fn to_bytes(&self) -> Vec<u8> {
        ChannelType::iter()
            .map(|channel_type| bytemuck::bytes_of(self.channels.get(&channel_type).unwrap_or(&0)))
            .flatten()
            .cloned()
            .collect()
    }
}

impl Channels {
    pub fn clear(&mut self) {
        self.channels.clear();
    }

    pub fn get_index(&self) -> ChannelIndex {
        ChannelIndex {
            channels: self
                .channels
                .iter()
                .map(|(channel_type, data)| {
                    (
                        channel_type.clone(),
                        (data.len() / channel_type.get_size()) as _,
                    )
                })
                .collect(),
        }
    }

    pub fn add(&mut self, channel_type: ChannelType, mut data: Vec<u8>) {
        if data.len() % channel_type.get_size() == 0 {
            if let Some(channel) = self.channels.get_mut(&channel_type) {
                channel.append(&mut data);
            } else {
                self.channels.insert(channel_type, data);
            }
        } else {
            panic!(
                "The data being transferred does not conform to the {:?} type because it is not a multiple of {} bytes.",
                channel_type,
                channel_type.get_size()
            )
        }
    }

    pub fn get(&self, channel_type: &ChannelType) -> &[u8] {
        self.channels
            .get(channel_type)
            .map(|data| data as _)
            .unwrap_or(&self.null_data)
    }

    pub fn update(
        &mut self,
        channel_type: &ChannelType,
        channel_index: &ChannelIndex,
        index: usize,
        data: Vec<u8>,
    ) {
        let i = (channel_index
            .channels
            .get(&channel_type)
            .cloned()
            .unwrap_or(0) as usize
            + index)
            * channel_type.get_size();
        let channel = self.channels.get_mut(channel_type).expect(&format!(
            "The {:?} channel does not yet exist.",
            channel_type
        ));
        channel.splice(i..(i + channel_type.get_size()), data);
    }
}
