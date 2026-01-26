struct Uniforms {
    resolution: vec2<f32>,
    mouse: vec2<f32>,
    monitor_pos: vec2<f32>,
};

@group(0) @binding(0)
var<uniform> uniforms: Uniforms;

@group(0) @binding(1)
var my_texture: texture_2d<f32>;

@group(0) @binding(2)
var my_sampler: sampler;

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
    let uv = (pixel_pos.xy + uniforms.monitor_pos) / vec2<f32>(textureDimensions(my_texture));
    let texture_color = textureSample(my_texture, my_sampler, uv);
    
    return texture_color;
}