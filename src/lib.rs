use std::string::ToString;

fn html_encode(s: &str) -> String {
    s.replace("&", "&amp;")
        .replace("<", "&lt;")
        .replace(">", "&gt;")
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

    fn with_label<T>(self, label_string: T) -> Self
    where
        T: Into<String>,
    {
        Self {
            label_string: Some(label_string.into()),
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

impl Dot for DataDescription {
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

impl Dot for u8 {
    fn data(&self) -> Option<String> {
        Some(self.to_string())
    }
}

impl Dot for usize {
    fn data(&self) -> Option<String> {
        Some(self.to_string())
    }
}

impl Dot for String {
    fn data(&self) -> Option<String> {
        Some(self.clone())
    }
}

impl Dot for &String {}

impl<T> Dot for Option<T>
where
    T: Dot,
{
    fn data(&self) -> Option<String> {
        Some(
            match self {
                Some(_) => "Some",
                None => "None",
            }
            .into(),
        )
    }

    fn associated_data(&self) -> Option<Vec<DataDescription>> {
        self.as_ref().map(|x| vec![DataDescription::from(x)])
    }
}

impl<T> Dot for &Option<T> {}

impl<T> Dot for Vec<T>
where
    T: Dot,
{
    fn associated_data(&self) -> Option<Vec<DataDescription>> {
        Some(self.iter().map(DataDescription::from).collect())
    }
}

impl<T> Dot for &Vec<T> {}

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

struct MyStruct<'a> {
    my_u8: u8,
    my_string: String,
    my_ref: &'a String,
}

impl<'a> Dot for MyStruct<'a> {
    fn associated_data(&self) -> Option<Vec<DataDescription>> {
        Some(vec![
            DataDescription::from(&self.my_u8).with_label("my_u8"),
            DataDescription::from(&self.my_string).with_label("my_string"),
            DataDescription::from(&self.my_ref).with_label("my_ref"),
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
        let my_other_string = String::from("yabadabadoo!");
        let my_struct = MyStruct {
            my_u8: 42,
            my_string: "HELLO WORLD".into(),
            my_ref: &my_other_string,
        };
        let my_struct_dot = (&my_struct).render_node();
        println!("{}", my_struct_dot);
        panic!();
    }

    #[test]
    fn test_data_description() {
        let my_other_string = String::from("yabadabadoo!");
        let my_struct = MyStruct {
            my_u8: 42,
            my_string: "HELLO WORLD".into(),
            my_ref: &my_other_string,
        };
        let my_struct_description =
            (&DataDescription::from(&my_struct).with_label("my_struct_description")).render_node();
        println!("{}", my_struct_description);
        panic!();
    }
}
