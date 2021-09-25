use crate::util::*;
use crate::Visualize;

#[derive(strum_macros::ToString)]
pub enum Value {
    Owned(String),
    Referenced(String),
}

#[readonly::make]
pub struct DataDescription {
    pub label_string: Option<String>,
    pub hex_address_string: String,
    pub type_string: String,
    pub value: Option<Value>,
    pub associated_data_descriptions: Option<Vec<DataDescription>>,
}

impl DataDescription {
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
        render_label_port(&self.hex_address_string)
    }

    fn render_type_port(&self) -> String {
        render_type_port(&self.hex_address_string)
    }

    fn render_address_port(&self) -> String {
        render_address_port(&self.hex_address_string)
    }

    fn render_value_port(&self) -> String {
        render_value_port(&self.hex_address_string)
    }

    fn render_associated_data_port(&self) -> String {
        render_associated_data_port(&self.hex_address_string)
    }

    fn render_reference(&self, node_name: &str) -> Option<String> {
        if let Some(Value::Referenced(target)) = &self.value {
            Some(format!(
                r#""{}":"{}" -> "{}":"{}""#,
                node_name,
                self.render_value_port(),
                target,
                render_address_port(&target)
            ))
        } else {
            None
        }
    }

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
            html_encode(&self.type_string)
        )
    }

    fn render_value_table_data(&self) -> String {
        format!(
            r#"<TD PORT="{}">{}</TD>"#,
            self.render_value_port(),
            match &self.value {
                Some(Value::Owned(value)) => html_encode(value),
                _ => String::new(),
            }
        )
    }

    fn render_associated_data_table(&self) -> String {
        match &self.associated_data_descriptions {
            Some(associated_data_descriptions) => format!(
                r#"<TD PORT="{}">{}</TD>"#,
                self.render_associated_data_port(),
                render_table(
                    associated_data_descriptions
                        .iter()
                        .map(DataDescription::render_table_row),
                )
            ),
            None => String::new(),
        }
    }

    pub fn render_table_row(&self) -> String {
        format!(
            r#"<TR>{}"{}"{}{}{}</TR>"#,
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
            hex_address_string: format!("{:?}", t as *const _),
            type_string: type_of(t),
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
    use super::*;

    #[test]
    fn test_label() {
        let label = "my_test_u8";
        let data_description = DataDescription::from(&8u8).with_label(label);
        assert_eq!(data_description.label_string.unwrap(), label);
    }
}
