//! Enhanced interaction with storage buffers as data channels for custom objects

use std::{
    collections::HashMap,
    fmt::{Debug, Display},
};

use crate::app::{
    edit_object::ui_utils::cube::Cube,
    shader::pipeline::base_storage_buffers::base_storage_buffer::BaseStorageBufferData,
};
use glam::Vec2;
use heck::ToSnakeCase;
use strum::{Display, EnumCount, EnumIter, IntoEnumIterator};

/// Channel types, each of which corresponds to a `BaseStorageBufferData`
/// It also participates in shader generation so that when adding a channel, you don't have to specify its receiving part in the shader
#[derive(EnumIter, Display, EnumCount, Clone, Debug, Default, Hash, PartialEq)]
pub enum ChannelType {
    #[default]
    Cube,
    F32,
    U32,
    I32,
    BezierPointsLen,
    BezierPoints,
    Enum,
}

impl Eq for ChannelType {}

/// Indexes for all channels consolidated into a single structure
#[derive(Clone, Debug, Default)]
pub struct ChannelIndex {
    channels: HashMap<ChannelType, u32>,
}

/// A system for storing and managing all channels of type `ChannelType`.
/// It provides a simple interface for querying, adding, and updating data in a channel. It also allows you to retrieve the entire channel as a byte array for passing it to a shader.
#[derive(Clone, Default)]
pub struct Channels {
    null_data: [u8; 0],
    channels: HashMap<ChannelType, Vec<u8>>,
}

impl ChannelType {
    pub fn get_storage_buffers_data(&self) -> BaseStorageBufferData {
        match self {
            ChannelType::Cube => self.get_storage_buffer_data(2, "cube", "Cube"),
            ChannelType::F32 => self.get_storage_buffer_data(1, "f32", "f32"),
            ChannelType::U32 => self.get_storage_buffer_data(1, "u32", "u32"),
            ChannelType::I32 => self.get_storage_buffer_data(1, "i32", "i32"),
            ChannelType::BezierPointsLen => {
                self.get_storage_buffer_data(1, "bezier_points_len", "u32")
            }
            ChannelType::BezierPoints => {
                self.get_storage_buffer_data(2, "bezier_points", "vec2<f32>")
            }
            ChannelType::Enum => self.get_storage_buffer_data(1, "enum", "Enum"),
        }
    }

    fn get_storage_buffer_data(
        &self,
        alignment_size: usize,
        name: impl Display,
        type_name: impl Display,
    ) -> BaseStorageBufferData {
        BaseStorageBufferData::new(
            self.get_size(),
            alignment_size,
            format!("{name}_channel").as_str(),
            type_name.to_string(),
        )
    }

    pub fn get_size(&self) -> usize {
        match self {
            ChannelType::Cube => std::mem::size_of::<Cube>(),
            ChannelType::F32 => std::mem::size_of::<f32>(),
            ChannelType::U32 => std::mem::size_of::<u32>(),
            ChannelType::I32 => std::mem::size_of::<i32>(),
            ChannelType::BezierPointsLen => std::mem::size_of::<u32>(),
            ChannelType::BezierPoints => std::mem::size_of::<Vec2>(),
            ChannelType::Enum => std::mem::size_of::<u32>(),
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

    pub fn get_shader() -> String {
        [
            "struct ChannelIndex {".to_string(),
            ChannelType::iter()
                .map(|channel_type| {
                    format!("{}_index: u32,", channel_type.to_string().to_snake_case())
                })
                .collect::<Vec<_>>()
                .join("\n"),
            "}".to_string(),
        ]
        .join("\n")
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

impl Debug for Channels {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let result_str = ChannelType::iter()
            .map(|channel_type| {
                format!(
                    "{channel_type}: {:?}",
                    self.channels
                        .get(&channel_type)
                        .unwrap_or(&vec![])
                        .chunks(channel_type.get_size())
                        .collect::<Vec<&[_]>>()
                )
            })
            .collect::<Vec<_>>()
            .join("\n");

        write!(f, "{}", result_str)
    }
}
