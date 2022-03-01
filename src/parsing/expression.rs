use crate::{
    gml::{Assignment, Constructor, Equality, Evaluation, Function, Identifier, Logical},
    parsing::statement::StatementBox,
    utils::Span,
};

use super::{IntoStatementBox, ParseVisitor, Statement};

/// A singular gml statement.
#[derive(Debug, PartialEq, Clone)]
pub enum Expression {
    /// Declaration of a function. Since gml supports anonymous functions, these are expressions,
    /// not statements!
    FunctionDeclaration(Function),
    Logical(Logical),
    Equality(Equality),
    Evaluation(Evaluation),
    NullCoalecence(ExpressionBox, ExpressionBox),
    Ternary(ExpressionBox, ExpressionBox, ExpressionBox),
    Assignment(Assignment),
    Unary(UnaryOperator, ExpressionBox),
    Postfix(ExpressionBox, PostfixOperator),
    Access(Scope, ExpressionBox),
    Call(ExpressionBox, Vec<ExpressionBox>, bool),
    Grouping(ExpressionBox),
    Literal(Literal),
    Identifier(Identifier),
}
impl Expression {
    pub fn into_box(self, span: Span) -> ExpressionBox {
        ExpressionBox(Box::new(self), span)
    }

    pub fn lazy_box(self) -> ExpressionBox {
        ExpressionBox(Box::new(self), Span::default())
    }

    pub fn visit_child_statements<S>(&self, mut statement_visitor: S)
    where
        S: FnMut(&StatementBox),
    {
        if let Expression::FunctionDeclaration(Function { body, .. }) = self {
            statement_visitor(body);
        }
    }

    pub fn visit_child_expressions<E>(&self, mut expression_visitor: E)
    where
        E: FnMut(&ExpressionBox),
    {
        match self {
            Expression::FunctionDeclaration(Function {
                parameters,
                constructor,
                ..
            }) => {
                for parameter in parameters.iter() {
                    if let Some(default_value) = &parameter.default_value {
                        expression_visitor(default_value);
                    }
                }
                if let Some(Constructor::WithInheritance(inheritance_call)) = &constructor {
                    expression_visitor(inheritance_call);
                }
            }
            Expression::Logical(Logical { left, right, .. })
            | Expression::Equality(Equality { left, right, .. })
            | Expression::Evaluation(Evaluation { left, right, .. })
            | Expression::Assignment(Assignment { left, right, .. })
            | Expression::NullCoalecence(left, right) => {
                expression_visitor(left);
                expression_visitor(right);
            }
            Expression::Ternary(condition, left, right) => {
                expression_visitor(condition);
                expression_visitor(left);
                expression_visitor(right);
            }
            Expression::Unary(_, right) => {
                expression_visitor(right);
            }
            Expression::Postfix(left, _) => {
                expression_visitor(left);
            }
            Expression::Access(scope, expression) => {
                expression_visitor(expression);
                match scope {
                    Scope::Dot(other) => {
                        expression_visitor(other);
                    }
                    Scope::Array(x, y, _) => {
                        expression_visitor(x);
                        if let Some(y) = y {
                            expression_visitor(y);
                        }
                    }
                    Scope::Map(key) => {
                        expression_visitor(key);
                    }
                    Scope::Grid(x, y) => {
                        expression_visitor(x);
                        expression_visitor(y);
                    }
                    Scope::List(index) => {
                        expression_visitor(index);
                    }
                    Scope::Struct(key) => {
                        expression_visitor(key);
                    }
                    Scope::Global | Scope::Current => {}
                }
            }
            Expression::Call(left, arguments, _) => {
                expression_visitor(left);
                for arg in arguments {
                    expression_visitor(arg);
                }
            }
            Expression::Grouping(expression) => {
                expression_visitor(expression);
            }
            Expression::Literal(_) | Expression::Identifier(_) => {}
        }
    }

    pub fn as_identifier(&self) -> Option<&Identifier> {
        match self {
            Expression::Identifier(identifier) => Some(identifier),
            _ => None,
        }
    }

    pub fn as_assignment(&self) -> Option<&Assignment> {
        match self {
            Expression::Assignment(assignment) => Some(assignment),
            _ => None,
        }
    }

    pub fn as_dot_access(&self) -> Option<(&Expression, &Expression)> {
        match self {
            Expression::Access(Scope::Dot(left), right) => Some((left.expression(), right.expression())),
            _ => None,
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct ExpressionBox(pub Box<Expression>, pub Span);
impl ExpressionBox {
    pub fn expression(&self) -> &Expression {
        self.0.as_ref()
    }
    pub fn span(&self) -> Span {
        self.1
    }
}
impl From<ExpressionBox> for Statement {
    fn from(expr: ExpressionBox) -> Self {
        Statement::Expression(expr)
    }
}
impl IntoStatementBox for ExpressionBox {}
impl ParseVisitor for ExpressionBox {
    fn visit_child_expressions<E: FnMut(&ExpressionBox)>(&self, expression_visitor: E) {
        self.expression().visit_child_expressions(expression_visitor)
    }

    fn visit_child_statements<S: FnMut(&StatementBox)>(&self, statement_visitor: S) {
        self.expression().visit_child_statements(statement_visitor)
    }
}

/// Derives two methods to convert the T into an [ExpressionBox], supporting both a standard
/// `into_expression_box` method, and a `into_lazy_box` for tests.
///
/// TODO: This could be a derive macro!
pub trait IntoExpressionBox: Sized + Into<Expression> {
    /// Converts self into an expression box with a provided span.
    fn into_expression_box(self, span: Span) -> ExpressionBox {
        ExpressionBox(Box::new(self.into()), span)
    }

    /// Converts self into an expression box with a default span. Used in tests, where all spans are
    /// expected to be 0, 0.
    fn into_lazy_box(self) -> ExpressionBox
    where
        Self: Sized,
    {
        self.into_expression_box(Default::default())
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum UnaryOperator {
    Increment,
    Decrement,
    Not,
    Positive,
    Negative,
    BitwiseNot,
}

#[derive(Debug, PartialEq, Clone)]
pub enum PostfixOperator {
    Increment,
    Decrement,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Literal {
    True,
    False,
    Undefined,
    Noone,
    String(String),
    Real(f64),
    Hex(String),
    Array(Vec<ExpressionBox>),
    Struct(Vec<(String, ExpressionBox)>),
    /// Any GML constant that we are aware of but do not have specific use for.
    Misc(String),
}

#[derive(Debug, PartialEq, Clone)]
pub enum Scope {
    Global,
    Current,
    Dot(ExpressionBox),
    Array(ExpressionBox, Option<ExpressionBox>, bool),
    Map(ExpressionBox),
    Grid(ExpressionBox, ExpressionBox),
    List(ExpressionBox),
    Struct(ExpressionBox),
}
