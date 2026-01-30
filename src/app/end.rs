use chrono::Local;
use image::{RgbaImage, imageops::crop};
use native_dialog::DialogBuilder;
use arboard::{Clipboard, ImageData};
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

use crate::{
    app::{Screenland, selection::Selection, settings::Settings},
    screenshots::full_screenshot,
};

#[derive(Clone, Serialize, Deserialize)]
pub enum End {
    Save,
    Copy,
}

impl Screenland {
    pub fn screenshot(selection: Selection) -> RgbaImage {
        let mut screenshot = full_screenshot();
        let select = selection.normalize();
        crop(
            &mut screenshot,
            select.start.x as _,
            select.start.y as _,
            (select.end.x - select.start.x) as _,
            (select.end.y - select.start.y) as _,
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

                clipboard.set_image(image_data).unwrap();
            }
        }
    }
}
