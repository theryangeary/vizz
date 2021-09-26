use crate::data_description::{DataDescription, Value};
use crate::util;

/// A trait for defining how to visually represent a type
///
/// For manual implementations, note that each of these methods has a default, so implementors need
/// only implement the methods they need to modify.
pub trait Visualize: Sized {
    /// How to represent the data for this type, if at all
    ///
    /// # Primitive Example
    ///
    /// For primitives, this will likely be the primitive's value. This example is ignored because
    /// it conflicts with the implementation of [Visualize] for [u8] provided by this crate.
    ///
    /// ```ignore
    /// use vizz::Value;
    /// use vizz::Visualize;
    ///
    /// impl Visualize for u8 {
    ///     fn data(&self) -> Option<Value> {
    ///         Some(Value::Owned(self.to_string()))
    ///     }
    /// }
    /// ```
    ///
    /// # Enum Example
    ///
    /// For enums, this will likely be the variant name.
    ///
    /// ```
    /// use vizz::Value;
    /// use vizz::Visualize;
    ///
    /// enum MyEnum {
    ///     SimpleVariant,
    ///     VariantWithData(u8),
    /// }
    ///
    /// impl Visualize for MyEnum {
    ///     fn data(&self) -> Option<Value> {
    ///         Some(Value::Owned(
    ///             match self {
    ///                 MyEnum::SimpleVariant => "SimpleVariant",
    ///                 MyEnum::VariantWithData(_) => "VariantWithData",
    ///             }.into()
    ///         ))
    ///     }
    /// }
    /// ```
    ///
    /// # Struct Example
    ///
    /// For structs, this will likely be [None] because structs don't have any inherent data, but
    /// instead contain other types, and therefore the default method implementation need not be
    /// overridden. The data contained in a struct should be used in [Visualize::associated_data].
    fn data(&self) -> Option<Value> {
        None
    }

    /// The list of associated data structures that should be included in a visualization of this
    /// data type
    ///
    /// For named associated data, such as struct members or named enum variant members, the
    /// [DataDescription::with_label] method can be used to add a label.
    ///
    /// # Primitive Example
    ///
    /// For primitives, this will likely be [None], and therefore the default method implementation
    /// need not be overridden.
    ///
    /// # Enum Example
    ///
    /// For enums with associated data, this will likely be those values. For enums with no
    /// associated data, this will likely be [None].
    ///
    /// ```
    /// use vizz::Visualize;
    /// use vizz::DataDescription;
    ///
    /// enum MyEnum {
    ///     SimpleVariant,
    ///     VariantWithData(u8),
    /// }
    ///
    /// impl Visualize for MyEnum {
    ///     fn associated_data(&self) -> Option<Vec<DataDescription>> {
    ///         match self {
    ///             MyEnum::SimpleVariant => None,
    ///             MyEnum::VariantWithData(data) => Some(vec![DataDescription::from(data)]),
    ///         }
    ///     }
    /// }
    /// ```
    ///
    /// # Struct Example
    ///
    /// For structs, this will likely be the struct members.
    ///
    /// ```
    /// use vizz::Visualize;
    /// use vizz::DataDescription;
    ///
    /// struct MyStruct<'a> {
    ///     my_string: String,
    ///     my_ref: &'a String,
    /// }
    ///
    /// impl<'a> Visualize for MyStruct<'a> {
    ///     fn associated_data(&self) -> Option<Vec<DataDescription>> {
    ///         Some(vec![
    ///             DataDescription::from(&self.my_string).with_label("my_string"),
    ///             DataDescription::from(&self.my_ref).with_label("my_ref"),
    ///         ])
    ///     }
    /// }
    /// ```
    fn associated_data(&self) -> Option<Vec<DataDescription>> {
        None
    }

    /// Render the node for this data
    ///
    /// N.B. that this node is useless on its own and must be put in the context of a [Graph]. End
    /// users seeking to create an actual visualization should use the [Graph] struct instead of
    /// manually trying to use this method.
    ///
    /// [Graph]: crate::Graph
    fn render_node(&self) -> String {
        let data_description = DataDescription::from(self);
        format!(
            r#"  node [shape=plaintext]
    "{}" [label=<{}>];
    {}"#,
            data_description.hex_address_string,
            util::render_table(std::iter::once(data_description.render_table_row())),
            data_description.render_references(&data_description.hex_address_string)
        )
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_render_node() {
        let target = 8u8;
        let target_address_string = util::address_of(&target);
        assert_eq!((&target).render_node(), format!("  node [shape=plaintext]\n    \"{0}\" [label=<<TABLE BORDER=\"0\" CELLBORDER=\"1\" CELLSPACING=\"0\"><TR><TD PORT=\"{0}-address\"><I>{0}</I></TD><TD PORT=\"{0}-type\"><B>u8</B></TD><TD PORT=\"{0}-value\">8</TD></TR></TABLE>>];\n    ", target_address_string))
    }
}
