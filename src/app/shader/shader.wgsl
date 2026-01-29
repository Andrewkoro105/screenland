const blackout = 0.7;

struct BaseData {
    resolution: vec2<f32>,
    monitor_pos: vec2<f32>,
};

struct Selection {
    start: vec2<f32>,
    end: vec2<f32>,
}

@group(0) @binding(0)
var my_texture: texture_2d<f32>;

@group(0) @binding(1)
var my_sampler: sampler;

@group(1) @binding(0)
var<uniform> base_data: BaseData;

@group(1) @binding(1)
var<uniform> selection: Selection;

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
    var texture_color = textureSample(my_texture, my_sampler, uv) * selection_effect(screen_pixel_pos);

    return texture_color;
}

fn selection_effect(screen_pixel_pos: vec2<f32>) -> vec4<f32> {
    if !(selection.end.x > screen_pixel_pos.x && screen_pixel_pos.x > selection.start.x && selection.end.y > screen_pixel_pos.y && screen_pixel_pos.y > selection.start.y) {
        return vec4(blackout, blackout, blackout, 0.);
    } else {
        return vec4(1.,1.,1.,1.);
    }
}