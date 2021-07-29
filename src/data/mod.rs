//! Imports and re-exports the data structures whose
//! features have been specified. (Default is all data structures)
//!
//!
#[cfg(feature = "graph")]
pub mod graph;
#[cfg(feature = "chain")]
pub mod chain;

#[cfg(feature = "graph")]
pub use graph::{Graph, Direction, Node, Edge};
