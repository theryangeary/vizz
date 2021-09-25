use std::io::{Result, Write};

use crate::Visualize;

#[derive(Debug, Clone)]
pub struct Graph {
    name: String,
    buffer: String,
}

impl Graph {
    pub fn new() -> Graph {
        Graph {
            name: String::from("visualization"),
            buffer: String::new(),
        }
    }

    pub fn set_name(self, new_name: impl Into<String>) -> Graph {
        Graph {
            name: new_name.into(),
            ..self
        }
    }

    pub fn add_node<V>(self, node: &V) -> Graph
    where
        V: Visualize,
    {
        Graph {
            buffer: self.buffer + &node.render_node(),
            ..self
        }
    }

    pub fn render(&self) -> String {
        format!(
            r#"digraph {} {{
{}
}}"#,
            self.name, self.buffer
        )
    }

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
    fn test_generate_graph() {
        let target = 8u8;
        let target_address_string = util::address_of(&target);
        let graph_name = "test_generate_graph";
        let graph = Graph::new().set_name(graph_name).add_node(&target);
        assert_eq!(graph.render(), format!("digraph {1} {{\n  node [shape=plaintext]\n    \"{0}\" [label=<<TABLE BORDER=\"0\" CELLBORDER=\"1\" CELLSPACING=\"0\"><TR><TD PORT=\"{0}-address\"><I>{0}</I></TD><TD PORT=\"{0}-type\"><B>u8</B></TD><TD PORT=\"{0}-value\">8</TD></TR></TABLE>>];\n    \n}}", target_address_string, graph_name));
    }
}
