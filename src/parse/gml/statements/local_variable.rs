use crate::parse::{ExpressionBox, IntoStatementBox, ParseVisitor, Statement, StatementBox};

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
        Self::LocalVariableSeries(series)
    }
}
impl IntoStatementBox for LocalVariableSeries {}
impl ParseVisitor for LocalVariableSeries {
    fn visit_child_expressions<E: FnMut(&ExpressionBox)>(&self, mut expression_visitor: E) {
        for declaration in self.declarations.iter() {
            match declaration {
                LocalVariable::Uninitialized(expr) => expression_visitor(expr),
                LocalVariable::Initialized(_) => {}
            }
        }
    }

    fn visit_child_statements<S: FnMut(&StatementBox)>(&self, mut statement_visitor: S) {
        for declaration in self.declarations.iter() {
            match declaration {
                LocalVariable::Uninitialized(_) => {}
                LocalVariable::Initialized(stmt) => statement_visitor(stmt),
            }
        }
    }
}

/// Representation of a local variable in gml.
///
/// Since local variables can be initialized without a value, this is divided in two -- one for
/// local variables that are just an identifier, and one for local variables that are a full
/// assignment.
#[derive(Debug, PartialEq, Clone)]
pub enum LocalVariable {
    /// Uninitialized local variable declarations, containing only their name as an identifier as an
    /// expression.
    Uninitialized(ExpressionBox),
    /// Initialized local variables containing their full assignment statement.
    Initialized(StatementBox),
}
impl LocalVariable {
    /// Retrieves the name of the local variable.
    ///
    /// FIXME: This code sure looks bad
    pub fn name(&self) -> &str {
        match self {
            LocalVariable::Uninitialized(expression_box) => {
                &expression_box
                    .expression()
                    .as_identifier()
                    .unwrap_or_else(|| unreachable!())
                    .name
            }
            LocalVariable::Initialized(statement_box) => statement_box
                .statement()
                .as_assignment()
                .unwrap_or_else(|| unreachable!())
                .left
                .expression()
                .as_identifier()
                .unwrap_or_else(|| unreachable!())
                .name
                .as_ref(),
        }
    }
}
