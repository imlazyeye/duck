#![warn(missing_docs)]
#![warn(clippy::dbg_macro)]
#![warn(clippy::print_stdout)]
#![warn(clippy::map_unwrap_or)]
#![warn(clippy::similar_names)]
#![warn(clippy::todo)]
#![warn(clippy::unimplemented)]
#![warn(clippy::undocumented_unsafe_blocks)]

//! Utilities for parsing and linting Gml.

#[macro_use]
extern crate lazy_static;

mod core {
    /// The core operations duck executes to process a project.
    pub mod driver;
    mod duck;
    pub use crate::core::duck::*;
    mod config;
    pub use config::*;
}
pub use crate::core::*;

/// Basic traits and types associated with lints.
pub mod lint {
    #[allow(clippy::module_inception)]
    mod lint;
    pub use lint::*;

    /// Collection of all of the various lints in duck.
    pub mod collection;

    #[cfg(test)]
    mod tests {
        mod lint_tests;
        mod validity_tests;
    }
}

/// Tools and types used to parse gml into an abstract syntax tree.
pub mod parse {
    mod gml {
        mod expressions {
            mod access;
            mod call;
            mod r#enum;
            mod equality;
            mod evaluation;
            mod function;
            mod grouping;
            mod identifier;
            mod literal;
            mod logical;
            mod r#macro;
            mod null_coalecence;
            mod postfix;
            mod ternary;
            mod unary;
            pub use access::*;
            pub use call::*;
            pub use equality::*;
            pub use evaluation::*;
            pub use function::*;
            pub use grouping::*;
            pub use identifier::*;
            pub use literal::*;
            pub use logical::*;
            pub use null_coalecence::*;
            pub use postfix::*;
            pub use r#enum::*;
            pub use r#macro::*;
            pub use ternary::*;
            pub use unary::*;
        }
        mod statements {
            mod assignment;
            mod block;
            mod delete;
            mod do_until;
            mod for_loop;
            mod globalvar;
            mod r#if;
            mod local_variable;
            mod repeat_loop;
            mod r#return;
            mod switch;
            mod throw;
            mod try_catch;
            mod while_loop;
            mod with_loop;
            pub use assignment::*;
            pub use block::*;
            pub use delete::*;
            pub use do_until::*;
            pub use for_loop::*;
            pub use globalvar::*;
            pub use local_variable::*;
            pub use r#if::*;
            pub use r#return::*;
            pub use repeat_loop::*;
            pub use switch::*;
            pub use throw::*;
            pub use try_catch::*;
            pub use while_loop::*;
            pub use with_loop::*;
        }
        mod expr;
        mod optional_initialization;
        mod stmt;
        mod token;
        pub use expr::*;
        pub use expressions::*;
        pub use optional_initialization::*;
        pub use statements::*;
        pub use stmt::*;
        pub use token::*;
    }
    mod ast;
    mod lexer;
    mod parser;
    mod utils;
    pub use ast::*;
    pub use gml::*;
    pub use lexer::*;
    pub use parser::*;
    pub use utils::*;

    #[cfg(test)]
    mod tests;
}

/// The future home of static-analysis features, but currently just home to [GlobalScope].
#[allow(missing_docs)]
pub mod analyze {
    mod global_scope;
    mod goals;
    mod record;
    mod solver;
    mod ty;
    mod unify;
    mod utils;
    pub use global_scope::*;
    pub use goals::*;
    pub use record::*;
    pub use solver::*;
    pub use ty::*;
    pub use unify::*;
    pub use utils::*;
    #[cfg(test)]
    mod tests {
        mod error_tests;
        mod type_tests;
        mod utils;
        pub use utils::*;
    }
}
