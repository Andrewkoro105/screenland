use bytemuck::{Pod, Zeroable};
use glam::Vec2;
use iced::wgpu;

use crate::app::selection::Selection;

#[repr(C)]
#[derive(Copy, Clone, Debug, Pod, Zeroable)]
pub struct BaseData {
    pub resolution: Vec2,
    pub monitor_pos: Vec2,
}

pub struct EditBG {
    pub bg: wgpu::BindGroup,
    pub bgl: wgpu::BindGroupLayout,

    pub base_data_buffer: wgpu::Buffer,
    pub selection_buffer: wgpu::Buffer,
}

impl EditBG {
    pub fn new(device: &wgpu::Device, _queue: &wgpu::Queue, _format: wgpu::TextureFormat) -> Self {
        let bgl = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("edit_bgl"),
            entries: &[
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
        Self {
            bg: device.create_bind_group(&wgpu::BindGroupDescriptor {
                label: Some("edit_bg"),
                layout: &bgl,
                entries: &[
                    wgpu::BindGroupEntry {
                        binding: 0,
                        resource: base_data_buffer.as_entire_binding(),
                    },
                    wgpu::BindGroupEntry {
                        binding: 1,
                        resource: selection_buffer.as_entire_binding(),
                    },
                ],
            }),
            bgl,
            base_data_buffer,
            selection_buffer,
        }
    }
}