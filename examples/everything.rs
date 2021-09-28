//! Example demonstrating how to derive the trait `Visualize` for a struct, and use it to generate
//! a DOT file.

use std::error::Error;
use std::fs::File;

use vizz::{Graph, Visualize};

#[derive(Visualize)]
struct NotCopy(String);

#[derive(Visualize)]
struct MyStruct<'a>(
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
    &'a u8,
    &'a u16,
    &'a u32,
    &'a u64,
    &'a u128,
    &'a usize,
    &'a i8,
    &'a i16,
    &'a i32,
    &'a i64,
    &'a i128,
    &'a isize,
    [u8; 0],
    [i64; 1],
    [usize; 32],
    bool,
    &'a bool,
    char,
    &'a char,
    &'a [NotCopy],
    &'a [usize],
    &'a [NotCopy],
    &'a str,
    (u8, usize, i16, i32, NotCopy, &'a str, bool, char),
    (),
);

pub fn main() -> Result<(), Box<dyn Error>> {
    // create some data
    let not_copy_array = [
        NotCopy(String::from("a")),
        NotCopy(String::from("b")),
        NotCopy(String::from("c")),
    ];
    let copy_array = [
        1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25,
        26, 27, 28, 29, 30, 31, 32,
    ];
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
        &u8::MAX,
        &u16::MAX,
        &u32::MAX,
        &u64::MAX,
        &u128::MAX,
        &usize::MAX,
        &i8::MAX,
        &i16::MAX,
        &i32::MAX,
        &i64::MAX,
        &i128::MAX,
        &isize::MAX,
        [],
        [i64::MIN],
        copy_array,
        true,
        &false,
        'c',
        &'d',
        &not_copy_array[0..2],
        &copy_array[4..16],
        &not_copy_array[0..0],
        "this is my &str",
        (1, 1, 1, 1, NotCopy(String::from("yeet")), "yort", true, 'A'),
        (),
    );

    // create file
    let mut dot_file = File::create("everything.dot")?;

    // create graph
    Graph::from(&my_struct).write_to(&mut dot_file)?;

    Ok(())
}
