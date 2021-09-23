use visualize::DataDescription;
use visualize::Visualize;

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

pub fn main() {
    let unowned_string = String::from("yabadabadoo!");
    let my_struct = MyStruct {
        my_u8: 42,
        my_string: "HELLO WORLD".into(),
        my_ref: &unowned_string,
    };
    let my_struct_dot = (&my_struct).render_node();
    println!("{}", my_struct_dot);
}
