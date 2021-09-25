//! The `Visualize` trait allows any Rust data structure to be graphically represented using
//! GraphViz and Dot.

// set some allowed lints to warnings based on https://rust-unofficial.github.io/patterns/anti_patterns/deny-warnings.html#alternatives
#![warn(
    missing_debug_implementations,
    missing_docs,
    trivial_casts,
    trivial_numeric_casts,
    unused_extern_crates,
    unused_import_braces,
    unused_qualifications,
    unused_results
)]

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
