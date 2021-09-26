use std::io::{Result, Write};

use crate::Visualize;

#[derive(Debug, Clone)]
/// A struct for building a graph
///
/// This is the main tool end users will want to use to generate graph visualizations.
///
/// # Example
///
/// ```
/// use vizz::Graph;
/// use std::fs::File;
///
/// struct MyStruct<'a> {
///     my_u8: u8,
///     my_string: String,
///     my_ref: &'a String,
/// }
///
/// # use vizz::Visualize;
/// # use vizz::DataDescription;
/// #
/// # impl<'a> Visualize for MyStruct<'a> {
/// #     fn associated_data(&self) -> Option<Vec<DataDescription>> {
/// #         Some(vec![
/// #             DataDescription::from(&self.my_u8).with_label("my_u8"),
/// #             DataDescription::from(&self.my_string).with_label("my_string"),
/// #             DataDescription::from(&self.my_ref).with_label("my_ref"),
/// #         ])
/// #     }
/// # }
/// #
/// // create some data
/// let unowned_string = String::from("this is referenced!");
/// let my_struct = MyStruct {
///     my_u8: 42,
///     my_string: "this is owned!".into(),
///     my_ref: &unowned_string,
/// };
///
/// // create file
/// let mut dot_file = File::create("/tmp/my_struct.dot").unwrap();
///
/// // create graph
/// Graph::new()
///     .set_id("my_test_graph")
///     .add_node(&my_struct)
///     .add_node(&unowned_string)
///     .write_to(&mut dot_file)
///     .unwrap();
/// ```
pub struct Graph {
    /// The ID of the graph in the DOT language grammar
    id: String,
    /// The string containing the dot file contents, to eventually be written to a file
    buffer: String,
}

impl Graph {
    /// Create a new graph
    pub fn new() -> Graph {
        Graph {
            id: String::from("visualization"),
            buffer: String::new(),
        }
    }

    /// Set the ID for the graph
    ///
    /// See the DOT language grammar ID for more info: <https://graphviz.org/doc/info/lang.html>
    pub fn set_id(self, new_id: impl Into<String>) -> Graph {
        Graph {
            id: new_id.into(),
            ..self
        }
    }

    /// Add a data structure that implements [Visualize] to the [Graph]
    pub fn add_node<V>(self, node: &V) -> Graph
    where
        V: Visualize,
    {
        Graph {
            buffer: self.buffer + &node.render_node(),
            ..self
        }
    }

    /// Create the full DOT graph file contents as a [String]
    pub fn render(&self) -> String {
        format!(
            r#"digraph {} {{
{}
}}"#,
            self.id, self.buffer
        )
    }

    /// Write the DOT file to the filesystem
    pub fn write_to<W: Write>(self, writer: &mut W) -> Result<()> {
        write!(writer, "{}", self.render())
    }
}

impl Default for Graph {
    fn default() -> Self {
        Graph::new()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::util;

    #[test]
    fn test_render_graph() {
        let target = 8u8;
        let target_address_string = util::address_of(&target);
        let graph_id = "test_generate_graph";
        let graph = Graph::new().set_id(graph_id).add_node(&target);
        assert_eq!(graph.render(), format!("digraph {1} {{\n  node [shape=plaintext]\n    \"{0}\" [label=<<TABLE BORDER=\"0\" CELLBORDER=\"1\" CELLSPACING=\"0\"><TR><TD PORT=\"{0}-address\"><I>{0}</I></TD><TD PORT=\"{0}-type\"><B>u8</B></TD><TD PORT=\"{0}-value\">8</TD></TR></TABLE>>];\n    \n}}", target_address_string, graph_id));
    }
}
