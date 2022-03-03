use crate::parsing::{Expression, ExpressionBox, IntoExpressionBox, ParseVisitor, StatementBox, Token};

/// Representation of an assignment expression in gml.
///
/// ### About parsing assignments
/// gml supports single-equals tokens as equality operators. This is demonic, and our solution isn't
/// amazing (because there isn't a great one, though please correct me if I'm wrong...
///
/// To support this, our recursive descent is set up with assignment *above* equality. This mean
/// that equality has "first dibs" at the an expression with the following grammar:
/// ```grammar
/// EXPRESSION = EXPRESSION
/// ```
/// But since this grammar is also valid as an assignment, this means that our parsing will *never*
/// naturally evaluate an assignment -- it treats everything as an equality. The only way we can
/// eventually have an assignment is by having a single-equals equality be the base of an expression
/// statement, at which time it is converted.
///
/// This sounds terrible (it is), but through my testing, it seems to be exaclty how gml functions.
/// The cost of this is that [Parser::expression] can never return an assignment, which will make
/// our unit testing strange.
///
/// I would very much like to improve this if at all possible in the future, but for now, its more
/// important to me that our parsing is accurate.
#[derive(Debug, PartialEq, Clone)]
pub struct Assignment {
    /// The left hand side of the assignment, aka the target.
    pub left: ExpressionBox,
    /// The operator used in this assignment.
    pub operator: AssignmentOperator,
    /// The right hand side of the assignment, aka the value.
    pub right: ExpressionBox,
}
impl Assignment {
    /// Creates a new assignment.
    pub fn new(left: ExpressionBox, operator: AssignmentOperator, right: ExpressionBox) -> Self {
        Self { left, operator, right }
    }
}
impl From<Assignment> for Expression {
    fn from(assignment: Assignment) -> Self {
        Self::Assignment(assignment)
    }
}
impl IntoExpressionBox for Assignment {}
impl ParseVisitor for Assignment {
    fn visit_child_expressions<E: FnMut(&ExpressionBox)>(&self, mut expression_visitor: E) {
        expression_visitor(&self.left);
        expression_visitor(&self.right);
    }
    fn visit_child_statements<S: FnMut(&StatementBox)>(&self, _statement_visitor: S) {}
}

/// The various assignment operations supported in gml.
#[derive(Debug, PartialEq, Copy, Clone)]
#[allow(clippy::enum_variant_names)]
pub enum AssignmentOperator {
    /// =
    Equal(Token),
    /// +=
    PlusEqual(Token),
    /// -=
    MinusEqual(Token),
    /// *=
    StarEqual(Token),
    /// /=
    SlashEqual(Token),
    /// ^=
    XorEqual(Token),
    /// |=
    OrEqual(Token),
    /// &=
    AndEqual(Token),
    /// ??=
    NullCoalecenceEqual(Token),
    /// %=
    ModEqual(Token),
}
