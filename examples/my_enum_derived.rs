//! Example of deriving Visualize for an enum
//!
//! The generated graph has no references between nodes, and thus is a bit contrived. But it
//! demonstrates adding unrelated nodes to the same graph.

use std::fs::File;

use vizz::Graph;
use vizz::Visualize;

#[derive(Visualize)]
enum MyEnum {
    Plain,
    WithU8(u8),
    WithU8AndString(u8, String),
    WithStruct { my_u8: u8, my_string: String },
}

pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    // create some values
    let plain_enum = MyEnum::Plain;
    let enum_with_u8_and_string = MyEnum::WithU8AndString(6, String::from("hey"));
    let enum_with_u8 = MyEnum::WithU8(10);
    let enum_with_named_fields = MyEnum::WithStruct {
        my_u8: 8,
        my_string: String::from("hey hey mic check 1 2 3"),
    };

    // create file
    let mut dot_file = File::create("my_enum.dot")?;

    // create graph
    Graph::new()
        .set_id("my_enum_visualization")
        .add_node(&plain_enum)
        .add_node(&enum_with_named_fields)
        .add_node(&enum_with_u8)
        .add_node(&enum_with_u8_and_string)
        .write_to(&mut dot_file)?;

    Ok(())
}
