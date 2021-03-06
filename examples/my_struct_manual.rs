//! This example implements a struct `MyStruct` and then implements the trait `Visualize` for it.
//! End users will likely have no need to manually implement this trait, and should prefer to use
//! the derive macro as shown in the `my_struct_derive` example.

use std::error::Error;
use std::fs::File;

use vizz::DataDescription;
use vizz::Graph;
use vizz::Visualize;

struct MyStruct<'a> {
    my_u8: u8,
    my_string: String,
    my_ref: &'a String,
}

impl<'a> Visualize for MyStruct<'a> {
    fn associated_data(&self) -> Option<Vec<DataDescription>> {
        Some(vec![
            DataDescription::from(&self.my_u8).with_label("my_u8"),
            DataDescription::from(&self.my_string).with_label("my_string"),
            DataDescription::from(&self.my_ref).with_label("my_ref"),
        ])
    }
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
    Graph::from(&my_struct).write_to(&mut dot_file)?;

    Ok(())
}
