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
pub use lint::{Lint, LintLevel, LintReport, LintTag};

pub mod gml;
pub mod parsing;

pub mod prelude {
    pub use crate::config::*;
    pub use crate::duck::*;
    pub use crate::duck_operation::*;
    pub use crate::duck_task::*;
    pub use crate::gml::*;
    pub use crate::lint::*;
    pub use crate::parsing::*;
    pub use crate::utils::*;
}
