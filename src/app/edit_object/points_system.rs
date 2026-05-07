//! Standardizes the use of point systems

use glam::Vec2;
use iced::{Task, advanced::graphics::futures::MaybeSend};

use crate::app::{
    self,
    edit_object::{
        custom_object::param::channel::ChannelType,
        ui_point::{UIMessages, UIPoint, UIPointElement},
    },
};

/// Indicates whether objects need to be reloaded into the shader; this is necessary if the amount of data in the channels has changed
pub struct Reload<T> {
    reload: bool,
    data: T,
}

/// Standardizes the use of point systems
pub trait PointsSystem<Message> {
    fn view(&self) -> Vec<UIPointElement<Message>>;

    fn get_ui_points(&self) -> Vec<UIPoint> {
        self.view().into_iter().map(Into::into).collect()
    }

    fn get_message(&mut self, position: &Vec2) -> Option<Reload<UIMessages<Message>>>;

    fn get_message_view_points(&mut self, position: &Vec2) -> Option<UIMessages<Message>> {
        <Self as PointsSystem<Message>>::view(self)
            .into_iter()
            .filter(|element| element.point.in_point(position))
            .map(|element| element.messages)
            .next()
    }

    fn update(&mut self, position: &Vec2, message: Option<Message>) -> Reload<Task<Message>>;

    fn in_object(&self, point: &Vec2) -> bool;

    fn get_data(&self) -> Vec<(ChannelType, Vec<Vec<u8>>)>;
}

#[macro_export]
macro_rules! into_points_system {
    ($base_type:ty, $base_message:ty, $target_message:ty) => {
        impl PointsSystem<$target_message> for $base_type {
            fn view(&self) -> Vec<UIPointElement<$target_message>> {
                <Self as PointsSystem<$base_message>>::view(self)
                    .into_iter()
                    .map(UIPointElement::into_ui_point_element)
                    .collect()
            }

            fn get_message(
                &mut self,
                position: &Vec2,
            ) -> Option<Reload<UIMessages<$target_message>>> {
                <Self as PointsSystem<$base_message>>::get_message(self, position)
                    .map(|reload| reload.map(|messages| messages.map(Into::into)))
            }

            fn update(
                &mut self,
                position: &Vec2,
                message: Option<$target_message>,
            ) -> Reload<Task<$target_message>> {
                let reload = <Self as PointsSystem<$base_message>>::update(
                    self,
                    position,
                    message.map(TryInto::try_into).map(Result::ok).flatten(),
                );

                reload.task_map(Into::into)
            }

            fn in_object(&self, point: &Vec2) -> bool {
                <Self as PointsSystem<$base_message>>::in_object(self, point)
            }

            fn get_data(&self) -> Vec<(ChannelType, Vec<Vec<u8>>)> {
                <Self as PointsSystem<$base_message>>::get_data(self)
            }
        }
    };
}

impl<T> Reload<T> {
    pub fn new(reload: bool, data: T) -> Self {
        Self { reload, data }
    }

    pub fn map<T2>(self, f: impl Fn(T) -> T2) -> Reload<T2> {
        Reload {
            reload: self.reload,
            data: f(self.data),
        }
    }
}

impl Reload<Task<app::Message>> {
    /// Returns `data` after calling `app::Message::ReloadShaderObjects` on it if `reload` is `true`
    pub fn get_task(self) -> Task<app::Message> {
        if self.reload {
            Task::done(app::Message::ReloadShaderObjects)
        } else {
            Task::none()
        }
        .chain(self.data)
    }
}

impl<M: MaybeSend + 'static> Reload<Task<M>> {
    pub fn task_map<M2: MaybeSend + 'static>(
        self,
        into: impl FnMut(M) -> M2 + MaybeSend + 'static,
    ) -> Reload<Task<M2>> {
        Reload {
            reload: self.reload,
            data: self.data.map(into),
        }
    }

    pub fn none() -> Self {
        Self::new(false, Task::none())
    }
}

impl<M> Reload<UIMessages<M>> {
    pub fn messages_map<M2>(
        self,
        into: impl Fn(M) -> M2,
    ) -> Reload<UIMessages<M2>> {
        Reload {
            reload: self.reload,
            data: self.data.map(into),
        }
    }
}
