pub mod pipeline;

use crate::app::Message;
use crate::app::edit_object;
use crate::app::edit_object::custom_object;
use crate::app::edit_object::custom_object::param::channel::ChanelType;
use crate::app::edit_object::ui_point::UIPoint;
use crate::app::edit_object::ui_utils::cube::Cube;
use crate::app::selection::Selection;
use crate::app::shader::pipeline::Pipeline;
use crate::app::shader::pipeline::edit_bg::BaseData;
use crate::app::shader::pipeline::edit_bg::BufferType;
use glam::Vec2;
use iced::Rectangle;
use iced::wgpu;
use iced::widget::shader;

pub mod get_shader;

#[derive(Debug, Clone)]
pub enum Command {
    None,
    Selection(Selection),
    Points(Vec<UIPoint>),
    UpdateEditObjects {
        shader_objects: Vec<edit_object::ShaderObjects>,
        custom_objects_chanel: custom_object::param::channel::Chanels,
    },
}

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
                    bytemuck::bytes_of(&selection.normalize()),
                ),
                Command::Points(ui_points) => {
                    let edited = pipeline.edit_bg.data.storage_buffers.set_buffer(
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
                    custom_objects_chanel,
                } => {
                    let mut custom_objects = vec![];
                    for object in shader_objects {
                        match object {
                            edit_object::ShaderObjects::Custom(custom_object_from_shader) => {
                                custom_objects.push(*custom_object_from_shader);
                            }
                        }
                    }

                    let resize = {
                        let edited_custom_objects_buffer =
                            pipeline.edit_bg.data.storage_buffers.set_buffer(
                                &BufferType::CustomObjects,
                                device,
                                custom_objects.len() as _,
                            );
                        let edited_f32_channel_buffer =
                            pipeline.edit_bg.data.storage_buffers.set_buffer(
                                &BufferType::Channel(ChanelType::F32),
                                device,
                                custom_objects_chanel.get_f32().len() as _,
                            );
                        let edited_cube_channel_buffer =
                            pipeline.edit_bg.data.storage_buffers.set_buffer(
                                &BufferType::Channel(ChanelType::Cube),
                                device,
                                custom_objects_chanel.get_cube().len() as _,
                            );
                        edited_custom_objects_buffer
                            || edited_f32_channel_buffer
                            || edited_cube_channel_buffer
                    };

                    if resize {
                        pipeline.edit_bg.reload_bg(device);
                    }

                    let write = || {
                        pipeline.edit_bg.data.storage_buffers.write(
                            &BufferType::CustomObjects,
                            queue,
                            &custom_objects,
                        );
                        pipeline.edit_bg.data.storage_buffers.write(
                            &BufferType::Channel(ChanelType::F32),
                            queue,
                            custom_objects_chanel.get_f32(),
                        );
                        pipeline.edit_bg.data.storage_buffers.write(
                            &BufferType::Channel(ChanelType::Cube),
                            queue,
                            &custom_objects_chanel
                                .get_cube()
                                .iter()
                                .map(Cube::normalize)
                                .collect::<Vec<_>>(),
                        );
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
