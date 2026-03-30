use glam::Vec2;
use iced::Task;
use serde::{Deserialize, Serialize};

use crate::app::{
    self,
    edit_object::{
        self,
        custom_object::{data_type::DataType, param::channel::{self, ChannelType}},
        ui_point::{PointsSystem, UIPoint},
        ui_utils::cube::{self, Cube},
    },
};

#[derive(Clone, Serialize, Deserialize)]
pub enum PointsFormat {
    Cube,
}

#[derive(Debug, Clone)]
pub enum PointsData {
    Cube(Cube),
}

#[derive(Clone, Debug)]
pub enum PointsMessage {
    Cube(cube::Message),
}

impl DataType for PointsFormat {
    fn get_name(&self) -> String {
        match self {
            PointsFormat::Cube => "cube".into(),
        }
    }

    fn get_type_name(&self) -> String {
        match self {
            PointsFormat::Cube => "Cube".into(),
        }
    }
}

impl From<PointsFormat> for PointsData {
    fn from(value: PointsFormat) -> Self {
        match value {
            PointsFormat::Cube => PointsData::Cube(Cube::default()),
        }
    }
}

impl PointsData {
    pub fn update(
        &mut self,
        i: usize,
        mouse_pos: Vec2,
        message: Option<PointsMessage>,
    ) -> Task<app::Message> {
        match self {
            PointsData::Cube(cube) => cube
                .update(&mouse_pos, message.map(PointsMessage::get_cube).flatten())
                .map(PointsMessage::Cube)
                .map(Some)
                .map(super::Message::Point)
                .map(move |message| {
                    app::Message::UpdateEditObject((
                        i.clone(),
                        edit_object::Message::Custom(message),
                    ))
                })
                .chain(Task::done(app::Message::CustomObjectsChannelUpdate {
                    i,
                    index: 0,
                    channel_type: channel::ChannelType::Cube,
                    data: bytemuck::bytes_of(&cube.normalize()).to_vec(),
                })),
        }
    }

    pub fn get_ui_point(&self) -> Vec<UIPoint> {
        match self {
            PointsData::Cube(cube) => cube
                .normalize()
                .view()
                .into_iter()
                .map(Into::into)
                .collect(),
        }
    }

    pub fn get_messages(&mut self, position: &Vec2) -> Option<PointsMessage> {
        match self {
            PointsData::Cube(cube) => cube
                .get_message(position)
                .map(PointsMessage::Cube),
        }
    }

    pub fn get_channel_type(&self) -> ChannelType {
        match self {
            PointsData::Cube(_) => ChannelType::Cube,
        }
    }

    pub fn get_data(&self) -> Vec<u8> {
        match self {
            PointsData::Cube(cube) => bytemuck::bytes_of(&cube.normalize()).to_vec(),
        }
    }
}

impl PointsMessage {
    pub fn get_cube(self) -> Option<cube::Message> {
        if let PointsMessage::Cube(message) = self {
            Some(message)
        } else {
            None
        }
    }
}
