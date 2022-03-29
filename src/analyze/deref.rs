use super::*;

#[derive(Debug, PartialEq, Clone)]
pub enum Deref {
    Field { field_name: String, target: Box<Term> },
    MemberType { target: Box<Term> },
}
