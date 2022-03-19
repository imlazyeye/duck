use crate::parse::{Block, Expr, ExprType, IntoExpr, OptionalInitilization, ParseVisitor, Stmt, StmtType};

use super::Identifier;

/// Representation of function declaration in gml.
#[derive(Debug, PartialEq, Clone)]
pub struct Function {
    /// The name, if any, of this function. Anonymous functions do not have names.
    pub name: Option<Identifier>,
    /// The parameters of this function.
    pub parameters: Vec<OptionalInitilization>,
    /// The constructor behavior of this function, if any.
    pub constructor: Option<Constructor>,
    /// The body of the function declaration.
    pub body: Stmt,
}
impl Function {
    /// Creates a new function declaration.
    pub fn new(name: Identifier, parameters: Vec<OptionalInitilization>, body: Stmt) -> Self {
        Self {
            name: Some(name),
            parameters,
            constructor: None,
            body,
        }
    }

    /// Creates a new anonymous function declaration.
    pub fn new_anonymous(parameters: Vec<OptionalInitilization>, body: Stmt) -> Self {
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
        parameters: Vec<OptionalInitilization>,
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
}
impl From<Function> for ExprType {
    fn from(function: Function) -> Self {
        Self::FunctionDeclaration(function)
    }
}
impl IntoExpr for Function {}
impl ParseVisitor for Function {
    fn visit_child_exprs<E: FnMut(&Expr)>(&self, mut visitor: E) {
        for param in self.parameters.iter() {
            match param {
                OptionalInitilization::Uninitialized(expr) => visitor(expr),
                OptionalInitilization::Initialized(_) => {}
            }
        }
        if let Some(Constructor::WithInheritance(inheritance_call)) = &self.constructor {
            visitor(inheritance_call);
        }
    }
    fn visit_child_exprs_mut<E: FnMut(&mut Expr)>(&mut self, mut visitor: E) {
        for param in self.parameters.iter_mut() {
            match param {
                OptionalInitilization::Uninitialized(expr) => visitor(expr),
                OptionalInitilization::Initialized(_) => {}
            }
        }
        if let Some(Constructor::WithInheritance(inheritance_call)) = &mut self.constructor {
            visitor(inheritance_call);
        }
    }
    fn visit_child_stmts<S: FnMut(&Stmt)>(&self, mut visitor: S) {
        for param in self.parameters.iter() {
            match param {
                OptionalInitilization::Uninitialized(_) => {}
                OptionalInitilization::Initialized(stmt) => visitor(stmt),
            }
        }
        visitor(&self.body);
    }
    fn visit_child_stmts_mut<S: FnMut(&mut Stmt)>(&mut self, mut visitor: S) {
        for param in self.parameters.iter_mut() {
            match param {
                OptionalInitilization::Uninitialized(_) => {}
                OptionalInitilization::Initialized(stmt) => visitor(stmt),
            }
        }
        visitor(&mut self.body);
    }
}

/// Representation of a constructor's behavior in a function declaration.
#[derive(Debug, PartialEq, Clone)]
pub enum Constructor {
    /// A constructor that inherits from the nested call.
    WithInheritance(Expr),
    /// A constructor that does not inherit from another constructor.
    WithoutInheritance,
}
