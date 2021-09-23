use std::string::ToString;

fn render_table(table_rows: impl Iterator<Item = String>) -> String {
    format!(
        r#"<TABLE BORDER="0" CELLBORDER="1" CELLSPACING="0">{}</TABLE>"#,
        table_rows.fold(String::new(), |acc, s| acc + &s)
    )
}

pub trait Dot: Sized {
    fn data_display(&self) -> String;

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
    hex_address_string: String,
    type_string: String,
    data_string: String,
    associated_data_descriptions: Option<Vec<DataDescription>>,
}

impl DataDescription {
    fn type_string_html_encoded(&self) -> String {
        self.type_string.replace("&", "&amp;")
    }

    fn data_string_html_encoded(&self) -> String {
        self.data_string.replace("&", "&amp;")
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
            self.type_string_html_encoded()
        )
    }

    fn render_value_table_data(&self) -> String {
        format!(
            r#"<TD PORT="{}-value">{}</TD>"#,
            self.hex_address_string,
            self.data_string_html_encoded()
        )
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
            None => String::from(""),
        }
    }

    fn render_table_row(&self) -> String {
        format!(
            r#"<TR>"{}"{}{}{}</TR>"#,
            self.render_hex_address_table_data(),
            self.render_type_table_data(),
            self.render_value_table_data(),
            self.render_associated_data_table(),
        )
    }
}

impl<T> From<&T> for DataDescription
where
    T: Dot,
{
    fn from(t: &T) -> Self {
        Self {
            hex_address_string: format!("{:?}", t as *const _),
            type_string: type_of(t),
            data_string: t.data_display(),
            associated_data_descriptions: t.associated_data(),
        }
    }
}

impl Dot for u8 {
    fn data_display(&self) -> String {
        self.to_string()
    }
}

impl Dot for String {
    fn data_display(&self) -> String {
        self.clone()
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
    fn data_display(&self) -> String {
        self.to_string()
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
}
