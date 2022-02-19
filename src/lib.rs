pub type AnyResult<T = ()> = color_eyre::Result<T>;
mod duck;
pub use crate::duck::*;

pub mod lints;
pub use lints::{Lint, LintCategory, LintLevel, LintReport, LintTag};

mod gml;
pub use gml::*;

pub mod parsing;

#[allow(unused_imports)]
#[macro_use]
extern crate log;
