use crate::{
    analyze::Scope,
    lint::LintTag,
    parse::{Span, *},
    FileId,
};

use super::{IntoStatementBox, Statement};

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

    /// Returns the expression as a Grouping or None.
    pub fn as_grouping(&self) -> Option<&Grouping> {
        match self {
            Expression::Grouping(inner) => Some(inner),
            _ => None,
        }
    }
}
impl IntoExpressionBox for Expression {}
impl ParseVisitor for Expression {
    fn visit_child_statements<S>(&self, visitor: S)
    where
        S: FnMut(&StatementBox),
    {
        match self {
            Expression::FunctionDeclaration(inner) => inner.visit_child_statements(visitor),
            Expression::Logical(inner) => inner.visit_child_statements(visitor),
            Expression::Equality(inner) => inner.visit_child_statements(visitor),
            Expression::Evaluation(inner) => inner.visit_child_statements(visitor),
            Expression::NullCoalecence(inner) => inner.visit_child_statements(visitor),
            Expression::Ternary(inner) => inner.visit_child_statements(visitor),
            Expression::Unary(inner) => inner.visit_child_statements(visitor),
            Expression::Postfix(inner) => inner.visit_child_statements(visitor),
            Expression::Access(inner) => inner.visit_child_statements(visitor),
            Expression::Call(inner) => inner.visit_child_statements(visitor),
            Expression::Grouping(inner) => inner.visit_child_statements(visitor),
            Expression::Literal(inner) => inner.visit_child_statements(visitor),
            Expression::Identifier(inner) => inner.visit_child_statements(visitor),
        }
    }

    fn visit_child_statements_mut<S>(&mut self, visitor: S)
    where
        S: FnMut(&mut StatementBox),
    {
        match self {
            Expression::FunctionDeclaration(inner) => inner.visit_child_statements_mut(visitor),
            Expression::Logical(inner) => inner.visit_child_statements_mut(visitor),
            Expression::Equality(inner) => inner.visit_child_statements_mut(visitor),
            Expression::Evaluation(inner) => inner.visit_child_statements_mut(visitor),
            Expression::NullCoalecence(inner) => inner.visit_child_statements_mut(visitor),
            Expression::Ternary(inner) => inner.visit_child_statements_mut(visitor),
            Expression::Unary(inner) => inner.visit_child_statements_mut(visitor),
            Expression::Postfix(inner) => inner.visit_child_statements_mut(visitor),
            Expression::Access(inner) => inner.visit_child_statements_mut(visitor),
            Expression::Call(inner) => inner.visit_child_statements_mut(visitor),
            Expression::Grouping(inner) => inner.visit_child_statements_mut(visitor),
            Expression::Literal(inner) => inner.visit_child_statements_mut(visitor),
            Expression::Identifier(inner) => inner.visit_child_statements_mut(visitor),
        }
    }

    fn visit_child_expressions<E>(&self, visitor: E)
    where
        E: FnMut(&ExpressionBox),
    {
        match self {
            Expression::FunctionDeclaration(inner) => inner.visit_child_expressions(visitor),
            Expression::Logical(inner) => inner.visit_child_expressions(visitor),
            Expression::Equality(inner) => inner.visit_child_expressions(visitor),
            Expression::Evaluation(inner) => inner.visit_child_expressions(visitor),
            Expression::NullCoalecence(inner) => inner.visit_child_expressions(visitor),
            Expression::Ternary(inner) => inner.visit_child_expressions(visitor),
            Expression::Unary(inner) => inner.visit_child_expressions(visitor),
            Expression::Postfix(inner) => inner.visit_child_expressions(visitor),
            Expression::Access(inner) => inner.visit_child_expressions(visitor),
            Expression::Call(inner) => inner.visit_child_expressions(visitor),
            Expression::Grouping(inner) => inner.visit_child_expressions(visitor),
            Expression::Literal(inner) => inner.visit_child_expressions(visitor),
            Expression::Identifier(inner) => inner.visit_child_expressions(visitor),
        }
    }

