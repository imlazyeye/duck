use crate::prelude::{ExpressionBox, IntoStatementBox, ParseVisitor, Statement, StatementBox};

/// Representation of a local variable declaration.
///
/// Slightly more complicated than other types due to how many local variables can be declared in
/// one swoop.
#[derive(Debug, PartialEq, Clone)]
pub struct LocalVariableSeries {
    /// The various declarations in this series.
    pub declarations: Vec<LocalVariable>,
}
impl LocalVariableSeries {
    /// Creates a new local variable series.
    pub fn new(declarations: Vec<LocalVariable>) -> Self {
        Self { declarations }
    }
}
impl From<LocalVariableSeries> for Statement {
    fn from(series: LocalVariableSeries) -> Self {
        Statement::LocalVariableSeries(series)
    }
}
impl IntoStatementBox for LocalVariableSeries {}
impl ParseVisitor for LocalVariableSeries {
    fn visit_child_expressions<E: FnMut(&ExpressionBox)>(&self, mut expression_visitor: E) {
        for declaration in self.declarations.iter() {
            expression_visitor(declaration.inner());
        }
    }

    fn visit_child_statements<S: FnMut(&StatementBox)>(&self, _statement_visitor: S) {}
}

/// Representation of a local variable in gml.
///
/// Since local variables can be initialized without a value, this is divided in two -- one for
/// local variables that are just an identifier, and one for local variables that are a full
/// assignment.
#[derive(Debug, PartialEq, Clone)]
pub enum LocalVariable {
    /// Uninitialized local variable declarations, containing only their name as an identifier.
    Uninitialized(ExpressionBox),
    /// Initialized local variables containing their full assignment expression.
    Initialized(ExpressionBox),
}
impl LocalVariable {
    /// Retrieves the name of the local variable.
    pub fn name(&self) -> &str {
        match self {
            LocalVariable::Uninitialized(expression_box) => &expression_box.expression().as_identifier().unwrap().name,
            LocalVariable::Initialized(expression_box) => expression_box
                .expression()
                .as_assignment()
                .unwrap()
                .left
                .expression()
                .as_identifier()
                .unwrap()
                .name
                .as_ref(),
        }
    }

    /// Returns a reference to the inner expression box.
    pub fn inner(&self) -> &ExpressionBox {
        match self {
            LocalVariable::Uninitialized(e) => e,
            LocalVariable::Initialized(e) => e,
        }
    }
}
