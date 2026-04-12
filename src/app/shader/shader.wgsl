const blackout = 0.7;
const select_border_size = 3.;
const base_color = vec4(0., 0., 1., 1.);

struct EditObjectBaseSettings {
    color: vec4<f32>,
    size: f32,
}

struct BaseData {
    resolution: vec2<f32>,
    monitor_pos: vec2<f32>,
};

struct Cube {
    start: vec2<f32>,
    end: vec2<f32>,
    start_touch: vec2<f32>,
    touched: u32,
    init: u32,
}

struct Selection {
    cube: Cube
}

struct UIPoint {
    pos: vec2<f32>,
    size: f32,
}

struct Iter {
    current: u32,
    size: u32,
}

//{ChannelIndex}

struct CustomObject {
    base_settings: EditObjectBaseSettings,
    custom_object_type: u32,
    channel_index: ChannelIndex
}

@group(0) @binding(0)
var my_texture: texture_2d<f32>;

@group(0) @binding(1)
var my_sampler: sampler;

@group(1) @binding(0)
var<uniform> base_data: BaseData;

@group(1) @binding(1)
var<uniform> selection: Selection;

//{STORAGE_BUFFERS}

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
    result = draw_custom_objects(result, screen_pixel_pos, custom_objects.len);
    result = selection_effect(result, screen_pixel_pos);
    result = ui_points(result, screen_pixel_pos);

    return result;
}

fn in_cube(screen_pixel_pos: vec2<f32>, start: vec2<f32>, end: vec2<f32>) -> bool {
    return end.x > screen_pixel_pos.x && screen_pixel_pos.x > start.x && end.y > screen_pixel_pos.y && screen_pixel_pos.y > start.y;
}

fn selection_effect(result: vec4<f32>, screen_pixel_pos: vec2<f32>) -> vec4<f32> {
    let in_border = in_cube(
        screen_pixel_pos,
        selection.cube.start - vec2<f32>(select_border_size, select_border_size),
        selection.cube.end + vec2<f32>(select_border_size, select_border_size));

    if !in_border {
        return result * vec4(blackout, blackout, blackout, 1.);
    } else if !in_cube(screen_pixel_pos, selection.cube.start, selection.cube.end) {
        return base_color;
    }
    return result;
}

fn ui_points(result: vec4<f32>, screen_pixel_pos: vec2<f32>) -> vec4<f32> {
    for (var i = 0u; i < points.len; i = i + 1u) {
        let size = points.data[i].size;
        if size != 0 {
            let r = sqrt(pow((screen_pixel_pos.x - points.data[i].pos.x), 2.) + pow((screen_pixel_pos.y - points.data[i].pos.y), 2.));
            if r < size {
                return base_color;
            }
        }
    }
    return result;
}

fn error(screen_pixel_pos: vec2<f32>) -> vec4<f32> {
    let uv = screen_pixel_pos / vec2<f32>(textureDimensions(my_texture));

    let thickness = 0.08;
    let space = 0.1;

    let left = 0.2;
    let right_vert = 0.3;
    
    let top = 0.5;
    let top_end = top + thickness;
    let middle = top_end + space;
    let middle_end = middle + thickness;
    let bottom = middle_end + space;
    let bottom_end = bottom + thickness;
    
    let right_horiz = 0.8;
    
    let vertical = (uv.x >= left && uv.x <= right_vert) && (uv.y >= top && uv.y <= bottom_end);
    
    let top_horiz = (uv.y >= top && uv.y <= top_end) && (uv.x >= left && uv.x <= right_horiz);
    let middle_horiz = (uv.y >= middle && uv.y <= middle_end) && (uv.x >= left && uv.x <= right_horiz);
    let bottom_horiz = (uv.y >= bottom && uv.y <= bottom_end) && (uv.x >= left && uv.x <= right_horiz);
    
    let is_e = vertical || top_horiz || middle_horiz || bottom_horiz;
    
    let color = select(vec3(0.0, 0.0, 0.0), vec3(1.0, 0.0, 0.0), is_e);
    return vec4(color, 1.0);
}

fn draw_custom_objects(input: vec4<f32>, screen_pixel_pos: vec2<f32>, number: u32) -> vec4<f32> {
    var result = input;
    for(var i = 0u; i < number; i = i + 1u) {
        let object = custom_objects.data[i];
        switch (object.custom_object_type) {
            //{DRAW_CUSTOM_OBJECTS}
            default: {return error(screen_pixel_pos);}
        }
    }
    return result;
}

//{DRAW_CUSTOM_OBJECTS_FOR_RECURSION}

//{DRAW_CUSTOM_OBJECTS_FUNCTION}