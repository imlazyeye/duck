use super::{IntoStmt, StmtKind};
use crate::{
    lint::LintTag,
    parse::{Span, *},
    FileId,
};
use itertools::Itertools;

/// A singular gml statement.
#[derive(Debug, PartialEq, Clone)]
pub enum ExprKind {
    /// Declaration of an enum.
    Enum(Enum),
    /// Declaration of a macro.
    Macro(Macro),
    /// Declaration of a function.
    Function(Function),
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
impl ExprKind {
    /// Returns the expression as an Identifier or None.
    pub fn as_identifier(&self) -> Option<&Identifier> {
        match self {
            ExprKind::Identifier(identifier) => Some(identifier),
            _ => None,
        }
    }

    /// Returns the expression a the interior fields of a Access::Dot, or None.
    pub fn as_dot_access(&self) -> Option<(&Self, &Identifier)> {
        match self {
            ExprKind::Access(Access::Dot { left, right }) => Some((left.inner(), right)),
            _ => None,
        }
    }

    /// Returns the expression a the interior fields of a Access::Identity, or None.
    pub fn as_identity_access(&self) -> Option<&Identifier> {
        match self {
            ExprKind::Access(Access::Identity { right }) => Some(right),
            _ => None,
        }
    }

    /// Returns the expression as an Literal or None.
    pub fn as_literal(&self) -> Option<&Literal> {
        match self {
            ExprKind::Literal(literal) => Some(literal),
            _ => None,
        }
    }

    /// Returns the expression as a Grouping or None.
    pub fn as_grouping(&self) -> Option<&Grouping> {
        match self {
            ExprKind::Grouping(inner) => Some(inner),
            _ => None,
        }
    }

    /// Returns the expression as a Equality or None.
    pub fn as_equality(&self) -> Option<&Equality> {
        match self {
            ExprKind::Equality(inner) => Some(inner),
            _ => None,
        }
    }

    /// Returns the expression as a function or None.
    pub fn as_function(&self) -> Option<&Function> {
        match self {
            ExprKind::Function(inner) => Some(inner),
            _ => None,
        }
    }

    /// Returns the expression as a Call or None.
    pub fn as_call(&self) -> Option<&Call> {
        match self {
            ExprKind::Call(inner) => Some(inner),
            _ => None,
        }
    }
}
impl IntoExpr for ExprKind {}

/// A wrapper around an [ExprType], containing additional information discovered while parsing.
#[derive(Debug, PartialEq, Clone)]
pub struct Expr {
    expr_type: Box<ExprKind>,
    id: ExprId,
    location: Location,
    lint_tag: Option<LintTag>,
}
impl Expr {
    /// Get a reference to the expression box's expr type.
    pub fn inner(&self) -> &ExprKind {
        self.expr_type.as_ref()
    }

    /// Get a mutable reference to the expression box's expr type.
    pub fn inner_mut(&mut self) -> &mut ExprKind {
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

    /// Get the expr's id.
    pub fn id(&self) -> ExprId {
        self.id
    }
}
impl From<Expr> for StmtKind {
    fn from(expr: Expr) -> Self {
        Self::Expr(expr)
    }
}
impl IntoStmt for Expr {}
impl ParseVisitor for Expr {
    fn visit_child_exprs<E>(&self, visitor: E)
    where
        E: FnMut(&Expr),
    {
        match self.inner() {
            ExprKind::Enum(inner) => inner.visit_child_exprs(visitor),
            ExprKind::Macro(inner) => inner.visit_child_exprs(visitor),
            ExprKind::Function(inner) => inner.visit_child_exprs(visitor),
            ExprKind::Logical(inner) => inner.visit_child_exprs(visitor),
            ExprKind::Equality(inner) => inner.visit_child_exprs(visitor),
            ExprKind::Evaluation(inner) => inner.visit_child_exprs(visitor),
            ExprKind::NullCoalecence(inner) => inner.visit_child_exprs(visitor),
            ExprKind::Ternary(inner) => inner.visit_child_exprs(visitor),
            ExprKind::Unary(inner) => inner.visit_child_exprs(visitor),
            ExprKind::Postfix(inner) => inner.visit_child_exprs(visitor),
            ExprKind::Access(inner) => inner.visit_child_exprs(visitor),
            ExprKind::Call(inner) => inner.visit_child_exprs(visitor),
            ExprKind::Grouping(inner) => inner.visit_child_exprs(visitor),
            ExprKind::Literal(inner) => inner.visit_child_exprs(visitor),
            ExprKind::Identifier(inner) => inner.visit_child_exprs(visitor),
        }
    }

