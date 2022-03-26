use super::*;
use crate::parse::{Block, Function, StmtType};
use hashbrown::HashMap;

#[derive(Debug, PartialEq, Clone)]
pub enum App {
    Array(Box<Term>),
    Object(HashMap<String, Term>),
    Function(Vec<Term>, Box<Term>, Function),
}
impl App {
    pub fn process_function(function: Function, page: &mut Page, printer: &mut Printer) -> (Vec<Term>, Box<Term>) {
        let body = match function.body.inner() {
            StmtType::Block(Block { body, .. }) => body,
            _ => unreachable!(),
        };
        page.apply_stmts(body, printer);
        let mut parameters = Vec::new();
        for param in function.parameters.iter() {
            let param_marker = page.scope.get_expr_marker(param.name_expr());
            let param_term = page.marker_to_term(param_marker);
            parameters.push(param_term);
        }
        (parameters, Box::new(page.return_term()))
    }
}
