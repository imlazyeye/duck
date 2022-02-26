mod config;
mod duck_operation;
pub use duck_operation::*;
mod duck_task;
pub use config::Config;
pub use duck_task::*;
mod duck;
pub mod utils;
pub use crate::duck::*;

pub mod lints;

mod lint;
pub use lint::{Lint, LintCategory, LintLevel, LintReport, LintTag};

pub mod parsing;

pub mod gml;

#[allow(unused_imports)]
#[macro_use]
extern crate log;

#[macro_use]
extern crate lazy_static;
