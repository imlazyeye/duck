use crate::{
    lint::LintTag,
    parse::{
        Block, Delete, DoUntil, Expr, ForLoop, Globalvar, If, LocalVariableSeries, Location, Macro, ParseVisitor,
        RepeatLoop, Span, Switch, TryCatch, WhileLoop, WithLoop,
    },
    FileId,
};

use super::{Assignment, Return, Throw};

/// A singular gml statement.
#[derive(Debug, PartialEq, Clone)]
pub enum StmtType {
    /// Declaration of a macro.
    MacroDeclaration(Macro),
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
impl IntoStmt for StmtType {}
impl ParseVisitor for StmtType {
    fn visit_child_exprs<E>(&self, mut visitor: E)
    where
        E: FnMut(&Expr),
    {
        match self {
            StmtType::MacroDeclaration(inner) => inner.visit_child_exprs(visitor),
            StmtType::GlobalvarDeclaration(inner) => inner.visit_child_exprs(visitor),
            StmtType::LocalVariableSeries(inner) => inner.visit_child_exprs(visitor),
            StmtType::TryCatch(inner) => inner.visit_child_exprs(visitor),
            StmtType::ForLoop(inner) => inner.visit_child_exprs(visitor),
            StmtType::WithLoop(inner) => inner.visit_child_exprs(visitor),
            StmtType::RepeatLoop(inner) => inner.visit_child_exprs(visitor),
            StmtType::DoUntil(inner) => inner.visit_child_exprs(visitor),
            StmtType::WhileLoop(inner) => inner.visit_child_exprs(visitor),
            StmtType::If(inner) => inner.visit_child_exprs(visitor),
            StmtType::Switch(inner) => inner.visit_child_exprs(visitor),
            StmtType::Block(inner) => inner.visit_child_exprs(visitor),
            StmtType::Return(inner) => inner.visit_child_exprs(visitor),
            StmtType::Throw(inner) => inner.visit_child_exprs(visitor),
            StmtType::Delete(inner) => inner.visit_child_exprs(visitor),
            StmtType::Assignment(inner) => inner.visit_child_exprs(visitor),
            StmtType::Expr(inner) => visitor(inner),
            StmtType::Break | StmtType::Continue | StmtType::Exit => {}
        }
    }
    fn visit_child_exprs_mut<E>(&mut self, mut visitor: E)
    where
        E: FnMut(&mut Expr),
    {
        match self {
            StmtType::MacroDeclaration(inner) => inner.visit_child_exprs_mut(visitor),
            StmtType::GlobalvarDeclaration(inner) => inner.visit_child_exprs_mut(visitor),
            StmtType::LocalVariableSeries(inner) => inner.visit_child_exprs_mut(visitor),
            StmtType::TryCatch(inner) => inner.visit_child_exprs_mut(visitor),
            StmtType::ForLoop(inner) => inner.visit_child_exprs_mut(visitor),
            StmtType::WithLoop(inner) => inner.visit_child_exprs_mut(visitor),
            StmtType::RepeatLoop(inner) => inner.visit_child_exprs_mut(visitor),
            StmtType::DoUntil(inner) => inner.visit_child_exprs_mut(visitor),
            StmtType::WhileLoop(inner) => inner.visit_child_exprs_mut(visitor),
            StmtType::If(inner) => inner.visit_child_exprs_mut(visitor),
            StmtType::Switch(inner) => inner.visit_child_exprs_mut(visitor),
            StmtType::Block(inner) => inner.visit_child_exprs_mut(visitor),
            StmtType::Return(inner) => inner.visit_child_exprs_mut(visitor),
            StmtType::Throw(inner) => inner.visit_child_exprs_mut(visitor),
            StmtType::Delete(inner) => inner.visit_child_exprs_mut(visitor),
            StmtType::Assignment(inner) => inner.visit_child_exprs_mut(visitor),
            StmtType::Expr(inner) => visitor(inner),
            StmtType::Break | StmtType::Continue | StmtType::Exit => {}
        }
    }

    fn visit_child_stmts<S>(&self, visitor: S)
    where
        S: FnMut(&Stmt),
    {
        match self {
            StmtType::MacroDeclaration(inner) => inner.visit_child_stmts(visitor),
            StmtType::GlobalvarDeclaration(inner) => inner.visit_child_stmts(visitor),
            StmtType::LocalVariableSeries(inner) => inner.visit_child_stmts(visitor),
            StmtType::TryCatch(inner) => inner.visit_child_stmts(visitor),
            StmtType::ForLoop(inner) => inner.visit_child_stmts(visitor),
            StmtType::WithLoop(inner) => inner.visit_child_stmts(visitor),
            StmtType::RepeatLoop(inner) => inner.visit_child_stmts(visitor),
            StmtType::DoUntil(inner) => inner.visit_child_stmts(visitor),
            StmtType::WhileLoop(inner) => inner.visit_child_stmts(visitor),
            StmtType::If(inner) => inner.visit_child_stmts(visitor),
            StmtType::Switch(inner) => inner.visit_child_stmts(visitor),
            StmtType::Block(inner) => inner.visit_child_stmts(visitor),
            StmtType::Return(inner) => inner.visit_child_stmts(visitor),
            StmtType::Throw(inner) => inner.visit_child_stmts(visitor),
            StmtType::Delete(inner) => inner.visit_child_stmts(visitor),
            StmtType::Assignment(inner) => inner.visit_child_stmts(visitor),
            StmtType::Expr(_) => {}
            StmtType::Break | StmtType::Continue | StmtType::Exit => {}
        }
    }

