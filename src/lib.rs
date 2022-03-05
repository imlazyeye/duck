#![warn(missing_docs)]
#![warn(clippy::dbg_macro)]
#![warn(clippy::print_stdout)]
#![warn(clippy::map_unwrap_or)] // gabe this was a mistake
#![warn(clippy::missing_errors_doc)]
#![warn(clippy::missing_panics_doc)]
#![warn(clippy::similar_names)]
#![warn(clippy::todo)]
#![warn(clippy::unimplemented)]
#![warn(clippy::too_many_lines)]
#![warn(clippy::undocumented_unsafe_blocks)]

//! Utilities for parsing and linting Gml.

mod core {
    mod duck;
    mod duck_operation;
    mod duck_task;
    pub use crate::core::duck::*;
    pub use duck_operation::*;
    pub use duck_task::*;
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
    pub mod collection {
        #![allow(missing_docs)]
        mod accessor_alternative;
        pub use accessor_alternative::AccessorAlternative;
        mod and_preference;
        pub use and_preference::AndPreference;
        mod anonymous_constructor;
        pub use anonymous_constructor::AnonymousConstructor;
        mod assignment_to_call;
        pub use assignment_to_call::AssignmentToCall;
        mod bool_equality;
        pub use bool_equality::BoolEquality;
        mod collapsable_if;
        pub use collapsable_if::CollapsableIf;
        mod deprecated;
        pub use deprecated::Deprecated;
        mod draw_sprite;
        pub use draw_sprite::DrawSprite;
        mod draw_text;
        pub use draw_text::DrawText;
        mod english_flavor_violation;
        pub use english_flavor_violation::EnglishFlavorViolation;
        mod exit;
        pub use exit::Exit;
        mod global;
        pub use global::Global;
        mod missing_case_member;
        pub use missing_case_member::MissingCaseMember;
        mod missing_default_case;
        pub use missing_default_case::MissingDefaultCase;
        mod mod_preference;
        pub use mod_preference::ModPreference;
        mod multi_var_declaration;
        pub use multi_var_declaration::MultiVarDeclaration;
        mod no_space_begining_comment;
        pub use no_space_begining_comment::NoSpaceBeginingComment;
        mod non_constant_default_parameter;
        pub use non_constant_default_parameter::NonConstantDefaultParameter;
        mod non_pascal_case;
        pub use non_pascal_case::NonPascalCase;
        mod non_scream_case;
        pub use non_scream_case::NonScreamCase;
        mod not_preference;
        pub use not_preference::NotPreference;
        mod or_preference;
        pub use or_preference::OrPreference;
        mod room_goto;
        pub use room_goto::RoomGoto;
        mod show_debug_message;
        pub use show_debug_message::ShowDebugMessage;
        mod single_switch_case;
        pub use single_switch_case::SingleSwitchCase;
        mod statement_parenthetical_preference;
        pub use statement_parenthetical_preference::StatementParentheticalPreference;
        mod suspicious_constant_usage;
        pub use suspicious_constant_usage::SuspicousConstantUsage;
        mod todo;
        pub use todo::Todo;
        mod too_many_arguments;
        pub use too_many_arguments::TooManyArguments;
        mod too_many_lines;
        pub use too_many_lines::TooManyLines;
        mod try_catch;
        pub use try_catch::TryCatch;
        mod var_prefix_violation;
        pub use var_prefix_violation::VarPrefixViolation;
        mod with_loop;
        pub use with_loop::WithLoop;
    }
}

/// Utilities used widely around the duck codebase.
pub mod utils;

/// Tools and types used to parse gml into an abstract syntax tree.
pub mod parse {
    mod gml {
        mod expressions {
            mod access;
            mod call;
            mod equality;
            mod evaluation;
            mod grouping;
            mod identifier;
            mod literal;
            mod logical;
            mod null_coalecence;
            mod postfix;
            mod ternary;
            mod unary;
            pub use access::*;
            pub use call::*;
            pub use equality::*;
            pub use evaluation::*;
            pub use grouping::*;
            pub use identifier::*;
            pub use literal::*;
            pub use logical::*;
            pub use null_coalecence::*;
            pub use postfix::*;
            pub use ternary::*;
            pub use unary::*;
        }
        mod statements {
            mod assignment;
            mod block;
            mod do_until;
            mod r#enum;
            mod for_loop;
            mod function;
            mod globalvar;
            mod r#if;
            mod local_variable;
            mod r#macro;
            mod repeat_loop;
            mod r#return;
            mod switch;
            mod try_catch;
            mod while_loop;
            mod with_loop;
            pub use assignment::*;
            pub use block::*;
            pub use do_until::*;
            pub use for_loop::*;
            pub use function::*;
            pub use globalvar::*;
            pub use local_variable::*;
            pub use r#enum::*;
            pub use r#if::*;
            pub use r#macro::*;
            pub use r#return::*;
            pub use repeat_loop::*;
            pub use switch::*;
            pub use try_catch::*;
            pub use while_loop::*;
            pub use with_loop::*;
        }
        mod expression;
        mod statement;
        mod token;
        pub use expression::*;
        pub use expressions::*;
        pub use statement::*;
        pub use statements::*;
        pub use token::*;
    }
    mod lexer;
    mod parse_error;
    mod parser;
    pub use gml::*;
    pub use lexer::*;
    pub use parse_error::*;
    pub use parser::*;

    #[cfg(test)]
    mod tests;
}

/// The future home of static-analysis features, but currently just home to [GlobalScope].
pub mod analyze {
    mod global_scope;
    pub use global_scope::*;
}
