use bytemuck::{Pod, Zeroable};
use glam::Vec2;
use iced::wgpu;

use crate::app::{
    edit_object::{custom_object::CustomObjectFromShader, ui_point::UIPoint},
    selection::Selection,
};

#[repr(C)]
#[derive(Copy, Clone, Debug, Pod, Zeroable)]
pub struct BaseData {
    pub resolution: Vec2,
    pub monitor_pos: Vec2,
}

pub struct EditBGData {
    pub bgl: wgpu::BindGroupLayout,

    pub base_data_buffer: wgpu::Buffer,
    pub selection_buffer: wgpu::Buffer,

    pub point_buffer_size: usize,
    pub point_buffer: wgpu::Buffer,

    pub f32_channel_size: usize,
    pub f32_channel: wgpu::Buffer,

    pub custom_objects_size: usize,
    pub custom_objects: wgpu::Buffer,
}

pub struct EditBG {
    pub bg: wgpu::BindGroup,
    pub data: EditBGData,
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
                wgpu::BindGroupLayoutEntry {
                    binding: 2,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Storage { read_only: true },
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 3,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Storage { read_only: true },
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 4,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Storage { read_only: true },
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

        let point_buffer = Self::get_points_buffer(device, 0);

        let custom_objects = Self::get_custom_objects_buffer(device, 0);

        let f32_channel = Self::get_f32_channel_buffer(device, 0);

        let data = EditBGData {
            bgl,

            base_data_buffer,
            selection_buffer,

            point_buffer_size: 0,
            point_buffer,

            custom_objects_size: 0,
            custom_objects,

            f32_channel_size: 0,
            f32_channel,
        };

        Self {
            bg: Self::create_bg(device, &data),
            data,
        }
    }

    fn get_vec_buff_size(mut len: usize, size: usize, len_padding: usize) -> u64 {
        if len == 0 {
            len = 1
        }
        (std::mem::size_of::<u32>() * len_padding + len * size) as _
    }

    pub fn reload_bg(&mut self, device: &wgpu::Device) {
        self.bg = Self::create_bg(device, &self.data);
    }

    pub fn set_points_buffer(&mut self, device: &wgpu::Device, size: usize) -> bool {
        if size != self.data.point_buffer_size {
            self.data.point_buffer_size = size;
            self.data.point_buffer = Self::get_points_buffer(device, size);

            true
        } else {
            false
        }
    }

    fn get_points_buffer(device: &wgpu::Device, size: usize) -> wgpu::Buffer {
        device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("point_buffer"),
            size: Self::get_vec_buff_size(size, std::mem::size_of::<UIPoint>(), 2),
            usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        })
    }

    pub fn set_custom_objects_buffer(&mut self, device: &wgpu::Device, size: usize) -> bool {
        if size != self.data.custom_objects_size {
            self.data.custom_objects_size = size;
            self.data.custom_objects = Self::get_custom_objects_buffer(device, size);

            true
        } else {
            false
        }
    }

    fn get_custom_objects_buffer(device: &wgpu::Device, size: usize) -> wgpu::Buffer {
        let len = Self::get_vec_buff_size(size, std::mem::size_of::<CustomObjectFromShader>(), 2);
        device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("custom_objects"),
            size: len,
            usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        })
    }

    pub fn set_f32_channel_buffer(&mut self, device: &wgpu::Device, size: usize) -> bool {
        if size != self.data.f32_channel_size {
            self.data.f32_channel_size = size;
            self.data.f32_channel = Self::get_f32_channel_buffer(device, size);

            true
        } else {
            false
        }
    }

    fn get_f32_channel_buffer(device: &wgpu::Device, size: usize) -> wgpu::Buffer {
        let len = Self::get_vec_buff_size(size, std::mem::size_of::<f32>(), 1);
        device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("f32_channel"),
            size: len,
            usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        })
    }

    fn create_bg(device: &wgpu::Device, edit_bg_data: &EditBGData) -> wgpu::BindGroup {
        device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("edit_bg"),
            layout: &edit_bg_data.bgl,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: edit_bg_data.base_data_buffer.as_entire_binding(),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: edit_bg_data.selection_buffer.as_entire_binding(),
                },
                wgpu::BindGroupEntry {
                    binding: 2,
                    resource: edit_bg_data.point_buffer.as_entire_binding(),
                },
                wgpu::BindGroupEntry {
                    binding: 3,
                    resource: edit_bg_data.custom_objects.as_entire_binding(),
                },
                wgpu::BindGroupEntry {
                    binding: 4,
                    resource: edit_bg_data.f32_channel.as_entire_binding(),
                },
            ],
        })
    }

    pub fn write_points_buffer(&self, queue: &wgpu::Queue, ui_points: &Vec<UIPoint>) {
        queue.write_buffer(
            &self.data.point_buffer,
            0,
            [
                bytemuck::bytes_of(&(ui_points.len() as u32)),
                &[0, 0, 0, 0],
                bytemuck::cast_slice(ui_points),
            ]
            .concat()
            .as_slice(),
        );
    }

    pub fn write_custom_objects_buffer(
        &self,
        queue: &wgpu::Queue,
        custom_objects: &Vec<CustomObjectFromShader>,
    ) {
        queue.write_buffer(
            &self.data.custom_objects,
            0,
            [
                bytemuck::bytes_of(&(custom_objects.len() as u32)),
                &[0, 0, 0, 0],
                bytemuck::cast_slice(custom_objects),
            ]
            .concat()
            .as_slice(),
        );
    }

    pub fn write_f32_channel_buffer(&self, queue: &wgpu::Queue, f32_channel: &Vec<f32>) {
        queue.write_buffer(
            &self.data.f32_channel,
            0,
            [
                bytemuck::bytes_of(&(f32_channel.len() as u32)),
                bytemuck::cast_slice(f32_channel),
            ]
            .concat()
            .as_slice(),
        );
    }
}
