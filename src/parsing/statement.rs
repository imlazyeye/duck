use crate::{
    parsing::{
        Block, DoUntil, Enum, ExpressionBox, ForLoop, Globalvar, If, LocalVariableSeries, Macro, RepeatLoop, Return,
        Switch, TryCatch, WhileLoop, WithLoop,
    },
    utils::Span,
};

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
    /// A return statement, with an optional return value.
    Return(Return),
    /// A break statement (from within a switch statement).
    Break,
    /// A continue statement (from within a continue statement).
    Continue,
    /// An exit statement.
    Exit,
    /// A statement expression, or in other words, an expression being executed on its own.
    /// Common examples below:
    /// ```gml
    /// foo(); // call expression
    /// foo += 1; // assignment expression
    /// foo++; // postfix expression
    /// ```
    Expression(ExpressionBox),
}
impl IntoStatementBox for Statement {}
impl ParseVisitor for Statement {
    fn visit_child_statements<S>(&self, statement_visitor: S)
    where
        S: FnMut(&StatementBox),
    {
        match self {
            Statement::MacroDeclaration(inner) => inner.visit_child_statements(statement_visitor),
            Statement::EnumDeclaration(inner) => inner.visit_child_statements(statement_visitor),
            Statement::GlobalvarDeclaration(inner) => inner.visit_child_statements(statement_visitor),
            Statement::LocalVariableSeries(inner) => inner.visit_child_statements(statement_visitor),
            Statement::TryCatch(inner) => inner.visit_child_statements(statement_visitor),
            Statement::ForLoop(inner) => inner.visit_child_statements(statement_visitor),
            Statement::WithLoop(inner) => inner.visit_child_statements(statement_visitor),
            Statement::RepeatLoop(inner) => inner.visit_child_statements(statement_visitor),
            Statement::DoUntil(inner) => inner.visit_child_statements(statement_visitor),
            Statement::WhileLoop(inner) => inner.visit_child_statements(statement_visitor),
            Statement::If(inner) => inner.visit_child_statements(statement_visitor),
            Statement::Switch(inner) => inner.visit_child_statements(statement_visitor),
            Statement::Block(inner) => inner.visit_child_statements(statement_visitor),
            Statement::Return(inner) => inner.visit_child_statements(statement_visitor),
            Statement::Expression(inner) => inner.visit_child_statements(statement_visitor),
            Statement::Break | Statement::Continue | Statement::Exit => {}
        }
    }

    fn visit_child_expressions<E>(&self, expression_visitor: E)
    where
        E: FnMut(&ExpressionBox),
    {
        match self {
            Statement::MacroDeclaration(inner) => inner.visit_child_expressions(expression_visitor),
            Statement::EnumDeclaration(inner) => inner.visit_child_expressions(expression_visitor),
            Statement::GlobalvarDeclaration(inner) => inner.visit_child_expressions(expression_visitor),
            Statement::LocalVariableSeries(inner) => inner.visit_child_expressions(expression_visitor),
            Statement::TryCatch(inner) => inner.visit_child_expressions(expression_visitor),
            Statement::ForLoop(inner) => inner.visit_child_expressions(expression_visitor),
            Statement::WithLoop(inner) => inner.visit_child_expressions(expression_visitor),
            Statement::RepeatLoop(inner) => inner.visit_child_expressions(expression_visitor),
            Statement::DoUntil(inner) => inner.visit_child_expressions(expression_visitor),
            Statement::WhileLoop(inner) => inner.visit_child_expressions(expression_visitor),
            Statement::If(inner) => inner.visit_child_expressions(expression_visitor),
            Statement::Switch(inner) => inner.visit_child_expressions(expression_visitor),
            Statement::Block(inner) => inner.visit_child_expressions(expression_visitor),
            Statement::Return(inner) => inner.visit_child_expressions(expression_visitor),
            Statement::Expression(inner) => inner.visit_child_expressions(expression_visitor),
            Statement::Break | Statement::Continue | Statement::Exit => {}
        }
    }
}

/// A wrapper around a Statement. Serves a few purposes:
///
/// 1. Prevents infinite-sizing issues on [Statement] (type T cannot itself directly hold another T)
/// 2. Contains the [Span] that describes where this statement came from
/// 3. In the future, will hold static-analysis data
#[derive(Debug, PartialEq, Clone)]
pub struct StatementBox(Box<Statement>, Span);
impl StatementBox {
    /// Returns a reference to the inner statement.
    pub fn statement(&self) -> &Statement {
        self.0.as_ref()
    }
    /// Returns a reference to the [Span] this statement originates from.
    pub fn span(&self) -> Span {
        self.1
    }
}

/// Derives two methods to convert the T into an [StatementBox], supporting both a standard
/// `into_statement_box` method, and a `into_lazy_box` for tests.
///
/// TODO: This could be a derive macro!
pub trait IntoStatementBox: Sized + Into<Statement> {
    /// Converts self into an statement box with a provided span.
    fn into_statement_box(self, span: Span) -> StatementBox {
        StatementBox(Box::new(self.into()), span)
    }

    /// Converts self into an statement box with a default span. Used in tests, where all spans are
    /// expected to be 0, 0.
    fn into_lazy_box(self) -> StatementBox
    where
        Self: Sized,
    {
        self.into_statement_box(Default::default())
    }
}

/// Used to visit the children of a Statement as we recurse down the tree.
pub trait ParseVisitor {
    /// Visits all expressions this T contains.
    fn visit_child_expressions<E: FnMut(&ExpressionBox)>(&self, expression_visitor: E);

    /// Visits all statements this T contains.
    fn visit_child_statements<S: FnMut(&StatementBox)>(&self, statement_visitor: S);
}
