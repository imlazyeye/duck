use super::*;
use hashbrown::HashMap;

#[derive(Debug, PartialEq, Clone)]
pub enum App {
    Array(Box<Term>),
    Object(HashMap<String, FieldOp>),
    Function {
        self_fields: Option<HashMap<String, FieldOp>>,
        parameters: Vec<Term>,
        return_type: Box<Term>,
    },
    Union(Vec<Term>),
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
