#![warn(missing_docs)]

//! Utilities and operations for parsing and linting Gml.

mod config;
pub use config::Config;
mod duck_operation;
pub use duck_operation::DuckOperation;

mod duck_task;
pub use duck_task::DuckTask;
mod duck;
pub use crate::duck::*;

pub mod lints;

/// Utilities used widely around the duck codebase.
pub mod utils;

mod lint;
pub use lint::{Lint, LintLevel, LintReport, LintTag};

/// Type definitions for various symbols in gml.
pub mod gml;

/// Tools used to parse gml into an abstract syntax tree.
pub mod parsing;

/// A collection of all of core features used in duck.
pub mod prelude {
    pub use crate::{config::*, duck::*, duck_operation::*, duck_task::*, gml::*, lint::*, parsing::*, utils::*};
}
