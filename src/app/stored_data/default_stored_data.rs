use iced_helper::ui_elements::num_input::NumInput;
use serde_saphyr::LitString;
use maplit::hashmap;

use crate::app::{
        edit_object::custom_object::{
            icon::Icon,
            param::{Param, ShaderType},
            points::PointsFormat,
            settings::{
                CustomObjectSettings,
                serde_help::add_type_id,
            },
        },
        stored_data::{StoredData, edit_object_base_settings::EditObjectBaseSettings, path_system::PathSystem, settings::Settings
    },
};

impl StoredData {
    pub fn new(path_system: PathSystem) -> Self {
        Self {
            path_system,
            settings: Settings::default(),
            edit_object_base_settings: EditObjectBaseSettings::default(),
            custom_objects: add_type_id(vec![
                CustomObjectSettings::new(
                    "rectangle".into(),
                    Icon::Name("square".into()),
                    vec![],
                    hashmap! {},
                    LitString(r"
    if in_cube(pixel_pos, data.cube.start, data.cube.end) && !in_cube(pixel_pos, data.cube.start + data.base_settings.size, data.cube.end - data.base_settings.size) {
        return vec4(mix(pixel_color.rgb, data.base_settings.color.rgb, data.base_settings.color.a), pixel_color.a);
    } else {
        return pixel_color;
    }".into()),
                    Some(PointsFormat::Cube),
                ), 
                CustomObjectSettings::new(
                    "circle".into(),
                    Icon::Name("circle".into()),
                    vec![],
                    hashmap! {
                        "in".to_string() => LitString(r"
      (pos: vec2<f32>, center: vec2<f32>, radius: vec2<f32>) -> bool {
          return pow(pos.x - center.x, 2) / pow(radius.x, 2) + pow(pos.y - center.y, 2) / pow(radius.y, 2) < 1;
      }".into()),
                    },
                    LitString(r"
    let result_color = vec4(mix(pixel_color.rgb, data.base_settings.color.rgb, data.base_settings.color.a), pixel_color.a);
    let radius = (data.cube.end - data.cube.start) / 2;
    let center = data.cube.start + radius;
    if circle_in(pixel_pos, center, radius) && !circle_in(pixel_pos, center, radius - data.base_settings.size) {
        return result_color;
    } else {
        return pixel_color;
    }".into()),
                    Some(PointsFormat::Cube),
                ),
                CustomObjectSettings::new(
                    "line".into(),
                    Icon::SolidName("arrow-left".into()),
                    vec![
                        Param::new("head_type".to_string(), ShaderType::Enum {
                            current: 0,
                            enums: vec![
                                ("arrow".to_string(), Icon::SolidName("arrow-left".to_string())),
                                ("none".to_string(), Icon::Name("circle".to_string())),
                            ]
                        })
                    ],
                    hashmap! {
                        "point_in_triangle".to_string() => LitString(r"
    (p: vec2<f32>, v0: vec2<f32>, v1: vec2<f32>, v2: vec2<f32>) -> bool {
        let e0 = v1 - v0;
        let e1 = v2 - v1;
        let e2 = v0 - v2;
        let d0 = line_cross2d(e0, p - v0);
        let d1 = line_cross2d(e1, p - v1);
        let d2 = line_cross2d(e2, p - v2);
    
        let has_neg = (d0 < 0.0) || (d1 < 0.0) || (d2 < 0.0);
        let has_pos = (d0 > 0.0) || (d1 > 0.0) || (d2 > 0.0);
        return !(has_neg && has_pos);
    }".into()),
                        "is_arrow".to_string() => LitString(r"
    (pixel_color: vec4<f32>, pixel_pos: vec2<f32>, base_settings: EditObjectBaseSettings, tip: vec2<f32>, tail: vec2<f32>) -> bool {
        let LINE_WIDTH = base_settings.size;
        var head_len = base_settings.size * 5;
        let head_wid = base_settings.size * 3;
        let p = pixel_pos.xy;
        let dist_tail_tip = distance(tip, tail);
    
        if dist_tail_tip < 0.5 {
            return false;
        }
    
        let dir = normalize(tip - tail);
        let perp = vec2(-dir.y, dir.x);
    
        head_len = min(head_len, dist_tail_tip);
    
        let base_center = tip - dir * head_len;
        let half_w = head_wid * 0.5;
        let base_left = base_center + perp * half_w;
        let base_right = base_center - perp * half_w;
    
        let line_threshold_sq = line_sq(base_settings.size * 0.5);
        let dist_to_line_sq = line_dist_sq_to_segment(p, tail, base_center);
        let on_line = dist_to_line_sq <= line_threshold_sq;
    
        let on_head = line_point_in_triangle(p, tip, base_left, base_right);
    
        if on_line || on_head {
            return true;
        } else {
            return false;
        }
    }".into()),
                        "sq".to_string() => LitString(r"(x: f32) -> f32 { return x * x; }".into()),
                        "cross2d".to_string() => LitString(r"
    (u: vec2<f32>, v: vec2<f32>) -> f32 {
        return u.x * v.y - u.y * v.x;
    }".into()),
                        "dist_sq_to_segment".to_string() => LitString(r"
    (p: vec2<f32>, a: vec2<f32>, b: vec2<f32>) -> f32 {
        let ab = b - a;
        let ap = p - a;
        let t = dot(ap, ab) / max(dot(ab, ab), 1e-6);
        if t <= 0.0 {
            return dot(ap, ap);
        } else if t >= 1.0 {
            let bp = p - b;
            return dot(bp, bp);
        } else {
            let proj = a + ab * t;
            let diff = p - proj;
            return dot(diff, diff);
        }
    }".into()),
                        "distance_to_segment".to_string() => LitString(r"
    (p: vec2<f32>, a: vec2<f32>, b: vec2<f32>) -> f32 {
        let ab = b - a;
        let ap = p - a;
        let t = dot(ap, ab) / dot(ab, ab);
        let t_clamped = clamp(t, 0.0, 1.0);
        let closest = a + ab * t_clamped;
        return distance(p, closest);
    }".into()),
                    },
                    LitString(r"
    if data.bezier_points.size != 0 {
        let result_color = vec4(mix(pixel_color.rgb, data.base_settings.color.rgb, data.base_settings.color.a), pixel_color.a);
        var points_offset: u32 = 0;
        if data.bezier_points.size >= 2 {
            switch (data.head_type.index) {
                case line_arrow: {
                    let is_arrow = line_is_arrow(
                        pixel_color,
                        pixel_pos,
                        data.base_settings,
                        bezier_points_channel.data[data.bezier_points.current],
                        bezier_points_channel.data[data.bezier_points.current + 1]
                    );
    
                    if is_arrow {
                        return result_color;
                    } else {
                        points_offset = 1;    
                    }
                }
                case line_none: {}
                default: {}
            }
        }
    
        let radius = data.base_settings.size / 2;
        for(var i = data.bezier_points.current + points_offset; i < data.bezier_points.current + data.bezier_points.size - 1; i = i + 1u) {
            if distance(bezier_points_channel.data[i], pixel_pos) < radius {
                return result_color;
            }
    
            let thickness = radius;
            let dist = line_distance_to_segment(
                pixel_pos, 
                bezier_points_channel.data[i],
                bezier_points_channel.data[i + 1], 
            );
    
            if dist < thickness {
                return result_color;
            }
        }
    
        if distance(bezier_points_channel.data[data.bezier_points.current + data.bezier_points.size - 1], pixel_pos) < radius {
            return result_color;
        }
    }
    return pixel_color;".into()),
                    Some(PointsFormat::BezierPoints),
                ),
                CustomObjectSettings::new(
                    "blur".into(),
                    Icon::SolidName("water".into()),
                    vec![
                        Param::new("blur_factor".to_string(), ShaderType::U32 { num_input: NumInput::new(3) })
                    ],
                    hashmap! {},
                    LitString(r"
    if in_cube(pixel_pos, data.cube.start, data.cube.end) {
        var blur_factor = i32(data.blur_factor);
        if blur_factor > 20 {
            blur_factor = 20;
        }
        let texSize = vec2<f32>(textureDimensions(my_texture));
        let uv = pixel_pos.xy / texSize;
        let offset = vec2<f32>(1.0) / texSize;
    
        var blurColor = vec4<f32>(0.0);
        for (var i = -blur_factor; i <= blur_factor; i++) {
            for (var j = -blur_factor; j <= blur_factor; j++) {
                let offset_pos = vec2<f32>(f32(i), f32(j));
                let sampleUV = uv + offset_pos * offset;
                blurColor += draw_custom_objects_for_blur(textureSample(my_texture, my_sampler, sampleUV), pixel_pos + offset_pos, index);
            }
        }
        blurColor /= pow((f32(blur_factor) * 2) + 1, 2);
        return blurColor;
    } else {
      return pixel_color;
    }".into()),
                    Some(PointsFormat::Cube),
                ), 
            ]),
        }
    }
}