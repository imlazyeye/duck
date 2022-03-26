use super::*;
use hashbrown::HashMap;

#[derive(Debug, PartialEq, Clone)]
pub enum Impl {
    Fields(HashMap<String, Term>),
}
