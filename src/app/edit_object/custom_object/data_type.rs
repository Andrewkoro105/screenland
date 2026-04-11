use heck::ToSnakeCase;
use std::hash::Hash;

pub trait DataType: Sized + Hash + PartialEq + Eq {
    fn get_type_name(&self) -> String {
        if self.is_iter() {
            "Iter".into()
        } else {
            self.base_get_type_name()
        }
    }

    fn base_get_type_name(&self) -> String;

    fn get_name(&self) -> String {
        self.get_type_name().to_snake_case()
    }

    fn get_str_field(&self) -> String {
        format!("{}: {},", self.get_name(), self.get_type_name())
    }

    fn get_str_init_field(&self, i: usize) -> String {
        let type_name = if self.is_iter() {
            self.get_name()
        } else {
            self.get_type_name().to_snake_case()
        };
        if self.is_iter() {
            format!(
                "Iter(channel_index.{type_name}_index, {type_name}_len_channel.data[channel_index.{type_name}_len_index + {i}]),",
            )
        } else {
            format!("{type_name}_channel.data[channel_index.{type_name}_index + {i}],",)
        }
    }

    fn is_iter(&self) -> bool {
        false
    }
}
