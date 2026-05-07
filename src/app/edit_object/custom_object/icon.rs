//!It offers various ways to upload an icon 

use std::path::PathBuf;
use iced::widget::image;
use iced_font_awesome::{fa_icon, fa_icon_solid};
use serde::{Deserialize, Serialize};

/// It offers various ways to upload an icon 
#[derive(Clone, Debug, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub enum Icon {
    Name(String),
    SolidName(String),
    Path(PathBuf),
}

impl Icon {
    pub fn get_icon<M>(&self) -> iced::Element<'_, M> {
        match self {
            Icon::Name(name) => fa_icon(name).into(),
            Icon::SolidName(name) => fa_icon_solid(name).into(),
            Icon::Path(path) => image(path).into(),
        }
    }
}
