mod pipeline;

use crate::app::Message;
use crate::app::edit_object;
use crate::app::edit_object::EditObjectSettings;
use crate::app::edit_object::custom_object;
use crate::app::edit_object::custom_object::CustomIndexedObjectSettings;
use crate::app::edit_object::ui_point::UIPoint;
use crate::app::selection::Selection;
use crate::app::settings::Settings;
use crate::app::shader::pipeline::Pipeline;
use crate::app::shader::pipeline::edit_bg::BaseData;
use crate::app::shader::pipeline::edit_bg::EditBG;
use glam::Vec2;
use iced::Rectangle;
use iced::wgpu;
use iced::widget::shader;

#[derive(Debug, Clone)]
pub enum Command {
    None,
    Selection(Selection),
    Points(Vec<UIPoint>),
    UpdateEditObjects {
        shader_objects: Vec<edit_object::ShaderObjects>,
        custom_objects_chenel: custom_object::param::Chanel,
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
                    if pipeline.edit_bg.set_points_buffer(device, ui_points.len()) {
                        pipeline.edit_bg.reload_bg(device);
                    }
                    pipeline.edit_bg.write_points_buffer(queue, ui_points);
                }
                Command::UpdateEditObjects {
                    shader_objects,
                    custom_objects_chenel,
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
                        let seted_custom_objects_buffer = pipeline
                            .edit_bg
                            .set_custom_objects_buffer(device, custom_objects.len());
                        println!("custom_objects_chenel.get_f32().len(): {}", custom_objects_chenel.get_f32().len());
                        let seted_f32_channel_buffer = pipeline
                            .edit_bg
                            .set_f32_channel_buffer(device, custom_objects_chenel.get_f32().len());
                        seted_custom_objects_buffer || seted_f32_channel_buffer
                    };

                    if resize {
                        pipeline.edit_bg.reload_bg(device);
                        println!("edit_bg.reload_bg");
                    }

                    let write = || {
                        pipeline
                            .edit_bg
                            .write_custom_objects_buffer(queue, &custom_objects);
                        pipeline
                            .edit_bg
                            .write_f32_channel_buffer(queue, custom_objects_chenel.get_f32());
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

pub fn get_shader() -> String {
    let custom_objects = Settings::load(None, None).custom_objects;

    include_str!("shader.wgsl")
        .to_string()
        .replace(
            "//{DRAW_CUSTOM_OBJECTS}",
            &custom_objects
                .iter()
                .enumerate()
                .map(|(i, custom_object)| {
                    let name: String = custom_object.get_name();
                    format!("case {i}: {{result = draw_{name}(result, screen_pixel_pos, get_data_{name}(custom_objects.custom_objects[i].channel_index));}}")
                })
                .collect::<Vec<_>>()
                .join("\t\t\t\n")
        )
        .replace(
            "//{DRAW_CUSTOM_OBJECTS_FUNCTION}",
            &custom_objects
                .iter()
                .map(CustomIndexedObjectSettings::get_shader)
                .collect::<Vec<_>>()
                .join("\n"),
        )
}
