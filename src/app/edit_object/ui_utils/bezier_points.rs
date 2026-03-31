use glam::Vec2;
use iced::Task;

use crate::app::edit_object::ui_point::{PointsSystem, UIPointElement};

#[derive(Clone, Debug)]
pub enum Message {
    
}

#[derive(Debug, Clone, Default)]
pub struct BezierPoints {
    points: Vec<Vec2>,
}

