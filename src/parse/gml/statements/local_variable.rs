use crate::parse::{ExpressionBox, IntoStatementBox, OptionalInitilization, ParseVisitor, Statement, StatementBox};

/// Representation of a local variable declaration.
///
/// Slightly more complicated than other types due to how many local variables can be declared in
/// one swoop.
#[derive(Debug, PartialEq, Clone)]
pub struct LocalVariableSeries {
    /// The various declarations in this series.
    pub declarations: Vec<OptionalInitilization>,
}
impl LocalVariableSeries {
    /// Creates a new local variable series.
    pub fn new(declarations: Vec<OptionalInitilization>) -> Self {
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
    fn visit_child_expressions<E: FnMut(&ExpressionBox)>(&self, mut visitor: E) {
        for declaration in self.declarations.iter() {
            match declaration {
                OptionalInitilization::Uninitialized(expr) => visitor(expr),
                OptionalInitilization::Initialized(_) => {}
            }
        }
    }
    fn visit_child_expressions_mut<E: FnMut(&mut ExpressionBox)>(&mut self, mut visitor: E) {
        for declaration in self.declarations.iter_mut() {
            match declaration {
                OptionalInitilization::Uninitialized(expr) => visitor(expr),
                OptionalInitilization::Initialized(_) => {}
            }
        }
    }
    fn visit_child_statements<S: FnMut(&StatementBox)>(&self, mut visitor: S) {
        for declaration in self.declarations.iter() {
            match declaration {
                OptionalInitilization::Uninitialized(_) => {}
                OptionalInitilization::Initialized(stmt) => visitor(stmt),
            }
        }
    }
    fn visit_child_statements_mut<S: FnMut(&mut StatementBox)>(&mut self, mut visitor: S) {
        for declaration in self.declarations.iter_mut() {
            match declaration {
                OptionalInitilization::Uninitialized(_) => {}
                OptionalInitilization::Initialized(stmt) => visitor(stmt),
            }
        }
    }
}
