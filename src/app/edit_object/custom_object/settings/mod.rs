pub mod serde_help;

use serde::{Deserialize, Serialize};

use crate::app::{
    edit_object::{
        EditObjectSettings,
        custom_object::{
            CustomObject, data_type::DataType, icon::Icon, param::Param, points::PointsFormat,
        },
    },
    settings::edit_object_base_settings::EditObjectBaseSettingsFromShader,
};

#[derive(Clone, Serialize, Deserialize)]
pub struct CustomObjectSettings {
    name: String,
    icon: Icon,
    params: Vec<Param>,
    shader: String,
    points_format: Option<PointsFormat>,
}

#[derive(Clone)]
pub struct CustomIndexedObjectSettings {
    type_id: u32,
    name: String,
    icon: Icon,
    params: Vec<Param>,
    shader: String,
    points_format: Option<PointsFormat>,
}

impl CustomObjectSettings {
    pub fn new(
        name: String,
        icon: Icon,
        params: Vec<Param>,
        shader: String,
        points_format: Option<PointsFormat>,
    ) -> Self {
        Self {
            name,
            icon,
            params,
            shader,
            points_format,
        }
    }
}

impl CustomIndexedObjectSettings {
    pub fn new(
        type_id: u32,
        name: String,
        icon: Icon,
        params: Vec<Param>,
        shader: String,
        points_format: Option<PointsFormat>,
    ) -> Self {
        Self {
            type_id,
            name,
            icon,
            params,
            shader,
            points_format,
        }
    }
}

impl EditObjectSettings for CustomIndexedObjectSettings {
    type Object = CustomObject;

    fn get_icon(&self) -> iced::Element<'_, ()> {
        self.icon.get_icon()
    }

    fn get_name(&self) -> String {
        self.name.clone()
    }

    fn get_shader(&self) -> String {
        let name = &self.name;
        let shader = &self.shader;
        let params = self
            .params
            .iter()
            .map(DataType::get_str_field)
            .chain(self.points_format.as_ref().map(DataType::get_str_field))
            .collect::<Vec<_>>()
            .join("\n    ");
        let init_params = Param::indexing_params(&self.params)
            .iter()
            .map(|(i, param)| param.get_str_init_field(*i))
            .chain(
                self.points_format
                    .as_ref()
                    .map(|points_format| points_format.get_str_init_field(0)),
            )
            .collect::<Vec<_>>()
            .join("\n    ");

        format!(
            r"
struct Data_{name} {{
    base_settings: EditObjectBaseSettings,
    {params}
}}

fn get_data_{name}(objects: CustomObject) -> Data_{name} {{
    let channel_index = objects.channel_index;
    return Data_{name} (
        objects.base_settings,
        {init_params}
    );
}}
        
fn draw_{name}(pixel_color: vec4<f32>, pixel_pos: vec2<f32>, data: Data_{name}) -> vec4<f32> {{
{shader}
}}
"
        )
    }

    fn get_object(
        &self,
        i: usize,
        edit_object_base_settings: &EditObjectBaseSettingsFromShader,
    ) -> CustomObject {
        CustomObject {
            type_id: self.type_id,
            i,
            edit_object_base_settings: *edit_object_base_settings,
            points_data: self.points_format.clone().map(Into::into),
            params: self.params.clone(),
        }
    }
}
