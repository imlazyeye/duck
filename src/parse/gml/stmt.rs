use crate::{parse::*, FileId};

use super::{Assignment, Return, Throw};

/// A singular gml statement.
#[derive(Debug, PartialEq, Clone)]
pub enum StmtKind {
    /// Declaration of an enum.
    Enum(Enum),
    /// Declaration of a macro.
    Macro(Macro),
    /// Declaration of a globalvar.
    GlobalvarDeclaration(Globalvar),
    /// Declaration of one or more local variables.
    LocalVariableSeries(LocalVariableSeries),
    /// Declaration of a try / catch call.
    TryCatch(TryCatch),
    /// A for loop.
    ForLoop(ForLoop),
    /// A with loop.
    WithLoop(WithLoop),
    /// A repeat loop.
    RepeatLoop(RepeatLoop),
    /// A do until loop.
    DoUntil(DoUntil),
    /// A while loop.
    WhileLoop(WhileLoop),
    /// An if statement.
    If(If),
    /// A switch statement.
    Switch(Switch),
    /// A block, aka a series of statements.
    Block(Block),
    /// A return statement.
    Return(Return),
    /// A throw statement.
    Throw(Throw),
    /// A delete statement.
    Delete(Delete),
    /// A break statement (from within a switch statement).
    Break,
    /// A continue statement (from within a continue statement).
    Continue,
    /// An exit statement.
    Exit,
    /// An assignment statement.
    Assignment(Assignment),
    /// A statement expression, or in other words, an expression being executed on its own.
    /// Common examples below:
    /// ```gml
    /// foo(); // call expression
    /// foo++; // postfix expression
    /// ```
    Expr(Expr),
}
impl IntoStmt for StmtKind {}
impl ParseVisitor for StmtKind {
    fn visit_child_exprs<E>(&self, mut visitor: E)
    where
        E: FnMut(&Expr),
    {
        match self {
            StmtKind::Enum(inner) => inner.visit_child_exprs(visitor),
            StmtKind::Macro(inner) => inner.visit_child_exprs(visitor),
            StmtKind::GlobalvarDeclaration(inner) => inner.visit_child_exprs(visitor),
            StmtKind::LocalVariableSeries(inner) => inner.visit_child_exprs(visitor),
            StmtKind::TryCatch(inner) => inner.visit_child_exprs(visitor),
            StmtKind::ForLoop(inner) => inner.visit_child_exprs(visitor),
            StmtKind::WithLoop(inner) => inner.visit_child_exprs(visitor),
            StmtKind::RepeatLoop(inner) => inner.visit_child_exprs(visitor),
            StmtKind::DoUntil(inner) => inner.visit_child_exprs(visitor),
            StmtKind::WhileLoop(inner) => inner.visit_child_exprs(visitor),
            StmtKind::If(inner) => inner.visit_child_exprs(visitor),
            StmtKind::Switch(inner) => inner.visit_child_exprs(visitor),
            StmtKind::Block(inner) => inner.visit_child_exprs(visitor),
            StmtKind::Return(inner) => inner.visit_child_exprs(visitor),
            StmtKind::Throw(inner) => inner.visit_child_exprs(visitor),
            StmtKind::Delete(inner) => inner.visit_child_exprs(visitor),
            StmtKind::Assignment(inner) => inner.visit_child_exprs(visitor),
            StmtKind::Expr(inner) => visitor(inner),
            StmtKind::Break | StmtKind::Continue | StmtKind::Exit => {}
        }
    }
    fn visit_child_exprs_mut<E>(&mut self, mut visitor: E)
    where
        E: FnMut(&mut Expr),
    {
        match self {
            StmtKind::Enum(inner) => inner.visit_child_exprs_mut(visitor),
            StmtKind::Macro(inner) => inner.visit_child_exprs_mut(visitor),
            StmtKind::GlobalvarDeclaration(inner) => inner.visit_child_exprs_mut(visitor),
            StmtKind::LocalVariableSeries(inner) => inner.visit_child_exprs_mut(visitor),
            StmtKind::TryCatch(inner) => inner.visit_child_exprs_mut(visitor),
            StmtKind::ForLoop(inner) => inner.visit_child_exprs_mut(visitor),
            StmtKind::WithLoop(inner) => inner.visit_child_exprs_mut(visitor),
            StmtKind::RepeatLoop(inner) => inner.visit_child_exprs_mut(visitor),
            StmtKind::DoUntil(inner) => inner.visit_child_exprs_mut(visitor),
            StmtKind::WhileLoop(inner) => inner.visit_child_exprs_mut(visitor),
            StmtKind::If(inner) => inner.visit_child_exprs_mut(visitor),
            StmtKind::Switch(inner) => inner.visit_child_exprs_mut(visitor),
            StmtKind::Block(inner) => inner.visit_child_exprs_mut(visitor),
            StmtKind::Return(inner) => inner.visit_child_exprs_mut(visitor),
            StmtKind::Throw(inner) => inner.visit_child_exprs_mut(visitor),
            StmtKind::Delete(inner) => inner.visit_child_exprs_mut(visitor),
            StmtKind::Assignment(inner) => inner.visit_child_exprs_mut(visitor),
            StmtKind::Expr(inner) => visitor(inner),
            StmtKind::Break | StmtKind::Continue | StmtKind::Exit => {}
        }
    }

