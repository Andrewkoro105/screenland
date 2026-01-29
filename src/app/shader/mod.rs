mod pipeline;

use glam::Vec2;
use iced::Rectangle;
use iced::wgpu;
use iced::widget::shader;
use crate::app::Message;
use crate::app::selection::Selection;
use crate::app::shader::pipeline::Pipeline;
use crate::app::shader::pipeline::edit_bg::BaseData;

#[derive(Debug, Clone)]
pub enum PrimitiveCommand {
    None,
    Selection(Selection),
}

#[derive(Debug, Clone)]
pub struct Primitive {
    start_data: BaseData,
    command: PrimitiveCommand,
}

impl shader::Primitive for Primitive {
    type Pipeline = Pipeline;

    fn prepare(
        &self,
        pipeline: &mut Self::Pipeline,
        _device: &wgpu::Device,
        queue: &wgpu::Queue,
        _bounds: &Rectangle,
        _viewport: &shader::Viewport,
    ) {
        queue.write_buffer(
            &pipeline.edit_bg.base_data_buffer,
            0,
            bytemuck::bytes_of(&self.start_data),
        );

        match self.command {
            PrimitiveCommand::None => {}
            PrimitiveCommand::Selection(selection) => queue.write_buffer(
                &pipeline.edit_bg.selection_buffer,
                0,
                bytemuck::bytes_of(&selection.normalize()),
            ),
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

pub enum Command {
    None,
    Selection(Selection),
}

pub struct Program {
    pub monitor_pos: Vec2,
    pub command: Command,
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
            command: match self.command {
                Command::None => PrimitiveCommand::None,
                Command::Selection(selection) => PrimitiveCommand::Selection(selection),
            },
        }
    }
}
