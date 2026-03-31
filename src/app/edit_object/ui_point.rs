use bytemuck::{Pod, Zeroable};
use glam::Vec2;
use iced::{Task, advanced::graphics::futures::MaybeSend};

use crate::app::edit_object::custom_object::param::channel::ChannelType;

pub trait PointsSystem<Message> {
    fn view(&self) -> Vec<UIPointElement<Message>>;

    fn get_ui_points(&self) -> Vec<UIPoint> {
        self.view().into_iter().map(Into::into).collect()
    }

    fn get_message(&mut self, position: &Vec2) -> Option<Message>;

    fn update(&mut self, position: &Vec2, message: Option<Message>) -> Task<Message>;

    fn in_object(&self, point: &Vec2) -> bool;

    fn get_data(&self) -> Vec<(ChannelType, Vec<Vec<u8>>)>;
}

#[macro_export]
macro_rules! into_points_system {
    ($base_type:ty, $base_message:ty, $target_message:ty) => {
        impl PointsSystem<$target_message> for $base_type {
            fn view(&self) -> Vec<UIPointElement<$target_message>> {
                <Self as PointsSystem<$base_message>>::view(self)
                    .iter()
                    .map(|point| UIPointElement {
                        message: point.message.clone().into(),
                        point: point.point,
                    })
                    .collect()
            }

            fn get_message(&mut self, position: &Vec2) -> Option<$target_message> {
                <Self as PointsSystem<$base_message>>::get_message(self, position).map(Into::into)
            }

            fn update(
                &mut self,
                position: &Vec2,
                message: Option<$target_message>,
            ) -> Task<$target_message> {
                <Self as PointsSystem<$base_message>>::update(
                    self,
                    position,
                    message.map(TryInto::try_into).map(Result::ok).flatten(),
                )
                .map(Into::into)
            }

            fn in_object(&self, point: &Vec2) -> bool{
                <Self as PointsSystem<$base_message>>::in_object(self, point)
            }

            fn get_data(&self) -> Vec<(ChannelType, Vec<Vec<u8>>)> {
                <Self as PointsSystem<$base_message>>::get_data(self)
            }
        }
    };
}

#[derive(Clone)]
pub struct UIPointElement<Message> {
    pub point: UIPoint,
    pub message: Message,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Pod, Zeroable, Default)]
pub struct UIPoint {
    pub pos: Vec2,
    pub size: f32,
    _padding: f32,
}

impl UIPoint {
    pub fn new(pos: Vec2, size: f32) -> Self {
        Self {
            pos,
            size,
            _padding: 0.,
        }
    }

    pub fn in_point(&self, pos: &Vec2) -> bool {
        let r = ((self.pos.x - pos.x).powi(2) + (self.pos.y - pos.y).powi(2)).sqrt();
        r < self.size
    }
}