    fn visit_child_stmts_mut<S>(&mut self, visitor: S)
    where
        S: FnMut(&mut Stmt),
    {
        match self {
            StmtType::MacroDeclaration(inner) => inner.visit_child_stmts_mut(visitor),
            StmtType::GlobalvarDeclaration(inner) => inner.visit_child_stmts_mut(visitor),
            StmtType::LocalVariableSeries(inner) => inner.visit_child_stmts_mut(visitor),
            StmtType::TryCatch(inner) => inner.visit_child_stmts_mut(visitor),
            StmtType::ForLoop(inner) => inner.visit_child_stmts_mut(visitor),
            StmtType::WithLoop(inner) => inner.visit_child_stmts_mut(visitor),
            StmtType::RepeatLoop(inner) => inner.visit_child_stmts_mut(visitor),
            StmtType::DoUntil(inner) => inner.visit_child_stmts_mut(visitor),
            StmtType::WhileLoop(inner) => inner.visit_child_stmts_mut(visitor),
            StmtType::If(inner) => inner.visit_child_stmts_mut(visitor),
            StmtType::Switch(inner) => inner.visit_child_stmts_mut(visitor),
            StmtType::Block(inner) => inner.visit_child_stmts_mut(visitor),
            StmtType::Return(inner) => inner.visit_child_stmts_mut(visitor),
            StmtType::Throw(inner) => inner.visit_child_stmts_mut(visitor),
            StmtType::Delete(inner) => inner.visit_child_stmts_mut(visitor),
            StmtType::Assignment(inner) => inner.visit_child_stmts_mut(visitor),
            StmtType::Expr(_) => {}
            StmtType::Break | StmtType::Continue | StmtType::Exit => {}
        }
    }
}
impl StmtType {
    /// Returns the statement as a [Block] if it is a block.
    pub fn as_block(&self) -> Option<&Block> {
        match self {
            StmtType::Block(block) => Some(block),
            _ => None,
        }
    }
    /// Returns the statement as an [Expr] if it is an expression statement.
    pub fn as_expr_stmt(&self) -> Option<&Expr> {
        match self {
            StmtType::Expr(expr) => Some(expr),
            _ => None,
        }
    }

    /// Returns the statement as an [Assignment] if it is an assignment statement.
    pub fn as_assignment(&self) -> Option<&Assignment> {
        match self {
            StmtType::Assignment(assignment) => Some(assignment),
            _ => None,
        }
    }
}

/// A wrapper around a Stmt, containing additional information discovered while parsing.
#[derive(Debug, PartialEq, Clone)]
pub struct Stmt {
    stmt_type: Box<StmtType>,
    location: Location,
    lint_tag: Option<LintTag>,
}
impl Stmt {
    /// Returns a reference to the inner stmt type.
    pub fn inner(&self) -> &StmtType {
        self.stmt_type.as_ref()
    }
    /// Returns a mutable reference to the inner stmt type.
    pub fn inner_mut(&mut self) -> &mut StmtType {
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
    /// Returns the lint tag attached to this statement, if any.
    pub fn lint_tag(&self) -> Option<&LintTag> {
        self.lint_tag.as_ref()
    }
}
impl ParseVisitor for Stmt {
    fn visit_child_exprs<E: FnMut(&Expr)>(&self, visitor: E) {
        self.inner().visit_child_exprs(visitor)
    }
    fn visit_child_exprs_mut<E: FnMut(&mut Expr)>(&mut self, visitor: E) {
        self.inner_mut().visit_child_exprs_mut(visitor)
    }
    fn visit_child_stmts<S: FnMut(&Stmt)>(&self, visitor: S) {
        self.inner().visit_child_stmts(visitor)
    }
    fn visit_child_stmts_mut<S: FnMut(&mut Stmt)>(&mut self, visitor: S) {
        self.inner_mut().visit_child_stmts_mut(visitor)
    }
}

/// Derives two methods to convert the T into an [Stmt], supporting both a standard
/// `into_stmt` method, and a `into_stmt_lazy` for tests.
pub trait IntoStmt: Sized + Into<StmtType> {
    /// Converts self into an statement box.
    fn into_stmt(self, span: Span, file_id: FileId, lint_tag: Option<LintTag>) -> Stmt {
        Stmt {
            stmt_type: Box::new(self.into()),
            location: Location(file_id, span),
            lint_tag,
        }
    }

    /// Converts self into an statement box with a default span. Used in tests, where all spans are
    /// expected to be 0, 0.
    fn into_stmt_lazy(self) -> Stmt
    where
        Self: Sized,
    {
        self.into_stmt(Default::default(), 0, None)
    }
}
