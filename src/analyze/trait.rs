use super::*;
use hashbrown::HashMap;

#[derive(Debug, PartialEq, Clone)]
pub enum Trait {
    Contains(HashMap<String, Term>),
}