    fn visit_child_expressions_mut<E>(&mut self, visitor: E)
    where
        E: FnMut(&mut ExpressionBox),
    {
        match self {
            Expression::FunctionDeclaration(inner) => inner.visit_child_expressions_mut(visitor),
            Expression::Logical(inner) => inner.visit_child_expressions_mut(visitor),
            Expression::Equality(inner) => inner.visit_child_expressions_mut(visitor),
            Expression::Evaluation(inner) => inner.visit_child_expressions_mut(visitor),
            Expression::NullCoalecence(inner) => inner.visit_child_expressions_mut(visitor),
            Expression::Ternary(inner) => inner.visit_child_expressions_mut(visitor),
            Expression::Unary(inner) => inner.visit_child_expressions_mut(visitor),
            Expression::Postfix(inner) => inner.visit_child_expressions_mut(visitor),
            Expression::Access(inner) => inner.visit_child_expressions_mut(visitor),
            Expression::Call(inner) => inner.visit_child_expressions_mut(visitor),
            Expression::Grouping(inner) => inner.visit_child_expressions_mut(visitor),
            Expression::Literal(inner) => inner.visit_child_expressions_mut(visitor),
            Expression::Identifier(inner) => inner.visit_child_expressions_mut(visitor),
        }
    }
}

/// A wrapper around a Expression, containing additional information discovered while parsing.
#[derive(Debug, PartialEq, Clone)]
pub struct ExpressionBox {
    pub expression: Box<Expression>,
    pub scope: Option<Scope>,
    location: Location,
    lint_tag: Option<LintTag>,
}
impl ExpressionBox {
    /// Returns a reference to the inner expression.
    pub fn expression(&self) -> &Expression {
        self.expression.as_ref()
    }
    /// Returns a mutable reference to the inner expression.
    pub fn expression_mut(&mut self) -> &mut Expression {
        self.expression.as_mut()
    }
    /// Returns the Location this expression is from.
    pub fn location(&self) -> Location {
        self.location
    }
    /// Returns the span this expression originates from.
    pub fn span(&self) -> Span {
        self.location().1
    }
    /// Returns the file id this expression originates from.
    pub fn file_id(&self) -> FileId {
        self.location().0
    }
    /// Returns the lint tag attached to this statement, if any.
    pub fn lint_tag(&self) -> Option<&LintTag> {
        self.lint_tag.as_ref()
    }
}
impl From<ExpressionBox> for Statement {
    fn from(expr: ExpressionBox) -> Self {
        Self::Expression(expr)
    }
}
impl IntoStatementBox for ExpressionBox {}
impl ParseVisitor for ExpressionBox {
    fn visit_child_expressions<E: FnMut(&Self)>(&self, visitor: E) {
        self.expression().visit_child_expressions(visitor)
    }
    fn visit_child_expressions_mut<E: FnMut(&mut Self)>(&mut self, visitor: E) {
        self.expression_mut().visit_child_expressions_mut(visitor)
    }
    fn visit_child_statements<S: FnMut(&StatementBox)>(&self, visitor: S) {
        self.expression().visit_child_statements(visitor)
    }
    fn visit_child_statements_mut<S: FnMut(&mut StatementBox)>(&mut self, visitor: S) {
        self.expression_mut().visit_child_statements_mut(visitor)
    }
}

/// Derives two methods to convert the T into an [ExpressionBox], supporting both a standard
/// `into_expression_box` method, and a `into_lazy_box` for tests.
pub trait IntoExpressionBox: Sized + Into<Expression> {
    /// Converts self into an expression box.
    fn into_expression_box(self, span: Span, file_id: FileId, lint_tag: Option<LintTag>) -> ExpressionBox {
        ExpressionBox {
            expression: Box::new(self.into()),
            location: Location(file_id, span),
            scope: None,
            lint_tag,
        }
    }

    /// Converts self into an expression box with a default span. Used in tests, where all spans are
    /// expected to be 0, 0.
    fn into_lazy_box(self) -> ExpressionBox
    where
        Self: Sized,
    {
        self.into_expression_box(Default::default(), 0, None)
    }
}
