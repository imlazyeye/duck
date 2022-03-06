use crate::parse::{Expression, ExpressionBox, IntoExpressionBox, OptionalInitilization, ParseVisitor, StatementBox};

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
    pub body: StatementBox,
}
impl Function {
    /// Creates a new function declaration.
    pub fn new(name: Identifier, parameters: Vec<OptionalInitilization>, body: StatementBox) -> Self {
        Self {
            name: Some(name),
            parameters,
            constructor: None,
            body,
        }
    }

    /// Creates a new anonymous function declaration.
    pub fn new_anonymous(parameters: Vec<OptionalInitilization>, body: StatementBox) -> Self {
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
        body: StatementBox,
    ) -> Self {
        Self {
            name,
            parameters,
            constructor: Some(constructor),
            body,
        }
    }
}
impl From<Function> for Expression {
    fn from(function: Function) -> Self {
        Self::FunctionDeclaration(function)
    }
}
impl IntoExpressionBox for Function {}
impl ParseVisitor for Function {
    fn visit_child_expressions<E: FnMut(&ExpressionBox)>(&self, mut expression_visitor: E) {
        for param in self.parameters.iter() {
            match param {
                OptionalInitilization::Uninitialized(expr) => expression_visitor(expr),
                OptionalInitilization::Initialized(_) => {}
            }
        }
        if let Some(Constructor::WithInheritance(inheritance_call)) = &self.constructor {
            expression_visitor(inheritance_call);
        }
    }
    fn visit_child_statements<S: FnMut(&StatementBox)>(&self, mut statement_visitor: S) {
        for param in self.parameters.iter() {
            match param {
                OptionalInitilization::Uninitialized(_) => {}
                OptionalInitilization::Initialized(stmt) => statement_visitor(stmt),
            }
        }
        statement_visitor(&self.body);
    }
}

/// Representation of a constructor's behavior in a function declaration.
#[derive(Debug, PartialEq, Clone)]
pub enum Constructor {
    /// A constructor that inherits from the nested call.
    WithInheritance(ExpressionBox),
    /// A constructor that does not inherit from another constructor.
    WithoutInheritance,
}
