use crate::{
    analyze::Scope,
    lint::LintTag,
    parse::{
        Block, Delete, DoUntil, Enum, ExpressionBox, ForLoop, Globalvar, If, LocalVariableSeries, Location, Macro,
        ParseVisitor, RepeatLoop, Span, Switch, TryCatch, WhileLoop, WithLoop,
    },
    FileId,
};

use super::{Assignment, Return, Throw};

/// A singular gml statement.
#[derive(Debug, PartialEq, Clone)]
pub enum Statement {
    /// Declaration of a macro.
    MacroDeclaration(Macro),
    /// Declaration of an enum.
    EnumDeclaration(Enum),
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
    Expression(ExpressionBox),
}
impl IntoStatementBox for Statement {}
impl ParseVisitor for Statement {
    fn visit_child_expressions<E>(&self, mut visitor: E)
    where
        E: FnMut(&ExpressionBox),
    {
        match self {
            Statement::MacroDeclaration(inner) => inner.visit_child_expressions(visitor),
            Statement::EnumDeclaration(inner) => inner.visit_child_expressions(visitor),
            Statement::GlobalvarDeclaration(inner) => inner.visit_child_expressions(visitor),
            Statement::LocalVariableSeries(inner) => inner.visit_child_expressions(visitor),
            Statement::TryCatch(inner) => inner.visit_child_expressions(visitor),
            Statement::ForLoop(inner) => inner.visit_child_expressions(visitor),
            Statement::WithLoop(inner) => inner.visit_child_expressions(visitor),
            Statement::RepeatLoop(inner) => inner.visit_child_expressions(visitor),
            Statement::DoUntil(inner) => inner.visit_child_expressions(visitor),
            Statement::WhileLoop(inner) => inner.visit_child_expressions(visitor),
            Statement::If(inner) => inner.visit_child_expressions(visitor),
            Statement::Switch(inner) => inner.visit_child_expressions(visitor),
            Statement::Block(inner) => inner.visit_child_expressions(visitor),
            Statement::Return(inner) => inner.visit_child_expressions(visitor),
            Statement::Throw(inner) => inner.visit_child_expressions(visitor),
            Statement::Delete(inner) => inner.visit_child_expressions(visitor),
            Statement::Assignment(inner) => inner.visit_child_expressions(visitor),
            Statement::Expression(inner) => visitor(inner),
            Statement::Break | Statement::Continue | Statement::Exit => {}
        }
    }
    fn visit_child_expressions_mut<E>(&mut self, mut visitor: E)
    where
        E: FnMut(&mut ExpressionBox),
    {
        match self {
            Statement::MacroDeclaration(inner) => inner.visit_child_expressions_mut(visitor),
            Statement::EnumDeclaration(inner) => inner.visit_child_expressions_mut(visitor),
            Statement::GlobalvarDeclaration(inner) => inner.visit_child_expressions_mut(visitor),
            Statement::LocalVariableSeries(inner) => inner.visit_child_expressions_mut(visitor),
            Statement::TryCatch(inner) => inner.visit_child_expressions_mut(visitor),
            Statement::ForLoop(inner) => inner.visit_child_expressions_mut(visitor),
            Statement::WithLoop(inner) => inner.visit_child_expressions_mut(visitor),
            Statement::RepeatLoop(inner) => inner.visit_child_expressions_mut(visitor),
            Statement::DoUntil(inner) => inner.visit_child_expressions_mut(visitor),
            Statement::WhileLoop(inner) => inner.visit_child_expressions_mut(visitor),
            Statement::If(inner) => inner.visit_child_expressions_mut(visitor),
            Statement::Switch(inner) => inner.visit_child_expressions_mut(visitor),
            Statement::Block(inner) => inner.visit_child_expressions_mut(visitor),
            Statement::Return(inner) => inner.visit_child_expressions_mut(visitor),
            Statement::Throw(inner) => inner.visit_child_expressions_mut(visitor),
            Statement::Delete(inner) => inner.visit_child_expressions_mut(visitor),
            Statement::Assignment(inner) => inner.visit_child_expressions_mut(visitor),
            Statement::Expression(inner) => visitor(inner),
            Statement::Break | Statement::Continue | Statement::Exit => {}
        }
    }

    fn visit_child_statements<S>(&self, visitor: S)
    where
        S: FnMut(&StatementBox),
    {
        match self {
            Statement::MacroDeclaration(inner) => inner.visit_child_statements(visitor),
            Statement::EnumDeclaration(inner) => inner.visit_child_statements(visitor),
            Statement::GlobalvarDeclaration(inner) => inner.visit_child_statements(visitor),
            Statement::LocalVariableSeries(inner) => inner.visit_child_statements(visitor),
            Statement::TryCatch(inner) => inner.visit_child_statements(visitor),
            Statement::ForLoop(inner) => inner.visit_child_statements(visitor),
            Statement::WithLoop(inner) => inner.visit_child_statements(visitor),
            Statement::RepeatLoop(inner) => inner.visit_child_statements(visitor),
            Statement::DoUntil(inner) => inner.visit_child_statements(visitor),
            Statement::WhileLoop(inner) => inner.visit_child_statements(visitor),
            Statement::If(inner) => inner.visit_child_statements(visitor),
            Statement::Switch(inner) => inner.visit_child_statements(visitor),
            Statement::Block(inner) => inner.visit_child_statements(visitor),
            Statement::Return(inner) => inner.visit_child_statements(visitor),
            Statement::Throw(inner) => inner.visit_child_statements(visitor),
            Statement::Delete(inner) => inner.visit_child_statements(visitor),
            Statement::Assignment(inner) => inner.visit_child_statements(visitor),
            Statement::Expression(_) => {}
            Statement::Break | Statement::Continue | Statement::Exit => {}
        }
    }

