pub type AnyResult<T = ()> = color_eyre::Result<T>;
mod duck;
pub use crate::duck::*;

pub mod lints;

mod lint;
pub use lint::{Lint, LintCategory, LintLevel, LintReport, LintTag};

pub mod parsing;

#[allow(unused_imports)]
#[macro_use]
extern crate log;

#[macro_use]
extern crate lazy_static;
