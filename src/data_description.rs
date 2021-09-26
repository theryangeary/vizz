use crate::util;
use crate::Visualize;

#[derive(strum_macros::ToString, Debug, Clone)]
/// The value of a [Visualize] implementer
pub enum Value {
    /// The implementer owns this data and the data will appear as this string
    Owned(String),
    /// The implementer references this data and there will be a graph edge from this reference to the
    /// referenced data
    ///
    /// The string contains a hex value starting with `0x`, which is the memory address of the
    /// referenced data
    Referenced(String),
}

#[readonly::make]
#[derive(Default, Debug, Clone)]
/// The data needed to generate a graph node for a data structure
pub struct DataDescription {
    /// The label for this piece of data
    ///
    /// Typically this is the variable name or field name.
    ///
    /// Leaving this field as [None] will result in no TD box rendered for this field.
    pub label_string: Option<String>,
    /// The memory location of this data, as a hex string prefaced with `0x`
    pub hex_address_string: String,
    /// The fully qualified type of this data
    pub type_string: String,
    /// The value of this data
    ///
    /// This may be:
    ///
    /// 1. some owned data, such as a primitive or an enum value, which will be rendered
    /// as a string.
    /// 1. some referenced data, which will be rendered as an arrow pointing to the referenced
    ///    data.
    /// 1. nothing, as would make sense for a struct, where the real data is the fields, which are
    ///    represented in [DataDescription::associated_data_descriptions]
    pub value: Option<Value>,
    /// The [DataDescription]s owned by this data
    ///
    /// These will be rendered as part of this data.
    pub associated_data_descriptions: Option<Vec<DataDescription>>,
}

impl DataDescription {
    /// Add a label to this node
    ///
    /// Labels are generally optional but can be helpful for named structured data, like the
    /// fields of a struct. Labels are less likely to be used for tuple structs or tuple enums.
    pub fn with_label<T>(self, label_string: T) -> Self
    where
        T: Into<String>,
    {
        Self {
            label_string: Some(label_string.into()),
            ..self
        }
    }

    fn render_label_port(&self) -> String {
        util::render_label_port(&self.hex_address_string)
    }

    fn render_type_port(&self) -> String {
        util::render_type_port(&self.hex_address_string)
    }

    fn render_address_port(&self) -> String {
        util::render_address_port(&self.hex_address_string)
    }

    fn render_value_port(&self) -> String {
        util::render_value_port(&self.hex_address_string)
    }

    fn render_associated_data_port(&self) -> String {
        util::render_associated_data_port(&self.hex_address_string)
    }

    /// Create the DOT code to make an arrow from this node to another node
    ///
    /// The other node will need to be added to the graph separately from this call.
    ///
    /// The node_name is the name of the top level data this [DataDescription] lives inside of.
    fn render_reference(&self, node_name: &str) -> Option<String> {
        if let Some(Value::Referenced(target)) = &self.value {
            Some(format!(
                "\"{}\":\"{}\" -> \"{}\":\"{}\"\n",
                node_name,
                self.render_value_port(),
                target,
                util::render_address_port(target)
            ))
        } else {
            None
        }
    }

    /// Create the DOT code to make all arrows from data owned by this node to the data they
    /// reference
    ///
    /// The referenced nodes must be added to the graph separately.
    pub fn render_references(&self, node_name: &str) -> String {
        let this_reference = self.render_reference(node_name).unwrap_or_else(String::new);

        match &self.associated_data_descriptions {
            Some(associated_data_descriptions) => associated_data_descriptions
                .iter()
                .fold(this_reference, |acc, associated_data| {
                    acc + &associated_data.render_references(node_name)
                }),
            None => this_reference,
        }
    }

    fn render_label_table_data(&self) -> String {
        match &self.label_string {
            Some(label_string) => {
                format!(
                    r#"<TD PORT="{}">{}</TD>"#,
                    self.render_label_port(),
                    label_string
                )
            }
            None => String::new(),
        }
    }

    fn render_hex_address_table_data(&self) -> String {
        format!(
            r#"<TD PORT="{}"><I>{}</I></TD>"#,
            self.render_address_port(),
            self.hex_address_string,
        )
    }

    fn render_type_table_data(&self) -> String {
        format!(
            r#"<TD PORT="{}"><B>{}</B></TD>"#,
            self.render_type_port(),
            util::html_encode(&self.type_string)
        )
    }

