pub type AnyResult<T = ()> = color_eyre::Result<T>;
mod config;
pub use config::Config;
mod duck;
pub mod utils;
pub use crate::duck::*;
pub mod fs;

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
