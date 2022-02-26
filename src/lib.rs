mod config;
pub use config::Config;
mod duck_operation;
pub use duck_operation::DuckOperation;
mod duck_task;
pub use duck_task::DuckTask;
mod duck;
pub use crate::duck::*;

pub mod lints;
pub mod utils;

mod lint;
pub use lint::{Lint, LintCategory, LintLevel, LintReport, LintTag};

pub mod gml;
pub mod parsing;

#[macro_use]
extern crate lazy_static;
