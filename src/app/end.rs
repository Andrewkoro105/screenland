#[cfg(target_os = "linux")]
use arboard::SetExtLinux;
use arboard::{Clipboard, ImageData};
use chrono::Local;
use image::{RgbaImage, imageops::crop};
use native_dialog::DialogBuilder;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

use crate::{
    app::{Screenland, selection::Selection, settings::Settings},
    screenshots::{ColorFormat, full_screenshot},
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum End {
    Save,
    Copy,
}

impl Screenland {
    pub fn screenshot(selection: Selection, color_format: &ColorFormat) -> RgbaImage {
        let mut screenshot = full_screenshot(color_format);
        let select = selection.normalize();
        crop(
            &mut screenshot,
            select.cube.start.x as _,
            select.cube.start.y as _,
            (select.cube.end.x - select.cube.start.x) as _,
            (select.cube.end.y - select.cube.start.y) as _,
        )
        .to_image()
    }
}

impl End {
    pub fn end(&self, settings: &Settings, img: RgbaImage) {
        match self {
            End::Save => {
                img.save(
                    DialogBuilder::file()
                        .set_location(&settings.path)
                        .add_filter("PNG Image", ["png"])
                        .add_filter("JPEG Image", ["jpg", "jpeg"])
                        .set_filename(Local::now().format(&settings.format))
                        .save_single_file()
                        .show()
                        .unwrap()
                        .unwrap(),
                )
                .unwrap();
            }
            End::Copy => {
                let image_data = ImageData {
                    width: img.width() as usize,
                    height: img.height() as usize,
                    bytes: Cow::from(img.into_vec()),
                };

                let mut clipboard = Clipboard::new().unwrap();

                if cfg!(target_os = "linux") {
                    clipboard.set().wait().image(image_data).unwrap();
                } else {
                    clipboard.set_image(image_data).unwrap();
                }
            }
        }
    }
}
