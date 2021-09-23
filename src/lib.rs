use std::string::ToString;

fn html_encode(s: &str) -> String {
    s.replace("&", "&amp;")
}

fn render_table(table_rows: impl Iterator<Item = String>) -> String {
    format!(
        r#"<TABLE BORDER="0" CELLBORDER="1" CELLSPACING="0">{}</TABLE>"#,
        table_rows.fold(String::new(), |acc, s| acc + &s)
    )
}

pub trait Dot: Sized {
    fn data(&self) -> Option<String> {
        None
    }

    fn associated_data(&self) -> Option<Vec<DataDescription>> {
        None
    }

    fn render_node(&self) -> String {
        let data_description = DataDescription::from(self);
        format!(
            r#""{}" [label=<{}>];"#,
            data_description.hex_address_string,
            render_table(std::iter::once(data_description.render_table_row()))
        )
    }
}

fn type_of<T>(_: &T) -> String {
    std::any::type_name::<T>().into()
}

pub struct DataDescription {
    label_string: Option<String>,
    hex_address_string: String,
    type_string: String,
    data_string: Option<String>,
    associated_data_descriptions: Option<Vec<DataDescription>>,
}

impl DataDescription {
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

    fn render_table_row(&self) -> String {
        format!(
            r#"<TR>{}"{}"{}{}{}</TR>"#,
            self.render_label_table_data(),
            self.render_hex_address_table_data(),
            self.render_type_table_data(),
            self.render_value_table_data(),
            self.render_associated_data_table(),
        )
    }

    fn with_label(self, label_string: String) -> Self {
        Self {
            label_string: Some(label_string),
            ..self
        }
    }
}

impl<T> From<&T> for DataDescription
where
    T: Dot,
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

impl Dot for u8 {
    fn data(&self) -> Option<String> {
        Some(self.to_string())
    }
}

impl Dot for String {
    fn data(&self) -> Option<String> {
        Some(self.clone())
    }
}

#[derive(strum_macros::ToString)]
enum MyEnum {
    Plain,
    WithU8(u8),
    WithU8AndString(u8, String),
    WithStruct { my_u8: u8, my_string: String },
}

impl Dot for MyEnum {
    fn data(&self) -> Option<String> {
        Some(self.to_string())
    }

    fn associated_data(&self) -> Option<Vec<DataDescription>> {
        match self {
            MyEnum::Plain => None,
            MyEnum::WithU8(a) => Some(vec![DataDescription::from(a)]),
            MyEnum::WithU8AndString(a, b) => {
                Some(vec![DataDescription::from(a), DataDescription::from(b)])
            }
            MyEnum::WithStruct { my_u8, my_string } => Some(vec![
                DataDescription::from(my_u8),
                DataDescription::from(my_string),
            ]),
        }
    }
}

struct MyStruct {
    my_u8: u8,
    my_string: String,
}

impl Dot for MyStruct {
    fn associated_data(&self) -> Option<Vec<DataDescription>> {
        Some(vec![
            DataDescription::from(&self.my_u8).with_label("my_u8".into()),
            DataDescription::from(&self.my_string).with_label("my_string".into()),
        ])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_u8() {
        let my_int: u8 = 128;
        let my_int_dot = (&my_int).render_node();
        assert!(my_int_dot.contains("128"));
        assert!(my_int_dot.contains("u8"));
    }

    #[test]
    fn test_enum() {
        let my_enum = MyEnum::WithU8AndString(8, String::from("hey"));
        let my_enum_dot = (&my_enum).render_node();
        println!("{}", my_enum_dot);
        let my_enum = MyEnum::WithU8(8);
        let my_enum_dot = (&my_enum).render_node();
        println!("{}", my_enum_dot);
        let my_enum = MyEnum::WithStruct {
            my_u8: 8,
            my_string: String::from("hey"),
        };
        let my_enum_dot = (&my_enum).render_node();
        println!("{}", my_enum_dot);
        let my_enum = MyEnum::Plain;
        let my_enum_dot = (&my_enum).render_node();
        println!("{}", my_enum_dot);
        panic!();
    }

    #[test]
    fn test_struct() {
        let my_struct = MyStruct {
            my_u8: 42,
            my_string: "HELLO WORLD".into(),
        };
        let my_struct_dot = (&my_struct).render_node();
        println!("{}", my_struct_dot);
        panic!();
    }
}
