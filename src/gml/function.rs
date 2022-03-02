use crate::prelude::{Expression, ExpressionBox, IntoExpressionBox, ParseVisitor, StatementBox};

/// Representation of function declaration in gml.
#[derive(Debug, PartialEq, Clone)]
pub struct Function {
    /// The name, if any, of this function. Anonymous functions do not have names.
    pub name: Option<String>,
    /// The parameters of this function.
    pub parameters: Vec<Parameter>,
    /// The constructor behavior of this function, if any.
    pub constructor: Option<Constructor>,
    /// The body of the function declaration.
    pub body: StatementBox,
}
impl Function {
    /// Creates a new function declaration.
    pub fn new(name: impl Into<String>, parameters: Vec<Parameter>, body: StatementBox) -> Self {
        Self {
            name: Some(name.into()),
            parameters,
            constructor: None,
            body,
        }
    }

    /// Creates a new anonymous function declaration.
    pub fn new_anonymous(parameters: Vec<Parameter>, body: StatementBox) -> Self {
        Self {
            name: None,
            parameters,
            constructor: None,
            body,
        }
    }

    /// Creates a new constructor declaration.
    pub fn new_constructor(
        name: impl Into<Option<String>>,
        parameters: Vec<Parameter>,
        constructor: Constructor,
        body: StatementBox,
    ) -> Self {
        Self {
            name: name.into(),
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
        for parameter in self.parameters.iter() {
            if let Some(default_value) = &parameter.default_value {
                expression_visitor(default_value);
            }
        }
        if let Some(Constructor::WithInheritance(inheritance_call)) = &self.constructor {
            expression_visitor(inheritance_call);
        }
    }
    fn visit_child_statements<S: FnMut(&StatementBox)>(&self, mut statement_visitor: S) {
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

/// Representation of a parameter in a function declaration.
#[derive(Debug, PartialEq, Clone)]
pub struct Parameter {
    /// The name of the parameter.
    pub name: String,
    /// The default value, if any, assigned to this parameter.
    pub default_value: Option<ExpressionBox>,
}
impl Parameter {
    /// Creates a new parameter.
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            default_value: None,
        }
    }

    /// Creates a new parameter with a default value.
    pub fn new_with_default(name: impl Into<String>, default_value: ExpressionBox) -> Self {
        Self {
            name: name.into(),
            default_value: Some(default_value),
        }
    }
}
