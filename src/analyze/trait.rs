use hashbrown::HashMap;

use super::*;

#[derive(Debug, PartialEq, Clone)]
pub enum Trait {
    FieldOp(String, Box<FieldOp>),
    Callable {
        calling_scope: HashMap<String, FieldOp>,
        arguments: Vec<Term>,
        expected_return: Box<Term>,
        uses_new: bool,
    },
}
