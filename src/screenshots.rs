use grim_rs::{CaptureParameters, Grim};
use image::RgbaImage;

#[derive(Clone)]
pub struct MonitorData {
    pub name: String,
}

pub fn screenshots() -> Vec<(MonitorData, RgbaImage)> {
    hypr_screenshots()
}

pub fn get_outputs() -> Vec<String> {
    hypr_get_outputs()
}

pub fn hypr_get_outputs() -> Vec<String> {
    Grim::new().unwrap().get_outputs().unwrap().iter().map(|outputs| outputs.name().to_string()).collect()
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
            },
            RgbaImage::from_raw(img.width(), img.height(), img.data().to_vec()).unwrap(),
        )
    })
    .collect::<Vec<_>>()
}