    fn visit_child_exprs_mut<E>(&mut self, visitor: E)
    where
        E: FnMut(&mut Expr),
    {
        match self.inner_mut() {
            ExprKind::Enum(inner) => inner.visit_child_exprs_mut(visitor),
            ExprKind::Macro(inner) => inner.visit_child_exprs_mut(visitor),
            ExprKind::Function(inner) => inner.visit_child_exprs_mut(visitor),
            ExprKind::Logical(inner) => inner.visit_child_exprs_mut(visitor),
            ExprKind::Equality(inner) => inner.visit_child_exprs_mut(visitor),
            ExprKind::Evaluation(inner) => inner.visit_child_exprs_mut(visitor),
            ExprKind::NullCoalecence(inner) => inner.visit_child_exprs_mut(visitor),
            ExprKind::Ternary(inner) => inner.visit_child_exprs_mut(visitor),
            ExprKind::Unary(inner) => inner.visit_child_exprs_mut(visitor),
            ExprKind::Postfix(inner) => inner.visit_child_exprs_mut(visitor),
            ExprKind::Access(inner) => inner.visit_child_exprs_mut(visitor),
            ExprKind::Call(inner) => inner.visit_child_exprs_mut(visitor),
            ExprKind::Grouping(inner) => inner.visit_child_exprs_mut(visitor),
            ExprKind::Literal(inner) => inner.visit_child_exprs_mut(visitor),
            ExprKind::Identifier(inner) => inner.visit_child_exprs_mut(visitor),
        }
    }

    fn visit_child_stmts<S>(&self, visitor: S)
    where
        S: FnMut(&Stmt),
    {
        match self.inner() {
            ExprKind::Enum(inner) => inner.visit_child_stmts(visitor),
            ExprKind::Macro(inner) => inner.visit_child_stmts(visitor),
            ExprKind::Function(inner) => inner.visit_child_stmts(visitor),
            ExprKind::Logical(inner) => inner.visit_child_stmts(visitor),
            ExprKind::Equality(inner) => inner.visit_child_stmts(visitor),
            ExprKind::Evaluation(inner) => inner.visit_child_stmts(visitor),
            ExprKind::NullCoalecence(inner) => inner.visit_child_stmts(visitor),
            ExprKind::Ternary(inner) => inner.visit_child_stmts(visitor),
            ExprKind::Unary(inner) => inner.visit_child_stmts(visitor),
            ExprKind::Postfix(inner) => inner.visit_child_stmts(visitor),
            ExprKind::Access(inner) => inner.visit_child_stmts(visitor),
            ExprKind::Call(inner) => inner.visit_child_stmts(visitor),
            ExprKind::Grouping(inner) => inner.visit_child_stmts(visitor),
            ExprKind::Literal(inner) => inner.visit_child_stmts(visitor),
            ExprKind::Identifier(inner) => inner.visit_child_stmts(visitor),
        }
    }

