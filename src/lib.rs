pub type AnyResult<T = ()> = color_eyre::Result<T>;
mod clippie;
pub use clippie::*;

mod issues;
pub use issues::*;

mod gml;
pub use gml::*;

mod parsing;
pub use parsing::ClippieParseError;

#[allow(unused_imports)]
#[macro_use]
extern crate log;
