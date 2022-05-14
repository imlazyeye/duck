use crate::parse::{Expr, ExprKind, IntoExpr, Field, ParseVisitor, Stmt, StmtKind};

use super::Identifier;

/// Representation of function declaration in gml.
#[derive(Debug, PartialEq, Clone, serde::Serialize)]
pub struct Function {
    /// The name, if any, of this function. Anonymous functions do not have names.
    #[serde(flatten)]
    pub name: Option<Identifier>,
    /// The parameters of this function.
    pub parameters: Vec<Field>,
    /// The constructor behavior of this function, if any.
    pub constructor: Option<Constructor>,
    /// The body of the function declaration.
    pub body: Stmt,
}
impl Function {
    /// Creates a new function declaration.
    pub fn new(name: Identifier, parameters: Vec<Field>, body: Stmt) -> Self {
        Self {
            name: Some(name),
            parameters,
            constructor: None,
            body,
        }
    }

    /// Creates a new anonymous function declaration.
    pub fn new_anonymous(parameters: Vec<Field>, body: Stmt) -> Self {
        Self {
            name: None,
            parameters,
            constructor: None,
            body,
        }
    }

    /// Creates a new constructor declaration.
    pub fn new_constructor(
        name: Option<Identifier>,
        parameters: Vec<Field>,
        constructor: Constructor,
        body: Stmt,
    ) -> Self {
        Self {
            name,
            parameters,
            constructor: Some(constructor),
            body,
        }
    }

    /// Returns the list of statements in this function's body
    pub fn body_stmts(&self) -> &Vec<Stmt> {
        match self.body.kind() {
            StmtKind::Block(body) => &body.body,
            _ => unreachable!(),
        }
    }
}
impl From<Function> for ExprKind {
    fn from(function: Function) -> Self {
        Self::Function(function)
    }
}
impl IntoExpr for Function {}
impl ParseVisitor for Function {
    fn visit_child_exprs<E: FnMut(&Expr)>(&self, mut visitor: E) {
        for param in self.parameters.iter() {
            match param {
                Field::Uninitialized(expr) => visitor(expr),
                Field::Initialized(_) => {}
            }
        }
        if let Some(Constructor { inheritance: Some(call) }) = &self.constructor {
            visitor(call);
        }
    }
    fn visit_child_exprs_mut<E: FnMut(&mut Expr)>(&mut self, mut visitor: E) {
        for param in self.parameters.iter_mut() {
            match param {
                Field::Uninitialized(expr) => visitor(expr),
                Field::Initialized(_) => {}
            }
        }
        if let Some(Constructor { inheritance: Some(call) }) = &mut self.constructor {
            visitor(call);
        }
    }
    fn visit_child_stmts<S: FnMut(&Stmt)>(&self, mut visitor: S) {
        for param in self.parameters.iter() {
            match param {
                Field::Uninitialized(_) => {}
                Field::Initialized(stmt) => visitor(stmt),
            }
        }
        visitor(&self.body);
    }
    fn visit_child_stmts_mut<S: FnMut(&mut Stmt)>(&mut self, mut visitor: S) {
        for param in self.parameters.iter_mut() {
            match param {
                Field::Uninitialized(_) => {}
                Field::Initialized(stmt) => visitor(stmt),
            }
        }
        visitor(&mut self.body);
    }
}

/// Representation of a constructor's behavior in a function declaration.
#[derive(Debug, PartialEq, Clone, serde::Serialize)]
pub struct Constructor {
    /// The inheritance call this constructor has, if any.
    pub inheritance: Option<Expr>,
}
