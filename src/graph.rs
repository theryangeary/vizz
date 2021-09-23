use std::io::{Result, Write};

use crate::Visualize;

pub struct Graph {
    buffer: String,
}

impl Graph {
    pub fn new() -> Graph {
        Graph {
            buffer: String::new(),
        }
    }

    pub fn add<V>(self, node: &V) -> Graph
    where
        V: Visualize,
    {
        Graph {
            buffer: self.buffer + &node.render_node(),
        }
    }

    pub fn write_to<W: Write>(self, writer: &mut W) -> Result<()> {
        write!(
            writer,
            r#"
digraph structs {{
    {}
}}"#,
            self.buffer
        )
    }
}
