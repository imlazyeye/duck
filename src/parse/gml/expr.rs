use super::{IntoStmt, StmtType};
use crate::{
    analyze::Type,
    lint::LintTag,
    parse::{Span, *},
    FileId,
};
use itertools::Itertools;

/// A singular gml statement.
#[derive(Debug, PartialEq, Clone)]
pub enum ExprType {
    /// Declaration of a function.
    FunctionDeclaration(Function),
    /// A logical comparision.
    Logical(Logical),
    /// An equality assessment.
    Equality(Equality),
    /// A mathmatical evaluation.
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
impl ExprType {
    /// Returns the expression as an Identifier or None.
    pub fn as_identifier(&self) -> Option<&Identifier> {
        match self {
            ExprType::Identifier(identifier) => Some(identifier),
            _ => None,
        }
    }

    /// Returns the expression a the interior fields of a Access::Dot, or None.
    pub fn as_dot_access(&self) -> Option<(&Self, &Identifier)> {
        match self {
            ExprType::Access(Access::Dot { left, right }) => Some((left.inner(), right)),
            _ => None,
        }
    }

    /// Returns the expression as an Literal or None.
    pub fn as_literal(&self) -> Option<&Literal> {
        match self {
            ExprType::Literal(literal) => Some(literal),
            _ => None,
        }
    }

    /// Returns the expression as a Grouping or None.
    pub fn as_grouping(&self) -> Option<&Grouping> {
        match self {
            ExprType::Grouping(inner) => Some(inner),
            _ => None,
        }
    }

    /// Returns the expression as a Equality or None.
    pub fn as_equality(&self) -> Option<&Equality> {
        match self {
            ExprType::Equality(inner) => Some(inner),
            _ => None,
        }
    }
}
impl IntoExpr for ExprType {}

/// A wrapper around an [ExprType], containing additional information discovered while parsing.
#[derive(Debug, PartialEq, Clone)]
pub struct Expr {
    expr_type: Box<ExprType>,
    pub id: ExprId,
    pub tpe: Type,
    location: Location,
    lint_tag: Option<LintTag>,
}
impl Expr {
    /// Get a reference to the expression box's expr type.
    pub fn inner(&self) -> &ExprType {
        self.expr_type.as_ref()
    }

