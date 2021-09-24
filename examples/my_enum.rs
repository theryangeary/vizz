//! Example of manually implementing Visualize for an enum

use std::fs::File;

use visualize::DataDescription;
use visualize::Graph;
use visualize::Visualize;

#[derive(strum_macros::ToString)]
enum MyEnum {
    Plain,
    WithU8(u8),
    WithU8AndString(u8, String),
    WithStruct { my_u8: u8, my_string: String },
}

impl Visualize for MyEnum {
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
                DataDescription::from(my_u8).with_label("my_u8"),
                DataDescription::from(my_string).with_label("my_string"),
            ]),
        }
    }
}

pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    // create some values
    let plain_enum = MyEnum::Plain;
    let enum_with_u8_and_string = MyEnum::WithU8AndString(8, String::from("hey"));
    let enum_with_u8 = MyEnum::WithU8(8);
    let enum_with_named_fields = MyEnum::WithStruct {
        my_u8: 8,
        my_string: String::from("hey"),
    };

    // create file
    let mut dot_file = File::create("my_enum.dot")?;

    // create graph
    Graph::new()
        .add(&plain_enum)
        .add(&enum_with_named_fields)
        .add(&enum_with_u8)
        .add(&enum_with_u8_and_string)
        .write_to(&mut dot_file)?;

    Ok(())
}
