//! The `Visualize` trait allows any Rust data structure to be graphically represented using
//! GraphViz and Dot.

mod constants;
mod data_description;
mod graph;
mod impls;
mod util;
mod visualize;

pub use data_description::DataDescription;
pub use graph::Graph;
pub use visualize::Visualize;

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
