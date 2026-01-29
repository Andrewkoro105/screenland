use chrono::Local;
use image::{RgbaImage, imageops::crop};
use native_dialog::DialogBuilder;

use crate::{
    app::{Screenland, selection::Selection},
    screenshots::full_screenshot,
};

#[derive(Clone)]
pub enum End {
    Save,
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
    pub fn end(&self, img: RgbaImage) {
        match self {
            End::Save => {
                img.save(
                    DialogBuilder::file()
                        .set_location("~/Desktop")
                        .add_filter("PNG Image", ["png"])
                        .add_filter("JPEG Image", ["jpg", "jpeg"])
                        .set_filename(Local::now().format("screenshot_%Y-%m-%d_%H:%M:%S.png"))
                        .save_single_file()
                        .show()
                        .unwrap()
                        .unwrap(),
                )
                .unwrap();
            }
        }
    }
}