    fn visit_child_stmts_mut<S>(&mut self, visitor: S)
    where
        S: FnMut(&mut Stmt),
    {
        match self.inner_mut() {
            ExprKind::Enum(inner) => inner.visit_child_stmts_mut(visitor),
            ExprKind::Macro(inner) => inner.visit_child_stmts_mut(visitor),
            ExprKind::Function(inner) => inner.visit_child_stmts_mut(visitor),
            ExprKind::Logical(inner) => inner.visit_child_stmts_mut(visitor),
            ExprKind::Equality(inner) => inner.visit_child_stmts_mut(visitor),
            ExprKind::Evaluation(inner) => inner.visit_child_stmts_mut(visitor),
            ExprKind::NullCoalecence(inner) => inner.visit_child_stmts_mut(visitor),
            ExprKind::Ternary(inner) => inner.visit_child_stmts_mut(visitor),
            ExprKind::Unary(inner) => inner.visit_child_stmts_mut(visitor),
            ExprKind::Postfix(inner) => inner.visit_child_stmts_mut(visitor),
            ExprKind::Access(inner) => inner.visit_child_stmts_mut(visitor),
            ExprKind::Call(inner) => inner.visit_child_stmts_mut(visitor),
            ExprKind::Grouping(inner) => inner.visit_child_stmts_mut(visitor),
            ExprKind::Literal(inner) => inner.visit_child_stmts_mut(visitor),
            ExprKind::Identifier(inner) => inner.visit_child_stmts_mut(visitor),
        }
    }
}
impl std::fmt::Display for Expr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.inner() {
            ExprKind::Enum(Enum { name, members }) => {
                f.pad(&format!("enum {name} {{ {} }}", members.iter().join(", ")))
            }
            ExprKind::Macro(Macro { name, config, body }) => {
                if let Some(config) = config {
                    f.pad(&format!("#macro {config}:{name} {body}"))
                } else {
                    f.pad(&format!("#macro {name} {body}"))
                }
            }
            ExprKind::Function(Function {
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
            ExprKind::Logical(Logical { left, op, right }) => f.pad(&format!("{left} {op} {right}")),
            ExprKind::Equality(Equality { left, op, right }) => f.pad(&format!("{left} {op} {right}")),
            ExprKind::Evaluation(Evaluation { left, op, right }) => f.pad(&format!("{left} {op} {right}")),
            ExprKind::NullCoalecence(NullCoalecence { left, right }) => f.pad(&format!("{left} ?? {right}")),
            ExprKind::Ternary(Ternary {
                condition,
                true_value,
                false_value,
            }) => f.pad(&format!("{condition} ? {true_value} : {false_value}")),
            ExprKind::Unary(Unary { op, right }) => f.pad(&format!("{op}{right}")),
            ExprKind::Postfix(Postfix { left, op }) => f.pad(&format!("{left}{op}")),
            ExprKind::Access(access) => match access {
                Access::Global { right } => f.pad(&format!("global.{right}")),
                Access::Identity { right } => f.pad(&format!("self.{right}")),
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
                Access::Map { left, key } => f.pad(&format!("{left}[? {key}]")),
                Access::Grid {
                    left,
                    index_one,
                    index_two,
                } => f.pad(&format!("{left}[# {index_one}, {index_two}]")),
                Access::List { left, index } => f.pad(&format!("{left}[| {index}]")),
                Access::Struct { left, key } => f.pad(&format!("{left}[$ {key}]")),
            },
            ExprKind::Call(Call {
                left,
                arguments,
                uses_new,
            }) => f.pad(&format!(
                "{}{}({})",
                if *uses_new { "new " } else { "" },
                left,
                arguments.iter().join(", "),
            )),
            ExprKind::Grouping(Grouping { inner, .. }) => f.pad(&format!("({inner})",)),
            ExprKind::Literal(literal) => match literal {
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
            ExprKind::Identifier(iden) => f.pad(&iden.lexeme),
        }
    }
}

/// Derives two methods to convert the T into an [Expr], supporting both a standard
/// `into_expr` method, and a `into_expr_lazy` for tests.
pub trait IntoExpr: Sized + Into<ExprKind> {
    /// Converts self into an Expr.
    fn into_expr(self, id: ExprId, span: Span, file_id: FileId, lint_tag: Option<LintTag>) -> Expr {
        Expr {
            expr_type: Box::new(self.into()),
            id,
            location: Location(file_id, span),
            lint_tag,
        }
    }

    /// Converts self into an expression, using default values for everything else. Used in tests.
    fn into_expr_lazy(self) -> Expr
    where
        Self: Sized,
    {
        self.into_expr(ExprId::default(), Default::default(), 0, None)
    }
}

/// A unique id that each [Expr] has.
#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy, Default)]
pub struct ExprId(u64);
impl ExprId {
    /// Creates a new, random ExprId.
    pub fn new() -> Self {
        Self(rand::random())
    }
}
