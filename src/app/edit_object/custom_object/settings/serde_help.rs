//! Utility functions for `serde` that help load `CustomIndexedObjectSettings`

use serde::{Deserialize, Serialize, Serializer};
use serde_saphyr::LitString;

use crate::app::edit_object::custom_object::settings::{
    CustomIndexedObjectSettings, CustomObjectSettings,
};

pub fn add_type_id(value: Vec<CustomObjectSettings>) -> Vec<CustomIndexedObjectSettings> {
    value
        .into_iter()
        .enumerate()
        .map(|(i, object)| {
            CustomIndexedObjectSettings::new(
                i as _,
                object.name,
                object.icon,
                object.params,
                object.functions.into_iter().map(|(a, b)| (a, b.to_string())).collect(),
                object.shader.to_string(),
                object.points_format,
            )
        })
        .collect()
}

pub fn add_type_id_deserialize<'de, D>(
    deserializer: D,
) -> Result<Vec<CustomIndexedObjectSettings>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let intermediate = Vec::<CustomObjectSettings>::deserialize(deserializer)?;
    Ok(add_type_id(intermediate))
}

pub fn remove_type_id(value: Vec<CustomIndexedObjectSettings>) -> Vec<CustomObjectSettings> {
    value
        .into_iter()
        .map(|object| {
            CustomObjectSettings::new(
                object.name,
                object.icon,
                object.params,
                object.functions.into_iter().map(|(a, b)| (a, LitString(b))).collect(),
                LitString(object.shader),
                object.points_format,
            )
        })
        .collect()
}

pub fn remove_type_id_serialize<S>(
    value: &Vec<CustomIndexedObjectSettings>,
    serializer: S,
) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    remove_type_id(value.clone()).serialize(serializer)
}
