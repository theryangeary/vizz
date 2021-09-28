use crate::address::Address;
use crate::node::RenderedNode;
use crate::util;
use crate::Visualize;

#[derive(Debug, Clone)]
/// The value of a [Visualize] implementer
pub enum Value {
    /// The implementer owns this data and the data will appear as this string
    Owned(String),
    /// The implementer references this data and there will be a graph edge from this reference to the
    /// referenced data
    ///
    /// The memory address of the referenced data.
    Referenced(Address, RenderedNode),
}

impl Value {
    /// Create a [Value::Referenced] from parts that implement [Into] for their respective field
    /// types
    pub fn referenced<IntoAddress, IntoRenderedNode>(
        address: IntoAddress,
        rendered_node: IntoRenderedNode,
    ) -> Self
    where
        IntoAddress: Into<Address>,
        IntoRenderedNode: Into<RenderedNode>,
    {
        Value::Referenced(address.into(), rendered_node.into())
    }
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
    /// The memory location of this data
    pub address: Address,
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

    /// Create the DOT code to make an arrow from this node to another node
    ///
    /// The other node will need to be added to the graph separately from this call.
    ///
    /// The node_root_address is the name of the top level data this [DataDescription] lives inside of.
    fn render_reference(&self, node_root_address: &Address) -> Option<String> {
        if let Some(Value::Referenced(target, _)) = &self.value {
            Some(format!(
                "\"{}\":\"{}\" -> \"{}\":\"{}\"\n",
                node_root_address,
                self.address.render_value_port(),
                target,
                target.render_address_port()
            ))
        } else {
            None
        }
    }

    /// Create the DOT code to make all arrows from data owned by this node to the data they
    /// reference
    ///
    /// The referenced nodes must be added to the graph separately.
    pub fn render_references(&self, node_root_address: &Address) -> String {
        let this_reference = self
            .render_reference(node_root_address)
            .unwrap_or_else(String::new);

        match &self.associated_data_descriptions {
            Some(associated_data_descriptions) => associated_data_descriptions
                .iter()
                .fold(this_reference, |acc, associated_data| {
                    acc + &associated_data.render_references(node_root_address)
                }),
            None => this_reference,
        }
    }

    fn render_label_table_data(&self) -> String {
        match &self.label_string {
            Some(label_string) => {
                format!(
                    r#"<TD PORT="{}">{}</TD>"#,
                    self.address.render_label_port(),
                    label_string
                )
            }
            None => String::new(),
        }
    }

    fn render_hex_address_table_data(&self) -> String {
        format!(
            r#"<TD PORT="{}"><I>{}</I></TD>"#,
            self.address.render_address_port(),
            self.address,
        )
    }

    fn render_type_table_data(&self) -> String {
        format!(
            r#"<TD PORT="{}"><B>{}</B></TD>"#,
            self.address.render_type_port(),
            util::html_encode(&self.type_string)
        )
    }

    fn render_value_table_data(&self) -> String {
        match &self.value {
            Some(value) => format!(
                r#"<TD PORT="{}">{}</TD>"#,
                self.address.render_value_port(),
                match value {
                    Value::Owned(data) => util::html_encode(data),
                    Value::Referenced(_, _) => String::new(),
                }
            ),
            None => String::new(),
        }
    }

