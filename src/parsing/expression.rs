use crate::{parsing::*, utils::Span};

use super::{IntoStatementBox, ParseVisitor, Statement};

/// A singular gml statement.
#[derive(Debug, PartialEq, Clone)]
pub enum Expression {
    /// Declaration of a function. Since gml supports anonymous functions, these are expressions,
    /// not statements!
    FunctionDeclaration(Function),
    /// A logical comparision.
    Logical(Logical),
    /// An equality assessment.
    Equality(Equality),
    /// An evaluation.
    Evaluation(Evaluation),
    /// A null coalecence operation.
    NullCoalecence(NullCoalecence),
    /// A ternary operation.
    Ternary(Ternary),
    /// An assignment.
    Assignment(Assignment),
    /// A unary operation.
    Unary(Unary),
    /// A postfix operation.
    Postfix(Postfix),
    /// An access into another scope, such as an array lookup, or dot-notation on a struct.
    Access(Access),
    /// An invokation of a function.
    Call(Call),
    /// A grouping (expression surrounded by parenthesis.)
    Grouping(Grouping),
    /// A constant compile-time value in gml.
    Literal(Literal),
    /// An identifier (any floating lexeme, most often variables).
    Identifier(Identifier),
}
impl Expression {
    /// Returns the expression as an Identifier or None.
    pub fn as_identifier(&self) -> Option<&Identifier> {
        match self {
            Expression::Identifier(identifier) => Some(identifier),
            _ => None,
        }
    }

    /// Returns the expression as an Assignment or None.
    pub fn as_assignment(&self) -> Option<&Assignment> {
        match self {
            Expression::Assignment(assignment) => Some(assignment),
            _ => None,
        }
    }

    /// Returns the expression a the interior fields of a Access::Dot, or None.
    pub fn as_dot_access(&self) -> Option<(&Self, &Self)> {
        match self {
            Expression::Access(Access::Dot { left, right }) => Some((left.expression(), right.expression())),
            _ => None,
        }
    }

    /// Returns the expression as an Literal or None.
    pub fn as_literal(&self) -> Option<&Literal> {
        match self {
            Expression::Literal(literal) => Some(literal),
            _ => None,
        }
    }
}
impl ParseVisitor for Expression {
    fn visit_child_statements<S>(&self, statement_visitor: S)
    where
        S: FnMut(&StatementBox),
    {
        match self {
            Expression::FunctionDeclaration(inner) => inner.visit_child_statements(statement_visitor),
            Expression::Logical(inner) => inner.visit_child_statements(statement_visitor),
            Expression::Equality(inner) => inner.visit_child_statements(statement_visitor),
            Expression::Evaluation(inner) => inner.visit_child_statements(statement_visitor),
            Expression::NullCoalecence(inner) => inner.visit_child_statements(statement_visitor),
            Expression::Ternary(inner) => inner.visit_child_statements(statement_visitor),
            Expression::Assignment(inner) => inner.visit_child_statements(statement_visitor),
            Expression::Unary(inner) => inner.visit_child_statements(statement_visitor),
            Expression::Postfix(inner) => inner.visit_child_statements(statement_visitor),
            Expression::Access(inner) => inner.visit_child_statements(statement_visitor),
            Expression::Call(inner) => inner.visit_child_statements(statement_visitor),
            Expression::Grouping(inner) => inner.visit_child_statements(statement_visitor),
            Expression::Literal(inner) => inner.visit_child_statements(statement_visitor),
            Expression::Identifier(inner) => inner.visit_child_statements(statement_visitor),
        }
    }

    fn visit_child_expressions<E>(&self, expression_visitor: E)
    where
        E: FnMut(&ExpressionBox),
    {
        match self {
            Expression::FunctionDeclaration(inner) => inner.visit_child_expressions(expression_visitor),
            Expression::Logical(inner) => inner.visit_child_expressions(expression_visitor),
            Expression::Equality(inner) => inner.visit_child_expressions(expression_visitor),
            Expression::Evaluation(inner) => inner.visit_child_expressions(expression_visitor),
            Expression::NullCoalecence(inner) => inner.visit_child_expressions(expression_visitor),
            Expression::Ternary(inner) => inner.visit_child_expressions(expression_visitor),
            Expression::Assignment(inner) => inner.visit_child_expressions(expression_visitor),
            Expression::Unary(inner) => inner.visit_child_expressions(expression_visitor),
            Expression::Postfix(inner) => inner.visit_child_expressions(expression_visitor),
            Expression::Access(inner) => inner.visit_child_expressions(expression_visitor),
            Expression::Call(inner) => inner.visit_child_expressions(expression_visitor),
            Expression::Grouping(inner) => inner.visit_child_expressions(expression_visitor),
            Expression::Literal(inner) => inner.visit_child_expressions(expression_visitor),
            Expression::Identifier(inner) => inner.visit_child_expressions(expression_visitor),
        }
    }
}

/// A wrapper around a Expression. Serves a few purposes:
///
/// 1. Prevents infinite-sizing issues on [Expression] (type T cannot itself directly hold another
/// T) 2. Contains the [Span] that describes where this expression came from
/// 3. In the future, will hold static-analysis data
#[derive(Debug, PartialEq, Clone)]
pub struct ExpressionBox(pub Box<Expression>, pub Span);
impl ExpressionBox {
    /// Returns a reference to the inner expression.
    pub fn expression(&self) -> &Expression {
        self.0.as_ref()
    }
    /// Returns a reference to the inner span.
    pub fn span(&self) -> Span {
        self.1
    }
}
impl From<ExpressionBox> for Statement {
    fn from(expr: ExpressionBox) -> Self {
        Self::Expression(expr)
    }
}
impl IntoStatementBox for ExpressionBox {}
impl ParseVisitor for ExpressionBox {
    fn visit_child_expressions<E: FnMut(&Self)>(&self, expression_visitor: E) {
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