    fn render_value_table_data(&self) -> String {
        match &self.value {
            Some(value) => format!(
                r#"<TD PORT="{}">{}</TD>"#,
                self.render_value_port(),
                match value {
                    Value::Owned(data) => util::html_encode(data),
                    Value::Referenced(_) => String::new(),
                }
            ),
            None => String::new(),
        }
    }

    fn render_associated_data_table(&self) -> String {
        match &self.associated_data_descriptions {
            Some(associated_data_descriptions) => format!(
                r#"<TD PORT="{}">{}</TD>"#,
                self.render_associated_data_port(),
                util::render_table(
                    associated_data_descriptions
                        .iter()
                        .map(DataDescription::render_table_row),
                )
            ),
            None => String::new(),
        }
    }

    /// Create the HTML table row for this data
    pub fn render_table_row(&self) -> String {
        format!(
            "<TR>{}{}{}{}{}</TR>",
            self.render_label_table_data(),
            self.render_hex_address_table_data(),
            self.render_type_table_data(),
            self.render_value_table_data(),
            self.render_associated_data_table(),
        )
    }
}

impl<T> From<&T> for DataDescription
where
    T: Visualize,
{
    fn from(t: &T) -> Self {
        Self {
            label_string: None,
            hex_address_string: util::address_of(t),
            type_string: util::type_of(t),
            value: t.data(),
            associated_data_descriptions: t.associated_data(),
        }
    }
}

impl Visualize for Value {
    fn data(&self) -> Option<Value> {
        Some(Value::Owned(self.to_string()))
    }

    fn associated_data(&self) -> Option<Vec<DataDescription>> {
        match self {
            Value::Owned(a) => Some(vec![DataDescription::from(a)]),
            Value::Referenced(a) => Some(vec![DataDescription::from(a)]),
        }
    }
}

impl Visualize for DataDescription {
    fn associated_data(&self) -> Option<Vec<DataDescription>> {
        Some(vec![
            DataDescription::from(&self.label_string).with_label("label_string"),
            DataDescription::from(&self.hex_address_string).with_label("hex_address_string"),
            DataDescription::from(&self.type_string).with_label("type_string"),
            DataDescription::from(&self.value).with_label("value"),
            DataDescription::from(&self.associated_data_descriptions)
                .with_label("associated_data_descriptions"),
        ])
    }
}

#[cfg(test)]
mod test {
    //! Tests for DataDescription functionality
    //!
    //! Any long and complicated dot graph strings have been manually verified through visual
    //! inspection.

    use super::*;

    #[test]
    fn test_label() {
        let label = "my_test_u8";
        let data_description = DataDescription::from(&8u8).with_label(label);
        assert_eq!(data_description.label_string.unwrap(), label);
    }

    #[test]
    fn test_render_table_row_primitive() {
        let hex_address_string = String::from("0x12345678");
        let label_string = Some(String::from("my_label"));
        let type_string = String::from("u8");
        let value = Some(Value::Owned(String::from("145")));
        let associated_data_descriptions = None;

        let data_description = DataDescription {
            hex_address_string,
            label_string,
            type_string,
            value,
            associated_data_descriptions,
        };

        assert_eq!(data_description.render_table_row(), "<TR><TD PORT=\"0x12345678-label\">my_label</TD><TD PORT=\"0x12345678-address\"><I>0x12345678</I></TD><TD PORT=\"0x12345678-type\"><B>u8</B></TD><TD PORT=\"0x12345678-value\">145</TD></TR>");
    }

    #[test]
    fn test_render_table_row_enum_plain() {
        let hex_address_string = String::from("0x12345678");
        let label_string = Some(String::from("my_label"));
        let type_string = String::from("foo::bar::Enum");
        let value = Some(Value::Owned(String::from("MyEnumVariant")));
        let associated_data_descriptions = None;

        let data_description = DataDescription {
            hex_address_string,
            label_string,
            type_string,
            value,
            associated_data_descriptions,
        };

        assert_eq!(data_description.render_table_row(), "<TR><TD PORT=\"0x12345678-label\">my_label</TD><TD PORT=\"0x12345678-address\"><I>0x12345678</I></TD><TD PORT=\"0x12345678-type\"><B>foo::bar::Enum</B></TD><TD PORT=\"0x12345678-value\">MyEnumVariant</TD></TR>");
    }

