use crate::data_description::{DataDescription, Value};
use crate::util;

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
            util::render_table(std::iter::once(data_description.render_table_row())),
            data_description.render_references(&data_description.hex_address_string)
        )
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_render_node() {
        let target = 8u8;
        let target_address_string = util::address_of(&target);
        assert_eq!((&target).render_node(), format!("  node [shape=plaintext]\n    \"{0}\" [label=<<TABLE BORDER=\"0\" CELLBORDER=\"1\" CELLSPACING=\"0\"><TR><TD PORT=\"{0}-address\"><I>{0}</I></TD><TD PORT=\"{0}-type\"><B>u8</B></TD><TD PORT=\"{0}-value\">8</TD></TR></TABLE>>];\n    ", target_address_string))
    }
}
