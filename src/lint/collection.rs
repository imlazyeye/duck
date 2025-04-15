#![allow(missing_docs)]
mod accessor_alternative;
pub use accessor_alternative::AccessorAlternative;
mod and_preference;
pub use and_preference::AndPreference;
mod anonymous_constructor;
pub use anonymous_constructor::AnonymousConstructor;
mod bool_equality;
pub use bool_equality::BoolEquality;
mod casing_rules;
pub use casing_rules::CasingRules;
mod collapsable_if;
pub use collapsable_if::CollapsableIf;
mod condition_wrapper;
pub use condition_wrapper::ConditionWrapper;
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
mod invalid_assignment;
pub use invalid_assignment::InvalidAssignment;
mod invalid_comparison;
pub use invalid_comparison::InvalidComparison;
mod invalid_equality;
pub use invalid_equality::InvalidEquality;
mod missing_case_member;
pub use missing_case_member::MissingCaseMember;
mod missing_default_case;
pub use missing_default_case::MissingDefaultCase;
mod mod_preference;
pub use mod_preference::ModPreference;
mod multi_var_declaration;
pub use multi_var_declaration::MultiVarDeclaration;
mod non_constant_default_parameter;
pub use non_constant_default_parameter::NonConstantDefaultParameter;
mod non_simplified_expression;
pub use non_simplified_expression::NonSimplifiedExpression;
mod not_preference;
pub use not_preference::NotPreference;
mod or_preference;
pub use or_preference::OrPreference;
mod room_goto;
pub use room_goto::RoomGoto;
mod show_debug_message;
pub use show_debug_message::ShowDebugMessage;
mod single_equals_comparison;
pub use single_equals_comparison::SingleEqualsComparison;
mod single_switch_case;
pub use single_switch_case::SingleSwitchCase;
mod suspicious_constant_usage;
pub use suspicious_constant_usage::SuspicousConstantUsage;
mod switch_without_case;
pub use switch_without_case::SwitchWithoutCase;
mod todo;
pub use todo::Todo;
mod too_many_arguments;
pub use too_many_arguments::TooManyArguments;
mod try_catch;
pub use try_catch::TryCatch;
mod unassigned_constructor;
pub use unassigned_constructor::UnassignedConstructor;
mod unnecessary_grouping;
pub use unnecessary_grouping::UnnecessaryGrouping;
mod unused_parameter;
pub use unused_parameter::UnusedParameter;
mod useless_function;
pub use useless_function::UselessFunction;
mod var_prefix_violation;
pub use var_prefix_violation::VarPrefixViolation;
mod with_loop;
pub use with_loop::WithLoop;