    fn visit_child_stmts<S>(&self, visitor: S)
    where
        S: FnMut(&Stmt),
    {
        match self {
            StmtKind::Enum(inner) => inner.visit_child_stmts(visitor),
            StmtKind::Macro(inner) => inner.visit_child_stmts(visitor),
            StmtKind::GlobalvarDeclaration(inner) => inner.visit_child_stmts(visitor),
            StmtKind::LocalVariableSeries(inner) => inner.visit_child_stmts(visitor),
            StmtKind::TryCatch(inner) => inner.visit_child_stmts(visitor),
            StmtKind::ForLoop(inner) => inner.visit_child_stmts(visitor),
            StmtKind::WithLoop(inner) => inner.visit_child_stmts(visitor),
            StmtKind::RepeatLoop(inner) => inner.visit_child_stmts(visitor),
            StmtKind::DoUntil(inner) => inner.visit_child_stmts(visitor),
            StmtKind::WhileLoop(inner) => inner.visit_child_stmts(visitor),
            StmtKind::If(inner) => inner.visit_child_stmts(visitor),
            StmtKind::Switch(inner) => inner.visit_child_stmts(visitor),
            StmtKind::Block(inner) => inner.visit_child_stmts(visitor),
            StmtKind::Return(inner) => inner.visit_child_stmts(visitor),
            StmtKind::Throw(inner) => inner.visit_child_stmts(visitor),
            StmtKind::Delete(inner) => inner.visit_child_stmts(visitor),
            StmtKind::Assignment(inner) => inner.visit_child_stmts(visitor),
            StmtKind::Expr(_) => {}
            StmtKind::Break | StmtKind::Continue | StmtKind::Exit => {}
        }
    }

    fn visit_child_stmts_mut<S>(&mut self, visitor: S)
    where
        S: FnMut(&mut Stmt),
    {
        match self {
            StmtKind::Enum(inner) => inner.visit_child_stmts_mut(visitor),
            StmtKind::Macro(inner) => inner.visit_child_stmts_mut(visitor),
            StmtKind::GlobalvarDeclaration(inner) => inner.visit_child_stmts_mut(visitor),
            StmtKind::LocalVariableSeries(inner) => inner.visit_child_stmts_mut(visitor),
            StmtKind::TryCatch(inner) => inner.visit_child_stmts_mut(visitor),
            StmtKind::ForLoop(inner) => inner.visit_child_stmts_mut(visitor),
            StmtKind::WithLoop(inner) => inner.visit_child_stmts_mut(visitor),
            StmtKind::RepeatLoop(inner) => inner.visit_child_stmts_mut(visitor),
            StmtKind::DoUntil(inner) => inner.visit_child_stmts_mut(visitor),
            StmtKind::WhileLoop(inner) => inner.visit_child_stmts_mut(visitor),
            StmtKind::If(inner) => inner.visit_child_stmts_mut(visitor),
            StmtKind::Switch(inner) => inner.visit_child_stmts_mut(visitor),
            StmtKind::Block(inner) => inner.visit_child_stmts_mut(visitor),
            StmtKind::Return(inner) => inner.visit_child_stmts_mut(visitor),
            StmtKind::Throw(inner) => inner.visit_child_stmts_mut(visitor),
            StmtKind::Delete(inner) => inner.visit_child_stmts_mut(visitor),
            StmtKind::Assignment(inner) => inner.visit_child_stmts_mut(visitor),
            StmtKind::Expr(_) => {}
            StmtKind::Break | StmtKind::Continue | StmtKind::Exit => {}
        }
    }
}
impl StmtKind {
    /// Returns the statement as a [Block] if it is a block.
    pub fn as_block(&self) -> Option<&Block> {
        match self {
            StmtKind::Block(block) => Some(block),
            _ => None,
        }
    }
    /// Returns the statement as an [Expr] if it is an expression statement.
    pub fn as_expr_stmt(&self) -> Option<&Expr> {
        match self {
            StmtKind::Expr(expr) => Some(expr),
            _ => None,
        }
    }

