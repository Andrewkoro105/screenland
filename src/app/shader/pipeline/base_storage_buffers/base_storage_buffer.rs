use bytemuck::NoUninit;
use heck::ToPascalCase;
use iced::wgpu;

#[derive(Clone)]
pub struct BaseStorageBufferData {
    type_size: usize,
    len_padding: usize,
    name: &'static str,
    type_name: &'static str,
}

pub struct BaseStorageBuffer {
    buff: wgpu::Buffer,
    offset_binding: u32,
    len: u32,
    data: BaseStorageBufferData,
}

impl BaseStorageBufferData {
    pub fn new(
        type_size: usize,
        len_padding: usize,
        name: &'static str,
        type_name: &'static str,
    ) -> Self {
        Self {
            type_size,
            len_padding,
            name,
            type_name,
        }
    }
}

impl BaseStorageBuffer {
    pub fn new(device: &wgpu::Device, data: BaseStorageBufferData, offset_binding: u32) -> Self {
        let buff = Self::get_buffer(device, 0, &data.name, data.type_size, data.len_padding);
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

    pub fn write_buffer<A: NoUninit>(&self, queue: &wgpu::Queue, data: &[A]) {
        let new_data = [
            bytemuck::bytes_of(&self.len),
            vec![0; (self.data.len_padding - 1) * 4].as_slice(),
            bytemuck::cast_slice(data),
        ]
        .concat();

        queue.write_buffer(&self.buff, 0, new_data.as_slice());
    }

    pub fn set_buffer(&mut self, device: &wgpu::Device, len: u32) -> bool {
        if len != self.len {
            self.len = len;
            self.buff = Self::get_buffer(
                device,
                len,
                &self.data.name,
                self.data.type_size,
                self.data.len_padding,
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

    fn get_buffer(
        device: &wgpu::Device,
        len: u32,
        name: &'static str,
        type_size: usize,
        len_padding: usize,
    ) -> wgpu::Buffer {
        let size = Self::get_size(len, type_size, len_padding);
        device.create_buffer(&wgpu::BufferDescriptor {
            label: Some(name),
            size,
            usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        })
    }

    fn get_size(len: u32, type_size: usize, len_padding: usize) -> u64 {
        let len = if len == 0 { 1 } else { len } as usize;
        (std::mem::size_of::<u32>() * len_padding + len * type_size) as _
    }
}

impl BaseStorageBufferData {
    pub(super) fn get_wgsl_type(&self) -> String {
        let pascal_case_name = self.name.to_pascal_case();
        let type_name = self.type_name;
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
        let snake_case_name = self.name;
        let pascal_case_name = self.name.to_pascal_case();
        format!(
            r"
@group({group}) @binding({binding})
var<storage> {snake_case_name}: {pascal_case_name};
        "
        )
    }
}
