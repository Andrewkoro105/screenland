use bytemuck::{Pod, Zeroable};
use glam::Vec2;
use iced::Task;

use crate::{
    app::edit_object::{
        custom_object::{
            self,
            param::channel::ChannelType,
            points::{self},
        },
        ui_point::{PointsSystem, UIPoint, UIPointElement},
    },
    into_points_system,
};

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Message {
    Move,
    MoveStart,
    MoveStartX,
    MoveStartY,
    MoveEnd,
    MoveEndX,
    MoveEndY,
    MoveStartXEndY,
    MoveStartYEndX,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Pod, Zeroable, Default)]
pub struct Cube {
    pub start: Vec2,
    pub end: Vec2,
    pub start_touch: Vec2,
    pub touched: u32,
    pub init: u32,
}

into_points_system!(Cube, Message, custom_object::points::PointsMessage);

impl PointsSystem<Message> for Cube {
    fn view(&self) -> Vec<UIPointElement<Message>> {
        let size = if self.init == 0 { 0. } else { 10. };
        let half_hight = self.start.y.min(self.end.y) + ((self.start.y - self.end.y).abs() / 2.);
        let half_wight = self.start.x.min(self.end.x) + ((self.start.x - self.end.x).abs() / 2.);
        vec![
            UIPointElement {
                point: UIPoint::new(self.start, size),
                message: Message::MoveStart,
            },
            UIPointElement {
                point: UIPoint::new(self.end, size),
                message: Message::MoveEnd,
            },
            UIPointElement {
                point: UIPoint::new(
                    Vec2 {
                        x: self.start.x,
                        y: self.end.y,
                    },
                    size,
                ),
                message: Message::MoveStartXEndY,
            },
            UIPointElement {
                point: UIPoint::new(
                    Vec2 {
                        x: self.end.x,
                        y: self.start.y,
                    },
                    size,
                ),
                message: Message::MoveStartYEndX,
            },
            UIPointElement {
                point: UIPoint::new(
                    Vec2 {
                        x: self.start.x,
                        y: half_hight,
                    },
                    size,
                ),
                message: Message::MoveStartX,
            },
            UIPointElement {
                point: UIPoint::new(
                    Vec2 {
                        x: self.end.x,
                        y: half_hight,
                    },
                    size,
                ),
                message: Message::MoveEndX,
            },
            UIPointElement {
                point: UIPoint::new(
                    Vec2 {
                        x: half_wight,
                        y: self.start.y,
                    },
                    size,
                ),
                message: Message::MoveStartY,
            },
            UIPointElement {
                point: UIPoint::new(
                    Vec2 {
                        x: half_wight,
                        y: self.end.y,
                    },
                    size,
                ),
                message: Message::MoveEndY,
            },
        ]
    }

    fn get_message(&mut self, position: &Vec2) -> Option<Message> {
        if self.init == 0 {
            self.init = 1;
            self.start = *position;
            self.end = *position;
            Some(Message::MoveEnd)
        } else {
            <Self as PointsSystem<Message>>::view(self)
                .into_iter()
                .filter(|element| element.point.in_point(position))
                .map(|element| element.message)
                .next()
                .or_else(|| {
                    <Self as PointsSystem<Message>>::in_object(self, position)
                        .then_some(Message::Move)
                })
        }
    }

    fn update(&mut self, position: &Vec2, message: Option<Message>) -> Task<Message> {
        match message {
            None => {
                self.touched = 0;
            }
            Some(Message::Move) => {
                if self.touched == 1 {
                    let dist = *position - (self.start_touch + self.start);
                    self.start += dist;
                    self.end += dist;
                } else if self.touched == 0 {
                    self.touched = 1;
                    self.start_touch = *position - self.start;
                } else {
                    unreachable!("`Cube::touched` cannot be equal to {}", self.touched)
                }
            }
            Some(Message::MoveStart) => {
                self.start = *position;
            }
            Some(Message::MoveStartX) => {
                self.start.x = position.x;
            }
            Some(Message::MoveStartY) => {
                self.start.y = position.y;
            }
            Some(Message::MoveEnd) => {
                self.end = *position;
            }
            Some(Message::MoveEndX) => {
                self.end.x = position.x;
            }
            Some(Message::MoveEndY) => {
                self.end.y = position.y;
            }
            Some(Message::MoveStartXEndY) => {
                self.start.x = position.x;
                self.end.y = position.y;
            }
            Some(Message::MoveStartYEndX) => {
                self.start.y = position.y;
                self.end.x = position.x;
            }
        }

        Task::none()
    }

    fn in_object(&self, point: &Vec2) -> bool {
        let data = self.normalize();
        data.start.x < point.x
            && point.x < data.end.x
            && data.start.y < point.y
            && point.y < data.end.y
    }

    fn get_data(&self) -> Vec<(ChannelType, Vec<Vec<u8>>)> {
        vec![(
            ChannelType::Cube,
            vec![bytemuck::bytes_of(&self.normalize()).to_vec()],
        )]
    }
}

impl Cube {
    pub fn normalize(&self) -> Self {
        Self {
            start: Vec2 {
                x: self.start.x.min(self.end.x),
                y: self.start.y.min(self.end.y),
            },
            end: Vec2 {
                x: self.start.x.max(self.end.x),
                y: self.start.y.max(self.end.y),
            },
            ..*self
        }
    }
}

impl From<Message> for points::PointsMessage {
    fn from(value: Message) -> Self {
        points::PointsMessage::Cube(value)
    }
}

impl TryFrom<points::PointsMessage> for Message {
    type Error = ();

    fn try_from(value: points::PointsMessage) -> Result<Self, Self::Error> {
        if let points::PointsMessage::Cube(cube) = value {
            Ok(cube)
        } else {
            Err(())
        }
    }
}
