use super::*;
use hashbrown::HashMap;

#[derive(Debug, PartialEq, Clone)]
pub enum App {
    Array(Box<Term>),
    Object(HashMap<String, Term>),
    Function {
        self_parameter: Option<Box<Term>>,
        parameters: Vec<Term>,
        return_type: Box<Term>,
    },
}
