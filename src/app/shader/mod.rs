pub mod pipeline;

use crate::app::Message;
use crate::app::edit_object;
use crate::app::edit_object::custom_object;
use crate::app::edit_object::custom_object::CustomObjectFromShader;
use crate::app::edit_object::custom_object::param::channel::ChannelType;
use crate::app::edit_object::ui_point::UIPoint;
use crate::app::selection::Selection;
use crate::app::shader::pipeline::Pipeline;
use crate::app::shader::pipeline::edit_bg::BaseData;
use crate::app::shader::pipeline::edit_bg::BufferType;
use glam::Vec2;
use iced::Rectangle;
use iced::wgpu;
use iced::widget::shader;
use strum::IntoEnumIterator;

pub mod get_shader;

/// Data blocks that are updated synchronously
#[derive(Debug, Clone)]
pub enum Command {
    /// No data to update
    None,
    /// Update the selected region
    Selection(Selection),
    /// Updating the points displayed to the user
    Points(Vec<UIPoint>),
    /// Updating custom objects consists of the state of the objects and their channels 
    UpdateEditObjects {
        shader_objects: Vec<edit_object::ShaderObjects>,
        custom_objects_channel: custom_object::param::channel::Channels,
    },
}

/// The `Program` shader primitive passes data to the queue and reconfigures the pipeline
#[derive(Debug, Clone)]
pub struct Primitive {
    start_data: BaseData,
    commands: Vec<Command>,
}

impl shader::Primitive for Primitive {
    type Pipeline = Pipeline;

    fn prepare(
        &self,
        pipeline: &mut Self::Pipeline,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        _bounds: &Rectangle,
        _viewport: &shader::Viewport,
    ) {
        queue.write_buffer(
            &pipeline.edit_bg.data.base_data_buffer,
            0,
            bytemuck::bytes_of(&self.start_data),
        );

        for command in &self.commands {
            match command {
                Command::None => {}
                Command::Selection(selection) => queue.write_buffer(
                    &pipeline.edit_bg.data.selection_buffer,
                    0,
                    &selection.get_data(),
                ),
                Command::Points(ui_points) => {
                    let edited = pipeline.edit_bg.data.storage_buffers.resize(
                        &BufferType::Points,
                        device,
                        ui_points.len() as _,
                    );

                    if edited {
                        pipeline.edit_bg.reload_bg(device);
                    }

                    pipeline.edit_bg.data.storage_buffers.write(
                        &BufferType::Points,
                        queue,
                        ui_points,
                    );
                }
                Command::UpdateEditObjects {
                    shader_objects,
                    custom_objects_channel,
                } => {
                    let mut custom_objects = vec![];
                    for object in shader_objects {
                        match object {
                            edit_object::ShaderObjects::Custom(custom_object_from_shader) => {
                                custom_objects.push(custom_object_from_shader.clone());
                            }
                        }
                    }

                    let resize = {
                        let edited_custom_objects_buffer =
                            pipeline.edit_bg.data.storage_buffers.resize(
                                &BufferType::CustomObjects,
                                device,
                                custom_objects.len() as _,
                            );

                        let mut edited_channel_buffers = false;
                        for channel_type in ChannelType::iter() {
                            let edited_channel_buffer =
                                pipeline.edit_bg.data.storage_buffers.resize(
                                    &BufferType::Channel(channel_type.clone()),
                                    device,
                                    custom_objects_channel.get(&channel_type).len() as _,
                                );
                            edited_channel_buffers =
                                edited_channel_buffers || edited_channel_buffer;
                        }
                        edited_custom_objects_buffer || edited_channel_buffers
                    };

                    if resize {
                        pipeline.edit_bg.reload_bg(device);
                    }

                    let write = || {
                        pipeline.edit_bg.data.storage_buffers.write(
                            &BufferType::CustomObjects,
                            queue,
                            &custom_objects
                                .iter()
                                .flat_map(CustomObjectFromShader::to_bytes)
                                .collect::<Vec<_>>(),
                        );

                        for channel_type in ChannelType::iter() {
                            pipeline.edit_bg.data.storage_buffers.write(
                                &BufferType::Channel(channel_type.clone()),
                                queue,
                                custom_objects_channel.get(&channel_type),
                            );
                        }
                    };
                    write();
                }
            }
        }
    }

    fn draw(&self, pipeline: &Self::Pipeline, render_pass: &mut wgpu::RenderPass<'_>) -> bool {
        render_pass.set_pipeline(&pipeline.pipeline);
        render_pass.set_bind_group(0, &pipeline.screen_bg.bg, &[]);
        render_pass.set_bind_group(1, &pipeline.edit_bg.bg, &[]);
        render_pass.draw(0..3, 0..1);
        true
    }
}

/// Data passed to the shader during an update
pub struct Program {
    pub monitor_pos: Vec2,
    pub commands: Vec<Command>,
}

impl shader::Program<Message> for Program {
    type State = ();
    type Primitive = Primitive;

    fn draw(
        &self,
        _state: &Self::State,
        _cursor: iced::mouse::Cursor,
        bounds: Rectangle,
    ) -> Self::Primitive {
        Self::Primitive {
            start_data: BaseData {
                resolution: Vec2 {
                    x: bounds.x,
                    y: bounds.y,
                },
                monitor_pos: self.monitor_pos,
            },
            commands: self.commands.clone(),
        }
    }
}
