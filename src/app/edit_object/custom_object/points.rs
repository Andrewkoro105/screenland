use std::fmt::Debug;

use glam::Vec2;
use heck::ToSnakeCase;
use iced::Task;
use serde::{Deserialize, Serialize};
use strum::Display;
use tracing::warn;

use crate::app::{
    self,
    edit_object::{
        self,
        custom_object::data_type::DataType,
        points_system::PointsSystem,
        ui_utils::{
            bezier_points::{self, BezierPoints},
            cube::{self, Cube},
        },
    },
};

#[derive(Clone, Display, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum PointsFormat {
    Cube,
    BezierPoints,
}

#[derive(Clone, Debug)]
pub enum PointsMessage {
    Cube(cube::Message),
    BezierPoints(bezier_points::Message),
}

impl DataType for PointsFormat {
    fn get_name(&self) -> String {
        match self {
            PointsFormat::BezierPoints => self.to_string(),
            _ => self.get_type_name(),
        }
        .to_snake_case()
    }

    fn base_get_type_name(&self) -> String {
        self.to_string()
    }

    fn is_iter(&self) -> bool {
        match self {
            PointsFormat::BezierPoints => true,
            _ => false,
        }
    }
}

impl From<PointsFormat> for Box<dyn CustomObjectPointSystem> {
    fn from(value: PointsFormat) -> Self {
        match value {
            PointsFormat::Cube => Box::new(Cube::default()),
            PointsFormat::BezierPoints => {
                warn!(
                    "`BezierPoints::view()` is not yet complete, and the intermediate points are being placed linearly even though they should be placed differently"
                );
                Box::new(BezierPoints::default())
            }
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

impl<T: PointsSystem<PointsMessage> + Debug> CustomObjectPointSystem for T {
    fn custom_object_update(
        &mut self,
        i: usize,
        mouse_pos: Vec2,
        message: Option<PointsMessage>,
    ) -> Task<app::Message> {
        let reload = self.update(&mouse_pos, message);
        reload
            .task_map(Some)
            .task_map(super::Message::Point)
            .task_map(move |message| {
                app::Message::UpdateEditObject((i.clone(), edit_object::Message::Custom(message)))
            })
            .map(|task| {
                task.chain(Task::batch(self.get_data().into_iter().flat_map(
                    |(channel_type, data)| {
                        data.into_iter().enumerate().map(move |(index, data)| {
                            Task::done(app::Message::CustomObjectsChannelUpdate {
                                i,
                                index,
                                channel_type: channel_type.clone(),
                                data,
                            })
                        })
                    },
                )))
            })
            .get_task()
    }
}
