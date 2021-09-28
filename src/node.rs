use crate::{util, DataDescription};

#[derive(Debug, Clone, Default)]
/// A string containing a graphviz graph node
pub struct RenderedNode(String);

impl RenderedNode {
    /// Get the string value for this node
    pub fn inner(&self) -> &str {
        &self.0
    }
}

impl std::fmt::Display for RenderedNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(&self.0, f)
    }
}

impl From<DataDescription> for RenderedNode {
    fn from(data_description: DataDescription) -> Self {
        Self(format!(
            r#"  node [shape=plaintext]
    "{}" [label=<{}>];
    {}"#,
            data_description.address,
            util::render_table(std::iter::once(data_description.render_table_row())),
            data_description.render_references(&data_description.address)
        ))
    }
}

// enable building dummy RenderedNode for tests that ignore it
#[cfg(test)]
impl<T> From<T> for RenderedNode
where
    T: Into<String>,
{
    fn from(t: T) -> Self {
        Self(t.into())
    }
}
