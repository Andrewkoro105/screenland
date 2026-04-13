use std::ops::Not;

use glam::{Vec2, vec2};
use iced::Task;

use crate::{
    app::edit_object::{
        custom_object::{self, param::channel::ChannelType},
        points_system::{PointsSystem, Reload},
        ui_point::{UIMessages, UIPoint, UIPointElement},
    },
    into_points_system,
};

#[derive(Clone, Debug)]
pub enum Message {
    Move(usize),
    MoveAll,
    ActivatePoints(usize),
}

#[derive(Debug, Clone, Default)]
pub struct BezierPoints {
    start_touch: Option<Vec2>,
    points: Vec<Vec2>,
}

into_points_system!(BezierPoints, Message, custom_object::points::PointsMessage);

impl PointsSystem<Message> for BezierPoints {
    fn view(&self) -> Vec<UIPointElement<Message>> {
        let indexed_points = self.points.iter().enumerate().collect::<Vec<_>>();
        let pair_indexed_points = indexed_points.iter().zip(indexed_points.iter().skip(1));
        let mut result = vec![];
        for ((i1, point1), (_, point2)) in pair_indexed_points {
            result.push(UIPointElement {
                point: UIPoint::new(**point1, 10.),
                messages: UIMessages::from_message(Message::Move(*i1)),
            });

            result.push(UIPointElement {
                point: UIPoint::new(**point1 + ((**point2 - **point1) / vec2(2., 2.)), 5.),
                messages: UIMessages {
                    message: Message::Move(*i1 + 1),
                    start_messages: vec![Message::ActivatePoints(*i1 + 1)],
                },
            });
        }

        if let Some((i, point)) = indexed_points.last() {
            result.push(UIPointElement {
                point: UIPoint::new(**point, 10.),
                messages: UIMessages::from_message(Message::Move(*i)),
            });
        }

        result
    }

    fn get_message(&mut self, position: &Vec2) -> Option<Reload<UIMessages<Message>>> {
        if self.points.is_empty() {
            self.points = vec![position.clone(), position.clone()];
            Some(Reload::new(
                true,
                UIMessages::from_message(Message::Move(0)),
            ))
        } else {
            <Self as PointsSystem<Message>>::get_message_view_points(self, position)
                .or_else(|| {
                    <Self as PointsSystem<Message>>::in_object(self, position)
                        .then_some(UIMessages::from_message(Message::MoveAll))
                })
                .map(|message| Reload::new(false, message))
        }
    }

    fn update(&mut self, position: &Vec2, message: Option<Message>) -> Reload<Task<Message>> {
        match message {
            Some(message) => match message {
                Message::Move(i) => {
                    self.points[i] = *position;
                    Reload::none()
                }
                Message::MoveAll => {
                    if let Some(first_points) = self.points.first() {
                        if let Some(start_touch) = self.start_touch {
                            let dist = *position - (start_touch + *first_points);
                            for point in self.points.iter_mut() {
                                *point += dist;
                            }
                            Reload::none()
                        } else {
                            self.start_touch = Some(*position - *first_points);
                            Reload::none()
                        }
                    } else {
                        Reload::none()
                    }
                }
                Message::ActivatePoints(i) => {
                    self.points.insert(i, *position);
                    Reload::new(true, Task::done(Message::Move(i)))
                }
            },
            None => {
                self.start_touch = None;
                Reload::none()
            }
        }
    }

    fn in_object(&self, point: &Vec2) -> bool {
        self.points
            .is_empty()
            .not()
            .then(|| {
                let min_x = self
                    .points
                    .iter()
                    .map(|point| point.x)
                    .min_by(|x1, x2| x1.total_cmp(&x2))
                    .unwrap();
                let max_x = self
                    .points
                    .iter()
                    .map(|point| point.x)
                    .max_by(|x1, x2| x1.total_cmp(&x2))
                    .unwrap();
                let min_y = self
                    .points
                    .iter()
                    .map(|point| point.y)
                    .min_by(|y1, y2| y1.total_cmp(&y2))
                    .unwrap();
                let max_y = self
                    .points
                    .iter()
                    .map(|point| point.y)
                    .max_by(|y1, y2| y1.total_cmp(&y2))
                    .unwrap();
                min_x < point.x && point.x < max_x && min_y < point.y && point.y < max_y
            })
            .unwrap_or(false)
    }

    fn get_data(&self) -> Vec<(ChannelType, Vec<Vec<u8>>)> {
        vec![
            (
                ChannelType::BezierPointsLen,
                vec![bytemuck::bytes_of(&(self.points.len() as u32)).to_vec()],
            ),
            (
                ChannelType::BezierPoints,
                self.points
                    .iter()
                    .map(|point| bytemuck::bytes_of(point).to_vec())
                    .collect(),
            ),
        ]
    }
}

impl TryFrom<custom_object::points::PointsMessage> for Message {
    type Error = ();

    fn try_from(value: custom_object::points::PointsMessage) -> Result<Self, Self::Error> {
        if let custom_object::points::PointsMessage::BezierPoints(message) = value {
            Ok(message)
        } else {
            Err(())
        }
    }
}

impl From<Message> for custom_object::points::PointsMessage {
    fn from(value: Message) -> Self {
        Self::BezierPoints(value)
    }
}
