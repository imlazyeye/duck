use super::*;

#[derive(Debug, PartialEq, Clone)]
pub enum Trait {
    FieldOp(FieldOp),
    Derive(Box<Term>),
}

#[derive(Debug, PartialEq, Clone)]
pub enum FieldOp {
    Read(String, Box<Term>),
    Write(String, Box<Term>),
}
impl FieldOp {
    pub fn name(&self) -> &str {
        match self {
            FieldOp::Read(name, _) | FieldOp::Write(name, _) => name,
        }
    }

    pub fn term(&self) -> &Term {
        match self {
            FieldOp::Read(_, term) | FieldOp::Write(_, term) => term.as_ref(),
        }
    }

    pub fn term_mut(&mut self) -> &mut Term {
        match self {
            FieldOp::Read(_, term) | FieldOp::Write(_, term) => term.as_mut(),
        }
    }
}