    fn visit_child_statements_mut<S>(&mut self, visitor: S)
    where
        S: FnMut(&mut StatementBox),
    {
        match self {
            Statement::MacroDeclaration(inner) => inner.visit_child_statements_mut(visitor),
            Statement::EnumDeclaration(inner) => inner.visit_child_statements_mut(visitor),
            Statement::GlobalvarDeclaration(inner) => inner.visit_child_statements_mut(visitor),
            Statement::LocalVariableSeries(inner) => inner.visit_child_statements_mut(visitor),
            Statement::TryCatch(inner) => inner.visit_child_statements_mut(visitor),
            Statement::ForLoop(inner) => inner.visit_child_statements_mut(visitor),
            Statement::WithLoop(inner) => inner.visit_child_statements_mut(visitor),
            Statement::RepeatLoop(inner) => inner.visit_child_statements_mut(visitor),
            Statement::DoUntil(inner) => inner.visit_child_statements_mut(visitor),
            Statement::WhileLoop(inner) => inner.visit_child_statements_mut(visitor),
            Statement::If(inner) => inner.visit_child_statements_mut(visitor),
            Statement::Switch(inner) => inner.visit_child_statements_mut(visitor),
            Statement::Block(inner) => inner.visit_child_statements_mut(visitor),
            Statement::Return(inner) => inner.visit_child_statements_mut(visitor),
            Statement::Throw(inner) => inner.visit_child_statements_mut(visitor),
            Statement::Delete(inner) => inner.visit_child_statements_mut(visitor),
            Statement::Assignment(inner) => inner.visit_child_statements_mut(visitor),
            Statement::Expression(_) => {}
            Statement::Break | Statement::Continue | Statement::Exit => {}
        }
    }
}
impl Statement {
    /// Returns the statement as a [Block] if it is a block.
    pub fn as_block(&self) -> Option<&Block> {
        match self {
            Statement::Block(block) => Some(block),
            _ => None,
        }
    }
    /// Returns the statement as an [ExpressionBox] if it is an expression statement.
    pub fn as_expression_statement(&self) -> Option<&ExpressionBox> {
        match self {
            Statement::Expression(expression_box) => Some(expression_box),
            _ => None,
        }
    }

    /// Returns the statement as an [Assignment] if it is an assignment statement.
    pub fn as_assignment(&self) -> Option<&Assignment> {
        match self {
            Statement::Assignment(assignment) => Some(assignment),
            _ => None,
        }
    }
}

/// A wrapper around a Statement, containing additional information discovered while parsing.
#[derive(Debug, PartialEq, Clone)]
pub struct StatementBox {
    pub statement: Box<Statement>,
    pub scope: Option<Scope>,
    location: Location,
    lint_tag: Option<LintTag>,
}
impl StatementBox {
    /// Returns a reference to the inner statement.
    pub fn statement(&self) -> &Statement {
        self.statement.as_ref()
    }
    /// Returns a mutable reference to the inner statement.
    pub fn statement_mut(&mut self) -> &mut Statement {
        self.statement.as_mut()
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
impl ParseVisitor for StatementBox {
    fn visit_child_expressions<E: FnMut(&ExpressionBox)>(&self, visitor: E) {
        self.statement().visit_child_expressions(visitor)
    }
    fn visit_child_expressions_mut<E: FnMut(&mut ExpressionBox)>(&mut self, visitor: E) {
        self.statement_mut().visit_child_expressions_mut(visitor)
    }
    fn visit_child_statements<S: FnMut(&StatementBox)>(&self, visitor: S) {
        self.statement().visit_child_statements(visitor)
    }
    fn visit_child_statements_mut<S: FnMut(&mut StatementBox)>(&mut self, visitor: S) {
        self.statement_mut().visit_child_statements_mut(visitor)
    }
}

/// Derives two methods to convert the T into an [StatementBox], supporting both a standard
/// `into_statement_box` method, and a `into_lazy_box` for tests.
pub trait IntoStatementBox: Sized + Into<Statement> {
    /// Converts self into an statement box.
    fn into_statement_box(self, span: Span, file_id: FileId, lint_tag: Option<LintTag>) -> StatementBox {
        StatementBox {
            statement: Box::new(self.into()),
            scope: None,
            location: Location(file_id, span),
            lint_tag,
        }
    }

    /// Converts self into an statement box with a default span. Used in tests, where all spans are
    /// expected to be 0, 0.
    fn into_lazy_box(self) -> StatementBox
    where
        Self: Sized,
    {
        self.into_statement_box(Default::default(), 0, None)
    }
}
