//! Buffer control system

use bytemuck::NoUninit;
use heck::ToPascalCase;
use iced::wgpu;

/// Static buffer data
#[derive(Clone)]
pub struct BaseStorageBufferData {
    /// Size of type in buffer
    type_size: usize,
    /// The size to which the passed structure is resized (for basic types, this is their size) divided by 4
    alignment_size: usize,
    /// Buffer structure name
    name: String,
    /// Type name in the buffer
    type_name: String,
}

/// Holds the `wgpu::Buffer` and the data needed to configure and use it
pub struct BaseStorageBuffer {
    /// WGPU buffer
    buff: wgpu::Buffer,
    /// The buffer number offset from the starting number
    offset_binding: u32,
    /// The number of elements in the buffer
    len: u32,
    /// Static buffer data
    data: BaseStorageBufferData,
}

impl BaseStorageBufferData {
    /// Creates a new `BaseStorageBufferData`
    ///
    /// # Arguments
    ///
    /// `type_size` - size of type in buffer
    ///
    /// `alignment_size` - the size to which the passed structure is resized (for basic types, this is their size) divided by 4
    ///
    /// `name` - buffer structure name
    ///
    /// `type_name` - type name in the buffer
    pub fn new(
        type_size: usize,
        alignment_size: usize,
        name: impl Into<String>,
        type_name: impl Into<String>,
    ) -> Self {
        Self {
            type_size,
            alignment_size,
            name: name.into(),
            type_name: type_name.into(),
        }
    }
}

impl BaseStorageBuffer {
    /// Creates a new `BaseStorageBuffer`
    ///
    /// # Arguments
    ///
    /// `device` - WGPU device for buffer initialization
    ///
    /// `data` - static buffer data
    ///
    /// `offset_binding` - static buffer data
    pub fn new(device: &wgpu::Device, data: BaseStorageBufferData, offset_binding: u32) -> Self {
        let buff = Self::get_buffer(device, 0, &data.name, data.type_size, data.alignment_size);
        Self {
            data,
            offset_binding,
            len: 0,
            buff,
        }
    }

    pub fn get_bind_group_layout_entry(&self, base_binding: u32) -> wgpu::BindGroupLayoutEntry {
        wgpu::BindGroupLayoutEntry {
            binding: base_binding + self.offset_binding,
            visibility: wgpu::ShaderStages::FRAGMENT,
            ty: wgpu::BindingType::Buffer {
                ty: wgpu::BufferBindingType::Storage { read_only: true },
                has_dynamic_offset: false,
                min_binding_size: None,
            },
            count: None,
        }
    }

    pub fn get_bind_group_entry(&self, base_binding: u32) -> wgpu::BindGroupEntry<'_> {
        wgpu::BindGroupEntry {
            binding: base_binding + self.offset_binding,
            resource: self.buff.as_entire_binding(),
        }
    }

    pub fn write<A: NoUninit>(&self, queue: &wgpu::Queue, data: &[A]) {
        let new_data = [
            bytemuck::bytes_of(&self.len),
            vec![0; (self.data.alignment_size - 1) * std::mem::size_of::<u32>()].as_slice(),
            bytemuck::cast_slice(data),
        ]
        .concat();

        queue.write_buffer(&self.buff, 0, new_data.as_slice());
    }

    pub fn resize(&mut self, device: &wgpu::Device, len: u32) -> bool {
        if len != self.len {
            self.len = len;
            self.buff = Self::get_buffer(
                device,
                len,
                &self.data.name,
                self.data.type_size,
                self.data.alignment_size,
            );

            true
        } else {
            false
        }
    }

    pub fn get_offset_binding(&self) -> u32 {
        self.offset_binding
    }

    pub fn get_data(&self) -> &BaseStorageBufferData {
        &self.data
    }

    /// Returns a buffer with default settings
    ///
    /// # Arguments
    ///
    /// `device` - WGPU device for buffer initialization
    ///
    /// `len` - The number of elements in the buffer
    ///
    /// `name` - Unique buffer name
    ///
    /// `type_size` - size of type in buffer
    ///
    /// `alignment_size` - the size to which the passed structure is resized (for basic types, this is their size) divided by 4
    ///
    fn get_buffer(
        device: &wgpu::Device,
        len: u32,
        name: &String,
        type_size: usize,
        alignment_size: usize,
    ) -> wgpu::Buffer {
        let size = Self::get_size(len, type_size, alignment_size);
        device.create_buffer(&wgpu::BufferDescriptor {
            label: Some(name),
            size,
            usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        })
    }

    /// Calculates the buffer size
    ///
    /// # Arguments
    ///
    /// `len` - The number of elements in the buffer
    ///
    /// `type_size` - size of type in buffer
    ///
    /// `alignment_size` - the size to which the passed structure is resized (for basic types, this is their size) divided by 4
    fn get_size(len: u32, type_size: usize, alignment_size: usize) -> u64 {
        let len = if len == 0 { 1 } else { len } as usize;
        (std::mem::size_of::<u32>() * alignment_size + len * type_size) as _
    }
}

impl BaseStorageBufferData {
    pub(super) fn get_wgsl_type(&self) -> String {
        let pascal_case_name = self.name.to_pascal_case();
        let type_name = &self.type_name;
        format!(
            r"
struct {pascal_case_name} {{
    len: u32,
    data: array<{type_name}>,
}}
        "
        )
    }

    pub(super) fn get_wgsl_var(&self, group: u32, binding: u32) -> String {
        let snake_case_name = &self.name;
        let pascal_case_name = self.name.to_pascal_case();
        format!(
            r"
@group({group}) @binding({binding})
var<storage> {snake_case_name}: {pascal_case_name};
        "
        )
    }
}
