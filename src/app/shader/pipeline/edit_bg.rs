use bytemuck::{Pod, Zeroable};
use glam::Vec2;
use iced::wgpu;
use strum::{EnumIter, IntoEnumIterator};

use crate::app::{
    edit_object::{
        custom_object::{CustomObjectFromShader, param::channel::ChannelType},
        ui_point::UIPoint,
    },
    selection::Selection,
    shader::pipeline::base_storage_buffers::{
        BaseStorageBuffers,
        base_storage_buffer::{BaseStorageBuffer, BaseStorageBufferData},
    },
};

#[repr(C)]
#[derive(Copy, Clone, Debug, Pod, Zeroable)]
pub struct BaseData {
    pub resolution: Vec2,
    pub monitor_pos: Vec2,
}

#[derive(Hash, PartialEq, EnumIter)]
pub enum BufferType {
    Points,
    CustomObjects,
    Channel(ChannelType),
}

impl Eq for BufferType {}

pub struct EditBGData {
    pub bgl: wgpu::BindGroupLayout,

    pub base_data_buffer: wgpu::Buffer,
    pub selection_buffer: wgpu::Buffer,

    pub storage_buffers: BaseStorageBuffers<BufferType, BaseStorageBuffer>,
}

pub struct EditBG {
    pub bg: wgpu::BindGroup,
    pub data: EditBGData,
}

impl BufferType {
    fn recursion_iter() -> impl Iterator<Item = BufferType> {
        Self::iter()
            .map(|buffer_type| {
                if let BufferType::Channel(_) = buffer_type {
                    ChannelType::iter().map(Self::Channel).collect()
                } else {
                    vec![buffer_type]
                }
            })
            .flatten()
    }

    fn get_storage_buffers_data(&self) -> BaseStorageBufferData {
        match self {
            BufferType::Points => {
                BaseStorageBufferData::new(std::mem::size_of::<UIPoint>(), 2, "points", "UIPoint")
            }
            BufferType::CustomObjects => BaseStorageBufferData::new(
                CustomObjectFromShader::get_size(),
                4,
                "custom_objects",
                "CustomObject",
            ),
            BufferType::Channel(channel_type) => channel_type.get_storage_buffers_data(),
        }
    }
}

pub fn get_storage_buffers_data() -> BaseStorageBuffers<BufferType, BaseStorageBufferData> {
    BaseStorageBuffers::new(
        1,
        2,
        BufferType::recursion_iter()
            .map(|buffer_type| {
                let storage_buffers_data = buffer_type.get_storage_buffers_data();
                (buffer_type, storage_buffers_data)
            })
            .collect(),
    )
}

impl EditBG {
    pub fn new(device: &wgpu::Device, _queue: &wgpu::Queue, _format: wgpu::TextureFormat) -> Self {
        let storage_buffers = get_storage_buffers_data().into_buff(device);

        let bgl = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("edit_bgl"),
            entries: &[
                &[
                    wgpu::BindGroupLayoutEntry {
                        binding: 0,
                        visibility: wgpu::ShaderStages::FRAGMENT,
                        ty: wgpu::BindingType::Buffer {
                            ty: wgpu::BufferBindingType::Uniform,
                            has_dynamic_offset: false,
                            min_binding_size: None,
                        },
                        count: None,
                    },
                    wgpu::BindGroupLayoutEntry {
                        binding: 1,
                        visibility: wgpu::ShaderStages::FRAGMENT,
                        ty: wgpu::BindingType::Buffer {
                            ty: wgpu::BufferBindingType::Uniform,
                            has_dynamic_offset: false,
                            min_binding_size: None,
                        },
                        count: None,
                    },
                ],
                storage_buffers.get_bind_group_layout_entry().as_slice(),
            ]
            .concat(),
        });

        let base_data_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("start_data_buffer"),
            size: std::mem::size_of::<BaseData>() as u64,
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        let selection_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("selection_buffer"),
            size: std::mem::size_of::<Selection>() as u64,
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        let data = EditBGData {
            bgl,

            base_data_buffer,
            selection_buffer,

            storage_buffers,
        };

        Self {
            bg: Self::create_bg(device, &data),
            data,
        }
    }

    pub fn reload_bg(&mut self, device: &wgpu::Device) {
        self.bg = Self::create_bg(device, &self.data);
    }

    fn create_bg(device: &wgpu::Device, edit_bg_data: &EditBGData) -> wgpu::BindGroup {
        device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("edit_bg"),
            layout: &edit_bg_data.bgl,
            entries: &[
                &[
                    wgpu::BindGroupEntry {
                        binding: 0,
                        resource: edit_bg_data.base_data_buffer.as_entire_binding(),
                    },
                    wgpu::BindGroupEntry {
                        binding: 1,
                        resource: edit_bg_data.selection_buffer.as_entire_binding(),
                    },
                ],
                edit_bg_data
                    .storage_buffers
                    .get_bind_group_entry()
                    .as_slice(),
            ]
            .concat(),
        })
    }
}