    fn render_associated_data_table(&self) -> String {
        match &self.associated_data_descriptions {
            Some(associated_data_descriptions) => format!(
                r#"<TD PORT="{}">{}</TD>"#,
                self.address.render_associated_data_port(),
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
            address: Address::new(t),
            type_string: util::type_of(t),
            value: t.data(),
            associated_data_descriptions: t.associated_data(),
        }
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
        let address = Address::from("0x12345678");
        let label_string = Some(String::from("my_label"));
        let type_string = String::from("u8");
        let value = Some(Value::Owned(String::from("145")));
        let associated_data_descriptions = None;

        let data_description = DataDescription {
            address,
            label_string,
            type_string,
            value,
            associated_data_descriptions,
        };

        assert_eq!(data_description.render_table_row(), "<TR><TD PORT=\"0x12345678-label\">my_label</TD><TD PORT=\"0x12345678-address\"><I>0x12345678</I></TD><TD PORT=\"0x12345678-type\"><B>u8</B></TD><TD PORT=\"0x12345678-value\">145</TD></TR>");
    }

    #[test]
    fn test_render_table_row_enum_plain() {
        let address = Address::from("0x12345678");
        let label_string = Some(String::from("my_label"));
        let type_string = String::from("foo::bar::Enum");
        let value = Some(Value::Owned(String::from("MyEnumVariant")));
        let associated_data_descriptions = None;

        let data_description = DataDescription {
            address,
            label_string,
            type_string,
            value,
            associated_data_descriptions,
        };

        assert_eq!(data_description.render_table_row(), "<TR><TD PORT=\"0x12345678-label\">my_label</TD><TD PORT=\"0x12345678-address\"><I>0x12345678</I></TD><TD PORT=\"0x12345678-type\"><B>foo::bar::Enum</B></TD><TD PORT=\"0x12345678-value\">MyEnumVariant</TD></TR>");
    }

    #[test]
    fn test_render_table_row_enum_tuple() {
        let address = Address::from("0x12345678");
        let label_string = None;
        let type_string = String::from("u8");
        let value = Some(Value::Owned(String::from("178")));
        let associated_data_descriptions = None;

        let inner1 = DataDescription {
            address,
            label_string,
            type_string,
            value,
            associated_data_descriptions,
        };

        let address = Address::from("0x12345678");
        let label_string = None;
        let type_string = String::from("alloc::string::String");
        let value = Some(Value::Owned(String::from("abcdefghi")));
        let associated_data_descriptions = None;

        let inner2 = DataDescription {
            address,
            label_string,
            type_string,
            value,
            associated_data_descriptions,
        };

        let address = Address::from("0x12345678");
        let label_string = Some(String::from("my_label"));
        let type_string = String::from("foo::bar::Enum");
        let value = Some(Value::Owned(String::from("MyEnumVariant")));
        let associated_data_descriptions = Some(vec![inner1, inner2]);

        let data_description = DataDescription {
            address,
            label_string,
            type_string,
            value,
            associated_data_descriptions,
        };

        assert_eq!(data_description.render_table_row(), "<TR><TD PORT=\"0x12345678-label\">my_label</TD><TD PORT=\"0x12345678-address\"><I>0x12345678</I></TD><TD PORT=\"0x12345678-type\"><B>foo::bar::Enum</B></TD><TD PORT=\"0x12345678-value\">MyEnumVariant</TD><TD PORT=\"0x12345678-associated-data\"><TABLE BORDER=\"0\" CELLBORDER=\"1\" CELLSPACING=\"0\"><TR><TD PORT=\"0x12345678-address\"><I>0x12345678</I></TD><TD PORT=\"0x12345678-type\"><B>u8</B></TD><TD PORT=\"0x12345678-value\">178</TD></TR><TR><TD PORT=\"0x12345678-address\"><I>0x12345678</I></TD><TD PORT=\"0x12345678-type\"><B>alloc::string::String</B></TD><TD PORT=\"0x12345678-value\">abcdefghi</TD></TR></TABLE></TD></TR>");
    }

    #[test]
    fn test_render_table_row_enum_struct() {
        let address = Address::from("0x12345678");
        let label_string = Some(String::from("my_u8"));
        let type_string = String::from("u8");
        let value = Some(Value::Owned(String::from("178")));
        let associated_data_descriptions = None;

        let inner1 = DataDescription {
            address,
            label_string,
            type_string,
            value,
            associated_data_descriptions,
        };

        let address = Address::from("0x12345678");
        let label_string = Some(String::from("my_string"));
        let type_string = String::from("alloc::string::String");
        let value = Some(Value::Owned(String::from("abcdefghi")));
        let associated_data_descriptions = None;

        let inner2 = DataDescription {
            address,
            label_string,
            type_string,
            value,
            associated_data_descriptions,
        };

        let address = Address::from("0x12345678");
        let label_string = Some(String::from("my_label"));
        let type_string = String::from("foo::bar::Enum");
        let value = Some(Value::Owned(String::from("MyEnumVariant")));
        let associated_data_descriptions = Some(vec![inner1, inner2]);

        let data_description = DataDescription {
            address,
            label_string,
            type_string,
            value,
            associated_data_descriptions,
        };

        assert_eq!(data_description.render_table_row(), "<TR><TD PORT=\"0x12345678-label\">my_label</TD><TD PORT=\"0x12345678-address\"><I>0x12345678</I></TD><TD PORT=\"0x12345678-type\"><B>foo::bar::Enum</B></TD><TD PORT=\"0x12345678-value\">MyEnumVariant</TD><TD PORT=\"0x12345678-associated-data\"><TABLE BORDER=\"0\" CELLBORDER=\"1\" CELLSPACING=\"0\"><TR><TD PORT=\"0x12345678-label\">my_u8</TD><TD PORT=\"0x12345678-address\"><I>0x12345678</I></TD><TD PORT=\"0x12345678-type\"><B>u8</B></TD><TD PORT=\"0x12345678-value\">178</TD></TR><TR><TD PORT=\"0x12345678-label\">my_string</TD><TD PORT=\"0x12345678-address\"><I>0x12345678</I></TD><TD PORT=\"0x12345678-type\"><B>alloc::string::String</B></TD><TD PORT=\"0x12345678-value\">abcdefghi</TD></TR></TABLE></TD></TR>");
    }

    #[test]
    fn test_render_table_row_struct() {
        let address = Address::from("0x12345678");
        let label_string = Some(String::from("my_u8"));
        let type_string = String::from("u8");
        let value = Some(Value::Owned(String::from("178")));
        let associated_data_descriptions = None;

        let inner1 = DataDescription {
            address,
            label_string,
            type_string,
            value,
            associated_data_descriptions,
        };

        let address = Address::from("0x12345678");
        let label_string = Some(String::from("my_string"));
        let type_string = String::from("alloc::string::String");
        let value = Some(Value::Owned(String::from("abcdefghi")));
        let associated_data_descriptions = None;

        let inner2 = DataDescription {
            address,
            label_string,
            type_string,
            value,
            associated_data_descriptions,
        };

        let address = Address::from("0x12345678");
        let label_string = None;
        let type_string = String::from("foo::bar::Struct");
        let value = None;
        let associated_data_descriptions = Some(vec![inner1, inner2]);

        let data_description = DataDescription {
            address,
            label_string,
            type_string,
            value,
            associated_data_descriptions,
        };

        assert_eq!(data_description.render_table_row(), "<TR><TD PORT=\"0x12345678-address\"><I>0x12345678</I></TD><TD PORT=\"0x12345678-type\"><B>foo::bar::Struct</B></TD><TD PORT=\"0x12345678-associated-data\"><TABLE BORDER=\"0\" CELLBORDER=\"1\" CELLSPACING=\"0\"><TR><TD PORT=\"0x12345678-label\">my_u8</TD><TD PORT=\"0x12345678-address\"><I>0x12345678</I></TD><TD PORT=\"0x12345678-type\"><B>u8</B></TD><TD PORT=\"0x12345678-value\">178</TD></TR><TR><TD PORT=\"0x12345678-label\">my_string</TD><TD PORT=\"0x12345678-address\"><I>0x12345678</I></TD><TD PORT=\"0x12345678-type\"><B>alloc::string::String</B></TD><TD PORT=\"0x12345678-value\">abcdefghi</TD></TR></TABLE></TD></TR>");
    }

    #[test]
    fn test_render_table_row_ref_to_struct() {
        let address = Address::from("0x12345678");
        let label_string = None;
        let type_string = String::from("&foo::bar::Struct");
        let value = Some(Value::Referenced(
            Address::from("0xcafebaee"),
            RenderedNode::from(String::from("this is unchecked")),
        ));
        let associated_data_descriptions = None;

        let data_description = DataDescription {
            address,
            label_string,
            type_string,
            value,
            associated_data_descriptions,
        };

        assert_eq!(data_description.render_table_row(), "<TR><TD PORT=\"0x12345678-address\"><I>0x12345678</I></TD><TD PORT=\"0x12345678-type\"><B>&amp;foo::bar::Struct</B></TD><TD PORT=\"0x12345678-value\"></TD></TR>");
    }

    #[test]
    fn test_render_references() {
        let address = Address::from("0x12345678");
        let label_string = Some(String::from("my_u8_ref"));
        let type_string = String::from("u8");
        let value = Some(Value::Referenced(
            Address::from("ref1"),
            RenderedNode::from(String::from("unchecked")),
        ));
        let associated_data_descriptions = None;

        let inner1 = DataDescription {
            address,
            label_string,
            type_string,
            value,
            associated_data_descriptions,
        };

        let address = Address::from("0x12345678");
        let label_string = Some(String::from("my_string_ref"));
        let type_string = String::from("alloc::string::String");
        let value = Some(Value::Referenced(
            Address::from("ref2"),
            RenderedNode::from(String::from("unchecked")),
        ));
        let associated_data_descriptions = None;

        let inner2 = DataDescription {
            address,
            label_string,
            type_string,
            value,
            associated_data_descriptions,
        };

        let address = Address::from("0x12345678");
        let label_string = None;
        let type_string = String::from("foo::bar::Struct");
        let value = None;
        let associated_data_descriptions = Some(vec![inner1, inner2]);

        let data_description = DataDescription {
            address,
            label_string,
            type_string,
            value,
            associated_data_descriptions,
        };

        assert_eq!(data_description.render_references(&Address::from("root-node-name")), "\"root-node-name\":\"0x12345678-value\" -> \"ref1\":\"ref1-address\"\n\"root-node-name\":\"0x12345678-value\" -> \"ref2\":\"ref2-address\"\n");
    }
}
