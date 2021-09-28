use vizz::{Graph, Visualize};

#[derive(Visualize)]
struct MyStruct<'a> {
    pub my_u8: u8,
    pub my_string: String,
    pub my_ref: &'a String,
}

#[test]
fn test_struct() {
    let unowned_string = String::from("yabadabadoo!");
    let my_struct = MyStruct {
        my_u8: 42,
        my_string: "HELLO WORLD".into(),
        my_ref: &unowned_string,
    };

    let ref_target_address = vizz::Address::new(&unowned_string);
    let struct_address = vizz::Address::new(&my_struct);
    let u8_address = vizz::Address::new(&my_struct.my_u8);
    let string_address = vizz::Address::new(&my_struct.my_string);
    let ref_address = vizz::Address::new(&my_struct.my_ref);

    assert_eq!(Graph::new().add_node(&my_struct).render(), format!("digraph visualization {{\n  node [shape=plaintext]\n    \"{0}\" [label=<<TABLE BORDER=\"0\" CELLBORDER=\"1\" CELLSPACING=\"0\"><TR><TD PORT=\"{0}-address\"><I>{0}</I></TD><TD PORT=\"{0}-type\"><B>struct::MyStruct</B></TD><TD PORT=\"{0}-associated-data\"><TABLE BORDER=\"0\" CELLBORDER=\"1\" CELLSPACING=\"0\"><TR><TD PORT=\"{2}-label\">my_u8</TD><TD PORT=\"{2}-address\"><I>{2}</I></TD><TD PORT=\"{2}-type\"><B>u8</B></TD><TD PORT=\"{2}-value\">42</TD></TR><TR><TD PORT=\"{3}-label\">my_string</TD><TD PORT=\"{3}-address\"><I>{3}</I></TD><TD PORT=\"{3}-type\"><B>alloc::string::String</B></TD><TD PORT=\"{3}-value\">HELLO WORLD</TD></TR><TR><TD PORT=\"{4}-label\">my_ref</TD><TD PORT=\"{4}-address\"><I>{4}</I></TD><TD PORT=\"{4}-type\"><B>&amp;alloc::string::String</B></TD><TD PORT=\"{4}-value\"></TD></TR></TABLE></TD></TR></TABLE>>];\n    \"{0}\":\"{4}-value\" -> \"{1}\":\"{1}-address\"\n  node [shape=plaintext]\n    \"{5}\" [label=<<TABLE BORDER=\"0\" CELLBORDER=\"1\" CELLSPACING=\"0\"><TR><TD PORT=\"{5}-address\"><I>{5}</I></TD><TD PORT=\"{5}-type\"><B>alloc::string::String</B></TD><TD PORT=\"{5}-value\">yabadabadoo!</TD></TR></TABLE>>];\n    \n\n}}", struct_address, ref_target_address, u8_address, string_address, ref_address, ref_target_address));
}
