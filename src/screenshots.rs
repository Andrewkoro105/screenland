use grim_rs::{CaptureParameters, Grim};
use image::RgbaImage;

#[derive(Clone, Debug)]
pub struct MonitorData {
    pub name: String,
    pub pos: (i32, i32),
    pub size: (u32, u32),
}

pub fn screenshots() -> Vec<(MonitorData, RgbaImage)> {
    hypr_screenshots()
}

pub fn full_screenshot() -> RgbaImage {
    hypr_full_screenshots()
}

pub fn hypr_full_screenshots() -> RgbaImage {
    let result = Grim::new().unwrap().capture_all().unwrap();
    RgbaImage::from_raw(result.width(), result.height(), result.data().to_vec()).unwrap()
}

pub fn get_outputs() -> Vec<MonitorData> {
    hypr_get_outputs()
}

pub fn hypr_get_outputs() -> Vec<MonitorData> {
    Grim::new()
        .unwrap()
        .get_outputs()
        .unwrap()
        .iter()
        .map(|outputs| MonitorData{
            name: outputs.name().to_string(),
            pos: (outputs.geometry().x(), outputs.geometry().y()),
            size: (outputs.geometry().width() as _, outputs.geometry().height() as _),
        })
        .collect()
}

pub fn hypr_screenshots() -> Vec<(MonitorData, RgbaImage)> {
    let mut grim = Grim::new().unwrap();
    let outputs = grim.get_outputs().unwrap();

    grim.capture_outputs(
        outputs
            .iter()
            .map(|output| CaptureParameters::new(output.name()))
            .collect(),
    )
    .unwrap()
    .into_outputs()
    .into_iter()
    .collect::<Vec<(_, _)>>()
    .iter()
    .map(|(output_str, img)| {
        let output = outputs
            .iter()
            .find(|output| output.name() == output_str)
            .unwrap()
            .clone();
        (
            MonitorData {
                name: output.name().to_string(),
                pos: (output.geometry().x(), output.geometry().y()),
                size: (
                    output.geometry().width().unsigned_abs(),
                    output.geometry().height().unsigned_abs(),
                ),
            },
            RgbaImage::from_raw(img.width(), img.height(), img.data().to_vec()).unwrap(),
        )
    })
    .collect::<Vec<_>>()
}