    #[test]
    fn test_render_table_row_enum_tuple() {
        let hex_address_string = String::from("0x12345678");
        let label_string = None;
        let type_string = String::from("u8");
        let value = Some(Value::Owned(String::from("178")));
        let associated_data_descriptions = None;

        let inner1 = DataDescription {
            hex_address_string,
            label_string,
            type_string,
            value,
            associated_data_descriptions,
        };

        let hex_address_string = String::from("0x12345678");
        let label_string = None;
        let type_string = String::from("alloc::string::String");
        let value = Some(Value::Owned(String::from("abcdefghi")));
        let associated_data_descriptions = None;

        let inner2 = DataDescription {
            hex_address_string,
            label_string,
            type_string,
            value,
            associated_data_descriptions,
        };

        let hex_address_string = String::from("0x12345678");
        let label_string = Some(String::from("my_label"));
        let type_string = String::from("foo::bar::Enum");
        let value = Some(Value::Owned(String::from("MyEnumVariant")));
        let associated_data_descriptions = Some(vec![inner1, inner2]);

        let data_description = DataDescription {
            hex_address_string,
            label_string,
            type_string,
            value,
            associated_data_descriptions,
        };

        assert_eq!(data_description.render_table_row(), "<TR><TD PORT=\"0x12345678-label\">my_label</TD><TD PORT=\"0x12345678-address\"><I>0x12345678</I></TD><TD PORT=\"0x12345678-type\"><B>foo::bar::Enum</B></TD><TD PORT=\"0x12345678-value\">MyEnumVariant</TD><TD PORT=\"0x12345678-associated-data\"><TABLE BORDER=\"0\" CELLBORDER=\"1\" CELLSPACING=\"0\"><TR><TD PORT=\"0x12345678-address\"><I>0x12345678</I></TD><TD PORT=\"0x12345678-type\"><B>u8</B></TD><TD PORT=\"0x12345678-value\">178</TD></TR><TR><TD PORT=\"0x12345678-address\"><I>0x12345678</I></TD><TD PORT=\"0x12345678-type\"><B>alloc::string::String</B></TD><TD PORT=\"0x12345678-value\">abcdefghi</TD></TR></TABLE></TD></TR>");
    }

    #[test]
    fn test_render_table_row_enum_struct() {
        let hex_address_string = String::from("0x12345678");
        let label_string = Some(String::from("my_u8"));
        let type_string = String::from("u8");
        let value = Some(Value::Owned(String::from("178")));
        let associated_data_descriptions = None;

        let inner1 = DataDescription {
            hex_address_string,
            label_string,
            type_string,
            value,
            associated_data_descriptions,
        };

        let hex_address_string = String::from("0x12345678");
        let label_string = Some(String::from("my_string"));
        let type_string = String::from("alloc::string::String");
        let value = Some(Value::Owned(String::from("abcdefghi")));
        let associated_data_descriptions = None;

        let inner2 = DataDescription {
            hex_address_string,
            label_string,
            type_string,
            value,
            associated_data_descriptions,
        };

        let hex_address_string = String::from("0x12345678");
        let label_string = Some(String::from("my_label"));
        let type_string = String::from("foo::bar::Enum");
        let value = Some(Value::Owned(String::from("MyEnumVariant")));
        let associated_data_descriptions = Some(vec![inner1, inner2]);

        let data_description = DataDescription {
            hex_address_string,
            label_string,
            type_string,
            value,
            associated_data_descriptions,
        };

        assert_eq!(data_description.render_table_row(), "<TR><TD PORT=\"0x12345678-label\">my_label</TD><TD PORT=\"0x12345678-address\"><I>0x12345678</I></TD><TD PORT=\"0x12345678-type\"><B>foo::bar::Enum</B></TD><TD PORT=\"0x12345678-value\">MyEnumVariant</TD><TD PORT=\"0x12345678-associated-data\"><TABLE BORDER=\"0\" CELLBORDER=\"1\" CELLSPACING=\"0\"><TR><TD PORT=\"0x12345678-label\">my_u8</TD><TD PORT=\"0x12345678-address\"><I>0x12345678</I></TD><TD PORT=\"0x12345678-type\"><B>u8</B></TD><TD PORT=\"0x12345678-value\">178</TD></TR><TR><TD PORT=\"0x12345678-label\">my_string</TD><TD PORT=\"0x12345678-address\"><I>0x12345678</I></TD><TD PORT=\"0x12345678-type\"><B>alloc::string::String</B></TD><TD PORT=\"0x12345678-value\">abcdefghi</TD></TR></TABLE></TD></TR>");
    }

