use glam::Vec2;
use iced::Task;
use serde::{Deserialize, Serialize};

use crate::app::edit_object::{ui_point::UIPoint, ui_utils::cube};

#[derive(Clone, Serialize, Deserialize)]
pub enum PointsFormat {
    Cube,
}

#[derive(Debug, Clone)]
pub enum PointsData {
    Cube { start: Vec2, end: Vec2 },
}

#[derive(Clone)]
pub enum PointsMessage {
    Cube(cube::Message),
}

impl PointsFormat {
    pub fn get_str_init_field(&self) -> String {
        match self {
            PointsFormat::Cube => "points_data: cube_channel[channel_index.cube]".into(),
        }
    }
}

impl From<PointsFormat> for PointsData {
    fn from(value: PointsFormat) -> Self {
        match value {
            PointsFormat::Cube => PointsData::Cube {
                start: Vec2 { x: 0., y: 0. },
                end: Vec2 { x: 0., y: 0. },
            },
        }
    }
}

impl PointsData {
    pub fn update(&mut self, mouse_pos: Vec2, message: PointsMessage) -> Task<PointsMessage> {
        match self {
            PointsData::Cube { start, end } => {
                cube::update(start, end, &mouse_pos, message.get_cube().unwrap())
                    .map(PointsMessage::Cube)
            }
        }
    }

    pub fn get_ui_point(&self) -> Vec<UIPoint> {
        match self {
            PointsData::Cube { start, end } => {
                let mut start = *start;
                let mut end = *end;
                cube::normalize(&mut start, &mut end);
                cube::view(&start, &end)
                    .into_iter()
                    .map(Into::into)
                    .collect()
            }
        }
    }

    pub fn get_messages(&self, position: &Vec2) -> Vec<PointsMessage> {
        match self {
            PointsData::Cube { start, end } => cube::get_message(start, end, position)
                .into_iter()
                .map(PointsMessage::Cube)
                .collect(),
        }
    }
}

impl PointsMessage {
    pub fn get_cube(&self) -> Option<cube::Message> {
        if let PointsMessage::Cube(message) = self {
            Some(message.clone())
        } else {
            None
        }
    }
}