    /// Get a mutable reference to the expression box's expr type.
    pub fn inner_mut(&mut self) -> &mut ExprType {
        &mut self.expr_type
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
impl From<Expr> for StmtType {
    fn from(expr: Expr) -> Self {
        Self::Expr(expr)
    }
}
impl IntoStmt for Expr {}
impl ParseVisitor for Expr {
    fn visit_child_stmts<S>(&self, visitor: S)
    where
        S: FnMut(&Stmt),
    {
        match self.inner() {
            ExprType::FunctionDeclaration(inner) => inner.visit_child_stmts(visitor),
            ExprType::Logical(inner) => inner.visit_child_stmts(visitor),
            ExprType::Equality(inner) => inner.visit_child_stmts(visitor),
            ExprType::Evaluation(inner) => inner.visit_child_stmts(visitor),
            ExprType::NullCoalecence(inner) => inner.visit_child_stmts(visitor),
            ExprType::Ternary(inner) => inner.visit_child_stmts(visitor),
            ExprType::Unary(inner) => inner.visit_child_stmts(visitor),
            ExprType::Postfix(inner) => inner.visit_child_stmts(visitor),
            ExprType::Access(inner) => inner.visit_child_stmts(visitor),
            ExprType::Call(inner) => inner.visit_child_stmts(visitor),
            ExprType::Grouping(inner) => inner.visit_child_stmts(visitor),
            ExprType::Literal(inner) => inner.visit_child_stmts(visitor),
            ExprType::Identifier(inner) => inner.visit_child_stmts(visitor),
        }
    }

    fn visit_child_stmts_mut<S>(&mut self, visitor: S)
    where
        S: FnMut(&mut Stmt),
    {
        match self.inner_mut() {
            ExprType::FunctionDeclaration(inner) => inner.visit_child_stmts_mut(visitor),
            ExprType::Logical(inner) => inner.visit_child_stmts_mut(visitor),
            ExprType::Equality(inner) => inner.visit_child_stmts_mut(visitor),
            ExprType::Evaluation(inner) => inner.visit_child_stmts_mut(visitor),
            ExprType::NullCoalecence(inner) => inner.visit_child_stmts_mut(visitor),
            ExprType::Ternary(inner) => inner.visit_child_stmts_mut(visitor),
            ExprType::Unary(inner) => inner.visit_child_stmts_mut(visitor),
            ExprType::Postfix(inner) => inner.visit_child_stmts_mut(visitor),
            ExprType::Access(inner) => inner.visit_child_stmts_mut(visitor),
            ExprType::Call(inner) => inner.visit_child_stmts_mut(visitor),
            ExprType::Grouping(inner) => inner.visit_child_stmts_mut(visitor),
            ExprType::Literal(inner) => inner.visit_child_stmts_mut(visitor),
            ExprType::Identifier(inner) => inner.visit_child_stmts_mut(visitor),
        }
    }

    fn visit_child_exprs<E>(&self, visitor: E)
    where
        E: FnMut(&Expr),
    {
        match self.inner() {
            ExprType::FunctionDeclaration(inner) => inner.visit_child_exprs(visitor),
            ExprType::Logical(inner) => inner.visit_child_exprs(visitor),
            ExprType::Equality(inner) => inner.visit_child_exprs(visitor),
            ExprType::Evaluation(inner) => inner.visit_child_exprs(visitor),
            ExprType::NullCoalecence(inner) => inner.visit_child_exprs(visitor),
            ExprType::Ternary(inner) => inner.visit_child_exprs(visitor),
            ExprType::Unary(inner) => inner.visit_child_exprs(visitor),
            ExprType::Postfix(inner) => inner.visit_child_exprs(visitor),
            ExprType::Access(inner) => inner.visit_child_exprs(visitor),
            ExprType::Call(inner) => inner.visit_child_exprs(visitor),
            ExprType::Grouping(inner) => inner.visit_child_exprs(visitor),
            ExprType::Literal(inner) => inner.visit_child_exprs(visitor),
            ExprType::Identifier(inner) => inner.visit_child_exprs(visitor),
        }
    }

    fn visit_child_exprs_mut<E>(&mut self, visitor: E)
    where
        E: FnMut(&mut Expr),
    {
        match self.inner_mut() {
            ExprType::FunctionDeclaration(inner) => inner.visit_child_exprs_mut(visitor),
            ExprType::Logical(inner) => inner.visit_child_exprs_mut(visitor),
            ExprType::Equality(inner) => inner.visit_child_exprs_mut(visitor),
            ExprType::Evaluation(inner) => inner.visit_child_exprs_mut(visitor),
            ExprType::NullCoalecence(inner) => inner.visit_child_exprs_mut(visitor),
            ExprType::Ternary(inner) => inner.visit_child_exprs_mut(visitor),
            ExprType::Unary(inner) => inner.visit_child_exprs_mut(visitor),
            ExprType::Postfix(inner) => inner.visit_child_exprs_mut(visitor),
            ExprType::Access(inner) => inner.visit_child_exprs_mut(visitor),
            ExprType::Call(inner) => inner.visit_child_exprs_mut(visitor),
            ExprType::Grouping(inner) => inner.visit_child_exprs_mut(visitor),
            ExprType::Literal(inner) => inner.visit_child_exprs_mut(visitor),
            ExprType::Identifier(inner) => inner.visit_child_exprs_mut(visitor),
        }
    }
}
impl std::fmt::Display for Expr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.inner() {
            ExprType::FunctionDeclaration(Function {
                name,
                parameters,
                constructor,
                ..
            }) => {
                let constructor_str = match constructor {
                    Some(Constructor::WithInheritance(call)) => format!(": {call} constructor"),
                    Some(Constructor::WithoutInheritance) => "constructor".into(),
                    None => "".into(),
                };
                let param_str = parameters.iter().join(", ");
                if let Some(Identifier { lexeme, .. }) = name {
                    f.pad(&format!("function {lexeme}({param_str}) {constructor_str} {{ ... }}"))
                } else {
                    f.pad(&format!("function({param_str}) {constructor_str} {{ ... }}"))
                }
            }
            ExprType::Logical(Logical { left, op, right }) => f.pad(&format!("{left} {op} {right}")),
            ExprType::Equality(Equality { left, op, right }) => f.pad(&format!("{left} {op} {right}")),
            ExprType::Evaluation(Evaluation { left, op, right }) => f.pad(&format!("{left} {op} {right}")),
            ExprType::NullCoalecence(NullCoalecence { left, right }) => f.pad(&format!("{left} ?? {right}")),
            ExprType::Ternary(Ternary {
                condition,
                true_value,
                false_value,
            }) => f.pad(&format!("{condition} ? {true_value} : {false_value}")),
            ExprType::Unary(Unary { op, right }) => f.pad(&format!("{op}{right}")),
            ExprType::Postfix(Postfix { left, op }) => f.pad(&format!("{left}{op}")),
            ExprType::Access(access) => match access {
                Access::Global { right } => f.pad(&format!("global.{right}")),
                Access::Current { right } => f.pad(&format!("self.{right}")),
                Access::Other { right } => f.pad(&format!("other.{right}")),
                Access::Dot { left, right } => f.pad(&format!("{left}.{right}")),
                Access::Array {
                    left,
                    index_one,
                    index_two,
                    using_accessor,
                } => {
                    let accessor = if *using_accessor { "@ " } else { "" };
                    if let Some(index_two) = index_two {
                        f.pad(&format!("{left}[{accessor}{index_one}, {index_two}]"))
                    } else {
                        f.pad(&format!("{left}[{accessor}{index_one}]"))
                    }
                }
                Access::Map { left, key } => todo!(),
                Access::Grid {
                    left,
                    index_one,
                    index_two,
                } => todo!(),
                Access::List { left, index } => todo!(),
                Access::Struct { left, key } => todo!(),
            },
            ExprType::Call(Call {
                left,
                arguments,
                uses_new,
            }) => f.pad(&format!(
                "{}{}({})",
                if *uses_new { "new " } else { "" },
                left,
                arguments.iter().join(", "),
            )),
            ExprType::Grouping(Grouping { inner, .. }) => f.pad(&format!("({inner})",)),
            ExprType::Literal(literal) => match literal {
                Literal::True => f.pad("true"),
                Literal::False => f.pad("false"),
                Literal::Undefined => f.pad("undefined"),
                Literal::Noone => f.pad("noone"),
                Literal::String(s) => f.pad(&format!("\"{}\"", s)),
                Literal::Real(r) => f.pad(&r.to_string()),
                Literal::Hex(h) => f.pad(&format!("hex<{}>", h)),
                Literal::Array(members) => f.pad(&format!(
                    "[{}]",
                    members.iter().map(|member| member.to_string()).join(", ")
                )),
                Literal::Struct(fields) => f.pad(&format!(
                    "{{ {} }}",
                    fields
                        .iter()
                        .map(|(Identifier { lexeme, .. }, symbol)| format!("{lexeme}: {symbol}"))
                        .join(", ")
                )),
                Literal::Misc(lexeme) => f.pad(lexeme),
            },
            ExprType::Identifier(iden) => f.pad(&iden.lexeme),
        }
    }
}

/// Derives two methods to convert the T into an [Expr], supporting both a standard
/// `into_expr` method, and a `into_expr_lazy` for tests.
pub trait IntoExpr: Sized + Into<ExprType> {
    /// Converts self into an Expr.
    fn into_expr(self, tpe: Type, id: ExprId, span: Span, file_id: FileId, lint_tag: Option<LintTag>) -> Expr {
        Expr {
            expr_type: Box::new(self.into()),
            id,
            tpe,
            location: Location(file_id, span),
            lint_tag,
        }
    }

    /// Converts self into an expression, using default values for everything else. Used in tests.
    fn into_expr_lazy(self) -> Expr
    where
        Self: Sized,
    {
        self.into_expr(Type::Unknown, ExprId::default(), Default::default(), 0, None)
    }
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy, Default)]
pub struct ExprId(u64);
impl ExprId {
    pub fn new() -> Self {
        Self(rand::random())
    }
}
