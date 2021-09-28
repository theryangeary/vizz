//! Example demonstrating how to derive the trait `Visualize` for a struct, and use it to generate
//! a DOT file.

use std::error::Error;
use std::fs::File;

use vizz::{Graph, Visualize};

#[derive(Visualize)]
struct MyStruct(
    u8,
    u16,
    u32,
    u64,
    u128,
    usize,
    i8,
    i16,
    i32,
    i64,
    i128,
    isize,
    [u8; 0],
    [i64; 1],
    [usize; 32],
    bool,
    char,
);

pub fn main() -> Result<(), Box<dyn Error>> {
    // create some data
    let my_struct = MyStruct(
        u8::MAX,
        u16::MAX,
        u32::MAX,
        u64::MAX,
        u128::MAX,
        usize::MAX,
        i8::MAX,
        i16::MAX,
        i32::MAX,
        i64::MAX,
        i128::MAX,
        isize::MAX,
        [],
        [i64::MIN],
        [
            1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24,
            25, 26, 27, 28, 29, 30, 31, 32,
        ],
        true,
        'c',
    );

    // create file
    let mut dot_file = File::create("everything.dot")?;

    // create graph
    Graph::from(&my_struct).write_to(&mut dot_file)?;

    Ok(())
}
