use std::{collections::HashMap, hash::Hash};

use bytemuck::NoUninit;
use iced::wgpu::{self, BindGroupEntry, BindGroupLayoutEntry};

use crate::app::shader::pipeline::base_storage_buffers::base_storage_buffer::{
    BaseStorageBuffer, BaseStorageBufferData,
};

pub mod base_storage_buffer;

pub trait GetDates {
    fn get_data(&self) -> Vec<(u32, &BaseStorageBufferData)>;
}

pub trait GetShader {
    fn get_shader(&self) -> String;
}

pub struct BaseStorageBuffers<BufferType: Hash + PartialEq + Eq, T>
where
    Self: GetDates,
{
    group: u32,
    start_binding: u32,
    buffers: HashMap<BufferType, T>,
}

impl<BufferType: Hash + PartialEq + Eq> BaseStorageBuffers<BufferType, BaseStorageBuffer> {
    pub fn get_bind_group_layout_entry(&self) -> Vec<BindGroupLayoutEntry> {
        self.get_vec()
            .into_iter()
            .map(|buff| buff.get_bind_group_layout_entry(self.start_binding))
            .collect()
    }

    pub fn get_bind_group_entry(&self) -> Vec<BindGroupEntry<'_>> {
        self.get_vec()
            .into_iter()
            .map(|buff| buff.get_bind_group_entry(self.start_binding))
            .collect()
    }

    pub fn set_buffer(&mut self, chanel: &BufferType, device: &wgpu::Device, len: u32) -> bool {
        self.buffers
            .get_mut(chanel)
            .unwrap()
            .set_buffer(device, len)
    }

    pub fn write<A: NoUninit>(&self, chanel: &BufferType, queue: &wgpu::Queue, data: &[A]) {
        self.buffers
            .get(chanel)
            .unwrap()
            .write_buffer(queue, data);
    }
}

impl<BufferType: Hash + PartialEq + Eq> BaseStorageBuffers<BufferType, BaseStorageBufferData> {
    pub fn into_buff(
        self,
        device: &wgpu::Device,
    ) -> BaseStorageBuffers<BufferType, BaseStorageBuffer> {
        BaseStorageBuffers::new(
            self.group,
            self.start_binding,
            self.buffers
                .into_iter()
                .enumerate()
                .map(|(i, (key, data))| (key, BaseStorageBuffer::new(device, data, i as _)))
                .collect(),
        )
    }
}

impl<BufferType: Hash + PartialEq + Eq, T> BaseStorageBuffers<BufferType, T>
where
    Self: GetDates,
{
    pub fn new(group: u32, start_binding: u32, buffers: HashMap<BufferType, T>) -> Self {
        Self {
            group,
            start_binding,
            buffers,
        }
    }

    fn get_vec(&self) -> Vec<&T> {
        self.buffers.iter().map(|(_, a)| a).collect()
    }
}

impl<BufferType: Hash + PartialEq + Eq, T> GetShader for BaseStorageBuffers<BufferType, T>
where
    Self: GetDates,
{
    fn get_shader(&self) -> String {
        let dates = self.get_data();
        [
            dates
                .iter()
                .map(|(_, data)| data.get_wgsl_type())
                .collect::<Vec<String>>()
                .join("\n"),
            dates
                .iter()
                .map(|(binding, data)| data.get_wgsl_var(self.group, self.start_binding + *binding))
                .collect::<Vec<String>>()
                .join("\n"),
        ]
        .join("\n")
    }
}

impl<BufferType: Hash + PartialEq + Eq> GetDates
    for BaseStorageBuffers<BufferType, BaseStorageBuffer>
{
    fn get_data(&self) -> Vec<(u32, &BaseStorageBufferData)> {
        self.get_vec()
            .into_iter()
            .map(|buff| {
                (
                    buff.get_offset_binding(),
                    buff.get_data(),
                )
            })
            .collect()
    }
}

impl<BufferType: Hash + PartialEq + Eq> GetDates
    for BaseStorageBuffers<BufferType, BaseStorageBufferData>
{
    fn get_data(&self) -> Vec<(u32, &BaseStorageBufferData)> {
        self.get_vec()
            .into_iter()
            .enumerate()
            .map(|(a, b)| (a as _, b))
            .collect()
    }
}
