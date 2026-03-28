use clap::Parser;

use crate::{Args, app::{
    edit_object::{EditObjectSettings, custom_object::settings::CustomIndexedObjectSettings},
    settings::Settings,
    shader::pipeline::{base_storage_buffers::GetShader, edit_bg::get_storage_buffers_data},
}};

pub fn get_shader(storage_buffers: Option<Vec<&dyn GetShader>>) -> String {
    let args = Args::parse();
    let custom_objects = Settings::load(Some(args.clone()), None, None).custom_objects;
    let new_storage_buffers = get_storage_buffers_data();
    let result = include_str!("shader.wgsl")
        .to_string()
        .replace(
            "//{STORAGE_BUFFERS}",
            &[
                "// The order of `storage_buffers` changes with every launch within a single `group` (this is taken into account when data is passed to the shader)", 
                &storage_buffers
                    .unwrap_or_else(|| vec![&new_storage_buffers])
                    .iter()
                    .map(|buff| buff.get_shader())
                    .collect::<Vec<_>>()
                    .join("\n")
            ].join("\n")
        )
        .replace(
            "//{DRAW_CUSTOM_OBJECTS}",
            &custom_objects
                .iter()
                .enumerate()
                .map(|(i, custom_object)| {
                    let name: String = custom_object.get_name();
                    format!("case {i}: {{result = draw_{name}(result, screen_pixel_pos, get_data_{name}(object));}}")
                })
                .collect::<Vec<_>>()
                .join("\t\t\t\n")
        )
        .replace(
            "//{DRAW_CUSTOM_OBJECTS_FUNCTION}",
            &custom_objects
                .iter()
                .map(CustomIndexedObjectSettings::get_shader)
                .collect::<Vec<_>>()
                .join("\n"),
        );

    if args.output_shader_and_run {
        println!("```wgsl\n{result}\n```");
    }

    result
}
