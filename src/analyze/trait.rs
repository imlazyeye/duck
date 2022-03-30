use hashbrown::HashMap;

use super::*;

#[derive(Debug, PartialEq, Clone)]
pub enum Trait {
    FieldOps(HashMap<String, Box<FieldOp>>),
    Derive(Box<Term>),
    Callable {
        calling_scope: Box<Term>,
        arguments: Vec<Term>,
        expected_return: Box<Term>,
    },
}

#[derive(Debug, PartialEq, Clone)]
pub enum FieldOp {
    Readable(Term),
    Writable(Term),
}
impl FieldOp {
    pub fn term(&self) -> &Term {
        match self {
            FieldOp::Readable(term) | FieldOp::Writable(term) => term,
        }
    }

    pub fn term_mut(&mut self) -> &mut Term {
        match self {
            FieldOp::Readable(term) | FieldOp::Writable(term) => term,
        }
    }
}
