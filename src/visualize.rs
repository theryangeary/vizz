use crate::data_description::{DataDescription, Value};
use crate::util::*;

pub trait Visualize: Sized {
    fn data(&self) -> Option<Value> {
        None
    }

    fn reference(&self) -> Option<String> {
        None
    }

    fn associated_data(&self) -> Option<Vec<DataDescription>> {
        None
    }

    fn render_node(&self) -> String {
        let data_description = DataDescription::from(self);
        format!(
            r#"  node [shape=plaintext]
    "{}" [label=<{}>];
    {}"#,
            data_description.hex_address_string,
            render_table(std::iter::once(data_description.render_table_row())),
            data_description.render_references(&data_description.hex_address_string)
        )
    }
}
