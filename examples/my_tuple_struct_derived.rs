use std::error::Error;
use std::fs::File;

use vizz::Graph;
use vizz::Visualize;

#[derive(Visualize)]
struct MyStruct(u8, usize, String);

pub fn main() -> Result<(), Box<dyn Error>> {
    // create some data
    let my_struct = MyStruct(45, 42_000_000_000, String::from("this is my tuple struct"));

    // create file
    let mut dot_file = File::create("my_struct.dot")?;

    // create graph
    Graph::from(&my_struct).write_to(&mut dot_file)?;

    Ok(())
}