    /// Returns the statement as an [Assignment] if it is an assignment statement.
    pub fn as_assignment(&self) -> Option<&Assignment> {
        match self {
            StmtKind::Assignment(assignment) => Some(assignment),
            _ => None,
        }
    }
}

/// A wrapper around a Stmt, containing additional information discovered while parsing.
#[derive(Debug, PartialEq, Clone)]
pub struct Stmt {
    stmt_type: Box<StmtKind>,
    id: StmtId,
    location: Location,
    tag: Option<Tag>,
}
impl Stmt {
    /// Returns a reference to the inner StmtKind.
    pub fn kind(&self) -> &StmtKind {
        self.stmt_type.as_ref()
    }
    /// Returns a mutable reference to the inner StmtKind.
    pub fn kind_mut(&mut self) -> &mut StmtKind {
        self.stmt_type.as_mut()
    }
    /// Returns the Location this statement is from.
    pub fn location(&self) -> Location {
        self.location
    }
    /// Returns the span this statement originates from.
    pub fn span(&self) -> Span {
        self.location().1
    }
    /// Returns the file id this statement originates from.
    pub fn file_id(&self) -> FileId {
        self.location().0
    }
    /// Returns the tag attached to this statement, if any.
    pub fn tag(&self) -> Option<&Tag> {
        self.tag.as_ref()
    }

    /// Get the stmt's id.
    pub fn id(&self) -> StmtId {
        self.id
    }
}
impl ParseVisitor for Stmt {
    fn visit_child_exprs<E: FnMut(&Expr)>(&self, visitor: E) {
        self.kind().visit_child_exprs(visitor)
    }
    fn visit_child_exprs_mut<E: FnMut(&mut Expr)>(&mut self, visitor: E) {
        self.kind_mut().visit_child_exprs_mut(visitor)
    }
    fn visit_child_stmts<S: FnMut(&Stmt)>(&self, visitor: S) {
        self.kind().visit_child_stmts(visitor)
    }
    fn visit_child_stmts_mut<S: FnMut(&mut Stmt)>(&mut self, visitor: S) {
        self.kind_mut().visit_child_stmts_mut(visitor)
    }
}

/// Derives two methods to convert the T into an [Stmt], supporting both a standard
/// `into_stmt` method, and a `into_stmt_lazy` for tests.
pub trait IntoStmt: Sized + Into<StmtKind> {
    /// Converts self into an statement box.
    fn into_stmt(self, id: StmtId, span: Span, file_id: FileId, tag: Option<Tag>) -> Stmt {
        Stmt {
            stmt_type: Box::new(self.into()),
            id,
            location: Location(file_id, span),
            tag,
        }
    }

    /// Converts self into an statement box with a default span. Used in tests, where all spans are
    /// expected to be 0, 0.
    fn into_stmt_lazy(self) -> Stmt
    where
        Self: Sized,
    {
        self.into_stmt(Default::default(), Default::default(), 0, None)
    }
}

/// A unique id that each [Stmt] has.
#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy, Default)]
pub struct StmtId(u64);
impl StmtId {
    /// Creates a new, random StmtId.
    pub fn new() -> Self {
        Self(rand::random())
    }
}
