//! The purpose of this crate is to provide implementations of
//! data structures & algorithms not in the standard library,
//! and to provide implementations of those already in the library
//! with personal design preferences or other ad-hoc changes.
//! Intended for personal use in projects developed by me,
//! but anyone is free to use should they find it useful.
//!
//! Currently, only the graph data structure is (kind of)
//! somewhat implemented. Very much so a work in progress!
//! ```rust
//! use recollection::data::{Graph, Node, Edge};
//! let mut g = Graph::<String, usize>::new_directed();
//! let n1: usize = g.add("First node!")?;
//! let n2: usize = g.add("Second node!")?;
//! let e1: Edge<usize> = g.add_edge(n1, n2, 10)?;
//! assert_eq!(g.node_count(), 2);
//! assert_eq!(g.edge_count(), 1);
//!
//! ````
pub mod data;
pub mod error;
pub mod prelude;
#[cfg(feature = "fs")]
pub mod fs;
#[cfg(feature = "config")]
pub mod config;
#[cfg(feature = "channel")]
pub mod channel;

#[cfg(feature = "derive")]
pub use recollection_derive::*;
pub use error::{RecolError, RecolResult};

pub fn init() -> RecolResult<()> {
    Ok(())
}

