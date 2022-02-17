pub type AnyResult<T = ()> = color_eyre::Result<T>;
mod duck;
pub use duck::*;

mod lints;
pub use lints::*;

mod gml;
pub use gml::*;

mod parsing;
pub use parsing::ParseError;

#[allow(unused_imports)]
#[macro_use]
extern crate log;
