//! The `Visualize` trait allows any Rust data structure to be graphically represented using
//! GraphViz and Dot.

mod constants;
mod data_description;
mod graph;
mod impls;
mod util;
mod visualize;

pub use crate::data_description::DataDescription;
pub use crate::data_description::Value;
pub use crate::graph::Graph;
pub use crate::visualize::Visualize;
