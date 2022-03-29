use super::*;

#[derive(Debug, PartialEq, Clone)]
pub enum Trait {
    FieldOp(FieldOp),
    Derive(Box<Term>),
    Callable(Vec<Term>, Box<Term>),
}

#[derive(Debug, PartialEq, Clone)]
pub enum FieldOp {
    Readable(String, Box<Term>),
    Writable(String, Box<Term>),
}
impl FieldOp {
    pub fn name(&self) -> &str {
        match self {
            FieldOp::Readable(name, _) | FieldOp::Writable(name, _) => name,
        }
    }

    pub fn term(&self) -> &Term {
        match self {
            FieldOp::Readable(_, term) | FieldOp::Writable(_, term) => term.as_ref(),
        }
    }

    pub fn term_mut(&mut self) -> &mut Term {
        match self {
            FieldOp::Readable(_, term) | FieldOp::Writable(_, term) => term.as_mut(),
        }
    }
}
