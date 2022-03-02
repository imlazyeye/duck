#![warn(missing_docs)]
#![warn(clippy::dbg_macro)]
#![warn(clippy::print_stdout)]
#![warn(clippy::map_unwrap_or)]
#![warn(clippy::missing_errors_doc)]
#![warn(clippy::missing_panics_doc)]
#![warn(clippy::similar_names)]
#![warn(clippy::todo)]
#![warn(clippy::unimplemented)]
#![warn(clippy::too_many_lines)]
#![warn(clippy::undocumented_unsafe_blocks)]
#![warn(clippy::use_self)]

//! Utilities for parsing and linting Gml.

mod core;
pub use crate::core::*;

/// Basic traits and types associated with lints.
pub mod lint;

/// Collection of all of the various lints in duck.
pub mod lints;

/// Utilities used widely around the duck codebase.
pub mod utils;

/// Tools and types used to parse gml into an abstract syntax tree.
pub mod parsing;

/// The future home of static-analysis features, but currently just home to [GlobalScope].
pub mod analysis;

/// A collection of all of core features used in duck.
pub mod prelude {
    pub use crate::{analysis::*, core::*, lint::*, parsing::*, utils::*};
}
