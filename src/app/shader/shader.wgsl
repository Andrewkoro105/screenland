const blackout = 0.7;
const select_border_size = 3.;
const base_color = vec4(0., 0., 1., 1.);

struct BaseData {
    resolution: vec2<f32>,
    monitor_pos: vec2<f32>,
};

struct Selection {
    start: vec2<f32>,
    end: vec2<f32>,
}

struct UIPoint {
    pos: vec2<f32>,
    size: f32,
}

struct Points {
    size: u32,
    points: array<UIPoint>,
}

@group(0) @binding(0)
var my_texture: texture_2d<f32>;

@group(0) @binding(1)
var my_sampler: sampler;

@group(1) @binding(0)
var<uniform> base_data: BaseData;

@group(1) @binding(1)
var<uniform> selection: Selection;

@group(1) @binding(2)
var<storage> points: Points;

@vertex
fn vs_main(@builtin(vertex_index) vertex_index: u32) -> @builtin(position) vec4<f32> {
    if (vertex_index == 0) {
        return vec4<f32>(-3., -1., 0.0, 1.0);
    } else if (vertex_index == 1) {
        return vec4<f32>(1., 3., 0.0, 1.0);
    } else {
        return vec4<f32>(1., -1., 0.0, 1.0);
    }
}

@fragment
fn fs_main(@builtin(position) pixel_pos: vec4<f32>) -> @location(0) vec4<f32> {
    let screen_pixel_pos = pixel_pos.xy + base_data.monitor_pos;
    let uv = screen_pixel_pos / vec2<f32>(textureDimensions(my_texture));
    var result = textureSample(my_texture, my_sampler, uv);
    result = selection_effect(result, screen_pixel_pos);
    result = ui_points(result, screen_pixel_pos);

    return result;
}

fn in_selection(screen_pixel_pos: vec2<f32>, start: vec2<f32>, end: vec2<f32>) -> bool {
    return end.x > screen_pixel_pos.x && screen_pixel_pos.x > start.x && end.y > screen_pixel_pos.y && screen_pixel_pos.y > start.y;
}

fn selection_effect(result: vec4<f32>, screen_pixel_pos: vec2<f32>) -> vec4<f32> {
    let in_border = in_selection(
        screen_pixel_pos,
        selection.start - vec2<f32>(select_border_size, select_border_size),
        selection.end + vec2<f32>(select_border_size, select_border_size));

    if !in_border {
        return result * vec4(blackout, blackout, blackout, 0.);
    } else if !in_selection(screen_pixel_pos, selection.start, selection.end) {
        return base_color;
    }
    return result;
}

fn ui_points(result: vec4<f32>, screen_pixel_pos: vec2<f32>) -> vec4<f32> {
    for (var i = 0u; i < arrayLength(&points.points); i = i + 1u) {
        let r = sqrt(pow((screen_pixel_pos.x - points.points[i].pos.x), 2.) + pow((screen_pixel_pos.y - points.points[i].pos.y), 2.));
        if r < points.points[i].size {
            return base_color;
        }
    }
    return result;
}