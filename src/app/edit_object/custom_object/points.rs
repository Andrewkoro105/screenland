use std::fmt::Debug;

use glam::Vec2;
use iced::Task;
use serde::{Deserialize, Serialize};

use crate::app::{
    self,
    edit_object::{
        self,
        custom_object::data_type::DataType,
        ui_point::PointsSystem,
        ui_utils::cube::{self, Cube},
    },
};

#[derive(Clone, Serialize, Deserialize)]
pub enum PointsFormat {
    Cube,
}

#[derive(Clone, Debug)]
pub enum PointsMessage {
    Cube(cube::Message),
}

impl DataType for PointsFormat {
    fn get_type_name(&self) -> String {
        match self {
            PointsFormat::Cube => "Cube".into(),
        }
    }
}

impl From<PointsFormat> for Box<dyn CustomObjectPointSystem> {
    fn from(value: PointsFormat) -> Self {
        match value {
            PointsFormat::Cube => Box::new(Cube::default()),
        }
    }
}

pub trait CustomObjectPointSystem: PointsSystem<PointsMessage> + Debug {
    fn custom_object_update(
        &mut self,
        i: usize,
        mouse_pos: Vec2,
        message: Option<PointsMessage>,
    ) -> Task<app::Message>;
}

impl<T: PointsSystem<PointsMessage> + Debug + Clone> CustomObjectPointSystem for T {
    fn custom_object_update(
        &mut self,
        i: usize,
        mouse_pos: Vec2,
        message: Option<PointsMessage>,
    ) -> Task<app::Message> {
        self.update(&mouse_pos, message)
            .map(Some)
            .map(super::Message::Point)
            .map(move |message| {
                app::Message::UpdateEditObject((i.clone(), edit_object::Message::Custom(message)))
            })
            .chain(Task::batch(self.get_data().into_iter().flat_map(
                |(channel_type, data)| {
                    data.into_iter().enumerate().map(move |(index, data)|
                    Task::done(app::Message::CustomObjectsChannelUpdate {
                        i,
                        index,
                        channel_type: channel_type.clone(),
                        data,
                    }))
                },
            )))
    }
}
