use std::path::PathBuf;

use iced::widget::image;
use iced_font_awesome::{fa_icon, fa_icon_solid};
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
pub enum Icon {
    Name(String),
    SolidName(String),
    Path(PathBuf),
}

impl Icon {
    pub fn get_icon(&self) -> iced::Element<'_, ()> {
        match self {
            Icon::Name(name) => fa_icon(name).into(),
            Icon::SolidName(name) => fa_icon_solid(name).into(),
            Icon::Path(path) => image(path).into(),
        }
    }
}
