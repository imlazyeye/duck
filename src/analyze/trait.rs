use super::*;

#[derive(Debug, PartialEq, Clone)]
pub enum Trait {
    FieldOp(String, Box<FieldOp>),
    Callable {
        arguments: Vec<Term>,
        expected_return: Box<Term>,
        uses_new: bool,
    },
}
