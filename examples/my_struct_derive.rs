//! Example demonstrating how to derive the trait `Visualize` for a struct, and use it to generate
//! a DOT file.

use std::error::Error;
use std::fs::File;

use vizz::{Graph, Visualize};

#[derive(Visualize)]
struct MyStruct<'a> {
    my_u8: u8,
    my_string: String,
    my_ref: &'a String,
}

pub fn main() -> Result<(), Box<dyn Error>> {
    // create some data
    let unowned_string = String::from("yabadabadoo!");
    let my_struct = MyStruct {
        my_u8: 42,
        my_string: "HELLO WORLD".into(),
        my_ref: &unowned_string,
    };

    // create file
    let mut dot_file = File::create("my_struct.dot")?;

    // create graph
    Graph::new()
        .add_node(&my_struct)
        .add_node(&unowned_string)
        .write_to(&mut dot_file)?;

    Ok(())
}
