pub mod ui_point;
pub mod ui_utils;
use glam::Vec2;
use iced::{Element, Task};

use crate::app::edit_object::ui_point::{UIPoint, UIPointElement};

pub trait EditObject<Message> {
    fn get_ui_point(&self) -> Vec<UIPoint>;

    fn get_messages(&self, position: &Vec2) -> Vec<Message>;

    fn update(&mut self, muse_position: Vec2, message: Message) -> Task<Message>;

    fn get_icon(&self) -> Element<'_, Message>;

    fn get_menu(&self) -> Option<Element<'_, Message>>;
}

impl<Message> From<UIPointElement<Message>> for UIPoint {
    fn from(value: UIPointElement<Message>) -> Self {
        value.point
    }
}
