use crate::util::*;
use crate::Visualize;

pub struct DataDescription {
    label_string: Option<String>,
    hex_address_string: String,
    type_string: String,
    data_string: Option<String>,
    associated_data_descriptions: Option<Vec<DataDescription>>,
}

impl DataDescription {
    pub fn label_string(&self) -> &Option<String> {
        &self.label_string
    }

    pub fn hex_address_string(&self) -> &String {
        &self.hex_address_string
    }

    pub fn type_string(&self) -> &String {
        &self.type_string
    }

    pub fn data_string(&self) -> &Option<String> {
        &self.data_string
    }

    pub fn associated_data_descriptions(&self) -> &Option<Vec<DataDescription>> {
        &self.associated_data_descriptions
    }

    pub fn with_label<T>(self, label_string: T) -> Self
    where
        T: Into<String>,
    {
        Self {
            label_string: Some(label_string.into()),
            ..self
        }
    }

    fn render_label_table_data(&self) -> String {
        match &self.label_string {
            Some(label_string) => format!(
                r#"<TD PORT="{}-label">{}</TD>"#,
                self.hex_address_string, label_string
            ),
            None => String::new(),
        }
    }

    fn render_hex_address_table_data(&self) -> String {
        format!(
            r#"<TD PORT="{0}-address"><I>{0}</I></TD>"#,
            self.hex_address_string,
        )
    }
    fn render_type_table_data(&self) -> String {
        format!(
            r#"<TD PORT="{}-type"><B>{}</B></TD>"#,
            self.hex_address_string,
            html_encode(&self.type_string)
        )
    }

    fn render_value_table_data(&self) -> String {
        match &self.data_string {
            Some(data_string) => format!(
                r#"<TD PORT="{}-value">{}</TD>"#,
                self.hex_address_string,
                html_encode(data_string)
            ),
            None => String::new(),
        }
    }

    fn render_associated_data_table(&self) -> String {
        match &self.associated_data_descriptions {
            Some(associated_data_descriptions) => format!(
                r#"<TD PORT="{}-associate-data">{}</TD>"#,
                self.hex_address_string,
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
            data_string: t.data(),
            associated_data_descriptions: t.associated_data(),
        }
    }
}

impl Visualize for DataDescription {
    fn associated_data(&self) -> Option<Vec<DataDescription>> {
        Some(vec![
            DataDescription::from(&self.label_string).with_label("label_string"),
            DataDescription::from(&self.hex_address_string).with_label("hex_address_string"),
            DataDescription::from(&self.type_string).with_label("type_string"),
            DataDescription::from(&self.data_string).with_label("data_string"),
            DataDescription::from(&self.associated_data_descriptions)
                .with_label("associated_data_descriptions"),
        ])
    }
}
