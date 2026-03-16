use std::str::FromStr;

use clap::Parser;
use grim_rs::{CaptureParameters, Grim};
use image::RgbaImage;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug)]
pub struct MonitorData {
    pub name: String,
    pub pos: (i32, i32),
    pub size: (u32, u32),
}

#[derive(Clone, Serialize, Deserialize, Parser)]
pub struct ColorFormat {
    pub r: usize,
    pub g: usize,
    pub b: usize,
    pub a: usize,
}

impl FromStr for ColorFormat {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() == 4 {
            let ints = s.chars().map(|ch| ch.to_digit(10)).collect::<Vec<_>>();
            let result = Self {
                r: ints[0].ok_or("The red channel for ColorFormat can only be parsed from numbers")?
                    as _,
                g: ints[1].ok_or("The green channel for ColorFormat can only be parsed from numbers")?
                    as _,
                b: ints[2].ok_or("The blue channel for ColorFormat can only be parsed from numbers")?
                    as _,
                a: ints[3].ok_or("The alpha channel for ColorFormat can only be parsed from numbers")?
                    as _,
            };

            if result.r > 3 {
                Err("The red channel for ColorFormat can only be parsed from numbers between 0 and 3".into())
            } else if result.g > 3 {
                Err("The green channel for ColorFormat can only be parsed from numbers between 0 and 3".into())
            } else if result.b > 3 {
                Err("The blue channel for ColorFormat can only be parsed from numbers between 0 and 3".into())
            } else if result.a > 3 {
                Err("The alpha channel for ColorFormat can only be parsed from numbers between 0 and 3".into())
            } else {
                Ok(result)
            }
        } else {
            Err(format!(
                "ColorFormat cannot be parsed from a string of length {}",
                s.len()
            ))
        }
    }
}

pub fn screenshots(color_format: &ColorFormat) -> Vec<(MonitorData, RgbaImage)> {
    hypr_screenshots(color_format)
}

pub fn full_screenshot(color_format: &ColorFormat) -> RgbaImage {
    hypr_full_screenshots(color_format)
}

pub fn hypr_full_screenshots(color_format: &ColorFormat) -> RgbaImage {
    let result = Grim::new().unwrap().capture_all().unwrap();
    let width = result.width();
    let height = result.height();
    let data = result.data();

    let mut rgba_data = Vec::with_capacity(data.len());
    for chunk in data.chunks_exact(4) {
        rgba_data.push(chunk[color_format.r]);
        rgba_data.push(chunk[color_format.g]);
        rgba_data.push(chunk[color_format.b]);
        rgba_data.push(chunk[color_format.a]);
    }

    RgbaImage::from_raw(width, height, rgba_data).unwrap()
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
        .map(|outputs| MonitorData {
            name: outputs.name().to_string(),
            pos: (outputs.geometry().x(), outputs.geometry().y()),
            size: (
                outputs.geometry().width() as _,
                outputs.geometry().height() as _,
            ),
        })
        .collect()
}

pub fn hypr_screenshots(color_format: &ColorFormat) -> Vec<(MonitorData, RgbaImage)> {
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
            {
                let width = img.width();
                let height = img.height();
                let data = img.data();

                let mut rgba_data = Vec::with_capacity(data.len());
                for chunk in data.chunks_exact(4) {
                    rgba_data.push(chunk[color_format.r]);
                    rgba_data.push(chunk[color_format.g]);
                    rgba_data.push(chunk[color_format.b]);
                    rgba_data.push(chunk[color_format.a]);
                }

                RgbaImage::from_raw(width, height, rgba_data).unwrap()
            },
        )
    })
    .collect::<Vec<_>>()
}
