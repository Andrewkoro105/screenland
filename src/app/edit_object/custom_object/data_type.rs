pub trait DataType {
    fn get_name(&self) -> String;
    fn get_type_name(&self) -> String;

    fn get_str_field(&self) -> String {
        format!("{}: {},", self.get_name(), self.get_type_name())
    }

    fn get_str_init_field(&self, i: usize) -> String {
        let type_name = self.get_type_name().to_lowercase();
        format!("{type_name}_channel.data[channel_index.{type_name}_index + {i}],",)
    }
}
