use vizz::{Graph, Visualize};

#[derive(Visualize)]
enum MyEnum {
    Plain,
    WithU8(u8),
    WithU8AndString(u8, String),
    WithStruct { my_u8: u8, my_string: String },
}

#[test]
fn test_graph() {
    let plain_enum = MyEnum::Plain;
    let enum_with_u8_and_string = MyEnum::WithU8AndString(6, String::from("hey"));
    let enum_with_u8 = MyEnum::WithU8(10);
    let enum_with_named_fields = MyEnum::WithStruct {
        my_u8: 8,
        my_string: String::from("hey hey mic check 1 2 3"),
    };

    let plain_address = vizz::util::address_of(&plain_enum);
    let enum_with_u8_and_string_address = vizz::util::address_of(&enum_with_u8_and_string);
    let (enum_with_u8_and_string_address_inner0, enum_with_u8_and_string_address_inner1) =
        if let MyEnum::WithU8AndString(a, b) = &enum_with_u8_and_string {
            (vizz::util::address_of(a), vizz::util::address_of(b))
        } else {
            panic!("how could this be the wrong variant")
        };
    let enum_with_u8_address = vizz::util::address_of(&enum_with_u8);
    let enum_with_u8_address_inner = if let MyEnum::WithU8(a) = &enum_with_u8 {
        vizz::util::address_of(a)
    } else {
        panic!("how could this be the wrong variant")
    };
    let enum_with_named_fields_address = vizz::util::address_of(&enum_with_named_fields);
    let (enum_with_named_fields_u8_address, enum_with_named_fields_string_address) =
        if let MyEnum::WithStruct { my_u8, my_string } = &enum_with_named_fields {
            (
                vizz::util::address_of(my_u8),
                vizz::util::address_of(my_string),
            )
        } else {
            panic!("how could this be the wrong variant")
        };

    assert_eq!(
        Graph::new()
            .add_node(&plain_enum)
            .add_node(&enum_with_u8)
            .add_node(&enum_with_named_fields)
            .add_node(&enum_with_u8_and_string)
            .render(),
            format!("digraph visualization {{\n  node [shape=plaintext]\n    \"{0}\" [label=<<TABLE BORDER=\"0\" CELLBORDER=\"1\" CELLSPACING=\"0\"><TR><TD PORT=\"{0}-address\"><I>{0}</I></TD><TD PORT=\"{0}-type\"><B>enum::MyEnum</B></TD><TD PORT=\"{0}-value\">Plain</TD></TR></TABLE>>];\n      node [shape=plaintext]\n    \"{4}\" [label=<<TABLE BORDER=\"0\" CELLBORDER=\"1\" CELLSPACING=\"0\"><TR><TD PORT=\"{4}-address\"><I>{4}</I></TD><TD PORT=\"{4}-type\"><B>enum::MyEnum</B></TD><TD PORT=\"{4}-value\">WithU8</TD><TD PORT=\"{4}-associated-data\"><TABLE BORDER=\"0\" CELLBORDER=\"1\" CELLSPACING=\"0\"><TR><TD PORT=\"{5}-address\"><I>{5}</I></TD><TD PORT=\"{5}-type\"><B>u8</B></TD><TD PORT=\"{5}-value\">10</TD></TR></TABLE></TD></TR></TABLE>>];\n      node [shape=plaintext]\n    \"{6}\" [label=<<TABLE BORDER=\"0\" CELLBORDER=\"1\" CELLSPACING=\"0\"><TR><TD PORT=\"{6}-address\"><I>{6}</I></TD><TD PORT=\"{6}-type\"><B>enum::MyEnum</B></TD><TD PORT=\"{6}-value\">WithStruct</TD><TD PORT=\"{6}-associated-data\"><TABLE BORDER=\"0\" CELLBORDER=\"1\" CELLSPACING=\"0\"><TR><TD PORT=\"{7}-label\">my_u8</TD><TD PORT=\"{7}-address\"><I>{7}</I></TD><TD PORT=\"{7}-type\"><B>u8</B></TD><TD PORT=\"{7}-value\">8</TD></TR><TR><TD PORT=\"{8}-label\">my_string</TD><TD PORT=\"{8}-address\"><I>{8}</I></TD><TD PORT=\"{8}-type\"><B>alloc::string::String</B></TD><TD PORT=\"{8}-value\">hey hey mic check 1 2 3</TD></TR></TABLE></TD></TR></TABLE>>];\n      node [shape=plaintext]\n    \"{1}\" [label=<<TABLE BORDER=\"0\" CELLBORDER=\"1\" CELLSPACING=\"0\"><TR><TD PORT=\"{1}-address\"><I>{1}</I></TD><TD PORT=\"{1}-type\"><B>enum::MyEnum</B></TD><TD PORT=\"{1}-value\">WithU8AndString</TD><TD PORT=\"{1}-associated-data\"><TABLE BORDER=\"0\" CELLBORDER=\"1\" CELLSPACING=\"0\"><TR><TD PORT=\"{2}-address\"><I>{2}</I></TD><TD PORT=\"{2}-type\"><B>u8</B></TD><TD PORT=\"{2}-value\">6</TD></TR><TR><TD PORT=\"{3}-address\"><I>{3}</I></TD><TD PORT=\"{3}-type\"><B>alloc::string::String</B></TD><TD PORT=\"{3}-value\">hey</TD></TR></TABLE></TD></TR></TABLE>>];\n    \n}}", plain_address, enum_with_u8_and_string_address, enum_with_u8_and_string_address_inner0, enum_with_u8_and_string_address_inner1, enum_with_u8_address, enum_with_u8_address_inner, enum_with_named_fields_address, enum_with_named_fields_u8_address, enum_with_named_fields_string_address)
    );
}