    #[test]
    fn test_render_table_row_struct() {
        let hex_address_string = String::from("0x12345678");
        let label_string = Some(String::from("my_u8"));
        let type_string = String::from("u8");
        let value = Some(Value::Owned(String::from("178")));
        let associated_data_descriptions = None;

        let inner1 = DataDescription {
            hex_address_string,
            label_string,
            type_string,
            value,
            associated_data_descriptions,
        };

        let hex_address_string = String::from("0x12345678");
        let label_string = Some(String::from("my_string"));
        let type_string = String::from("alloc::string::String");
        let value = Some(Value::Owned(String::from("abcdefghi")));
        let associated_data_descriptions = None;

        let inner2 = DataDescription {
            hex_address_string,
            label_string,
            type_string,
            value,
            associated_data_descriptions,
        };

        let hex_address_string = String::from("0x12345678");
        let label_string = None;
        let type_string = String::from("foo::bar::Struct");
        let value = None;
        let associated_data_descriptions = Some(vec![inner1, inner2]);

        let data_description = DataDescription {
            hex_address_string,
            label_string,
            type_string,
            value,
            associated_data_descriptions,
        };

        assert_eq!(data_description.render_table_row(), "<TR><TD PORT=\"0x12345678-address\"><I>0x12345678</I></TD><TD PORT=\"0x12345678-type\"><B>foo::bar::Struct</B></TD><TD PORT=\"0x12345678-associated-data\"><TABLE BORDER=\"0\" CELLBORDER=\"1\" CELLSPACING=\"0\"><TR><TD PORT=\"0x12345678-label\">my_u8</TD><TD PORT=\"0x12345678-address\"><I>0x12345678</I></TD><TD PORT=\"0x12345678-type\"><B>u8</B></TD><TD PORT=\"0x12345678-value\">178</TD></TR><TR><TD PORT=\"0x12345678-label\">my_string</TD><TD PORT=\"0x12345678-address\"><I>0x12345678</I></TD><TD PORT=\"0x12345678-type\"><B>alloc::string::String</B></TD><TD PORT=\"0x12345678-value\">abcdefghi</TD></TR></TABLE></TD></TR>");
    }

    #[test]
    fn test_render_table_row_ref_to_struct() {
        let hex_address_string = String::from("0x12345678");
        let label_string = None;
        let type_string = String::from("&foo::bar::Struct");
        let value = Some(Value::Referenced(String::from("0xcafebaee")));
        let associated_data_descriptions = None;

        let data_description = DataDescription {
            hex_address_string,
            label_string,
            type_string,
            value,
            associated_data_descriptions,
        };

        assert_eq!(data_description.render_table_row(), "<TR><TD PORT=\"0x12345678-address\"><I>0x12345678</I></TD><TD PORT=\"0x12345678-type\"><B>&amp;foo::bar::Struct</B></TD><TD PORT=\"0x12345678-value\"></TD></TR>");
    }

    #[test]
    fn test_render_references() {
        let hex_address_string = String::from("0x12345678");
        let label_string = Some(String::from("my_u8_ref"));
        let type_string = String::from("u8");
        let value = Some(Value::Referenced(String::from("ref1")));
        let associated_data_descriptions = None;

        let inner1 = DataDescription {
            hex_address_string,
            label_string,
            type_string,
            value,
            associated_data_descriptions,
        };

        let hex_address_string = String::from("0x12345678");
        let label_string = Some(String::from("my_string_ref"));
        let type_string = String::from("alloc::string::String");
        let value = Some(Value::Referenced(String::from("ref2")));
        let associated_data_descriptions = None;

        let inner2 = DataDescription {
            hex_address_string,
            label_string,
            type_string,
            value,
            associated_data_descriptions,
        };

        let hex_address_string = String::from("0x12345678");
        let label_string = None;
        let type_string = String::from("foo::bar::Struct");
        let value = None;
        let associated_data_descriptions = Some(vec![inner1, inner2]);

        let data_description = DataDescription {
            hex_address_string,
            label_string,
            type_string,
            value,
            associated_data_descriptions,
        };

        assert_eq!(data_description.render_references("root-node-name"), "\"root-node-name\":\"0x12345678-value\" -> \"ref1\":\"ref1-address\"\n\"root-node-name\":\"0x12345678-value\" -> \"ref2\":\"ref2-address\"\n");
    }
}
