use glam::Vec2;
use iced::Task;

use crate::app::edit_object::{UIPoint, UIPointElement};

#[derive(Clone)]
pub enum Message {
    MoveStart,
    MoveStartX,
    MoveStartY,
    MoveEnd,
    MoveEndX,
    MoveEndY,
    MoveStartXEndY,
    MoveStartYEndX,
}

pub fn view(start: &Vec2, end: &Vec2) -> Vec<UIPointElement<Message>> {
    let size = 10.;
    let half_hight = start.y.min(end.y) + ((start.y - end.y).abs() / 2.);
    let half_wight = start.x.min(end.x) + ((start.x - end.x).abs() / 2.);
    vec![
        UIPointElement {
            point: UIPoint::new(*start, size),
            message: Message::MoveStart,
        },
        UIPointElement {
            point: UIPoint::new(*end, size),
            message: Message::MoveEnd,
        },
        UIPointElement {
            point: UIPoint::new(
                Vec2 {
                    x: start.x,
                    y: end.y,
                },
                size,
            ),
            message: Message::MoveStartXEndY,
        },
        UIPointElement {
            point: UIPoint::new(
                Vec2 {
                    x: end.x,
                    y: start.y,
                },
                size,
            ),
            message: Message::MoveStartYEndX,
        },
        UIPointElement {
            point: UIPoint::new(
                Vec2 {
                    x: start.x,
                    y: half_hight,
                },
                size,
            ),
            message: Message::MoveStartX,
        },
        UIPointElement {
            point: UIPoint::new(
                Vec2 {
                    x: end.x,
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
                    y: start.y,
                },
                size,
            ),
            message: Message::MoveStartY,
        },
        UIPointElement {
            point: UIPoint::new(
                Vec2 {
                    x: half_wight,
                    y: end.y,
                },
                size,
            ),
            message: Message::MoveEndY,
        },
    ]
}

pub fn get_message(start: &Vec2, end: &Vec2, position: &Vec2) -> Vec<Message> {
    view(start, end)
        .into_iter()
        .filter(|element| element.point.in_point(position))
        .map(|element| element.message)
        .collect()
}

pub fn update(
    start: &mut Vec2,
    end: &mut Vec2,
    position: &Vec2,
    message: Message,
) -> Task<Message> {
    match message {
        Message::MoveStart => {
            *start = *position;
        }
        Message::MoveStartX => {
            start.x = position.x;
        }
        Message::MoveStartY => {
            start.y = position.y;
        }
        Message::MoveEnd => {
            *end = *position;
        }
        Message::MoveEndX => {
            end.x = position.x;
        }
        Message::MoveEndY => {
            end.y = position.y;
        }
        Message::MoveStartXEndY => {
            start.x = position.x;
            end.y = position.y;
        }
        Message::MoveStartYEndX => {
            start.y = position.y;
            end.x = position.x;
        }
    }

    Task::none()
}
