use crate::parsing::{
    expression::{Expression, ExpressionBox, Scope},
    statement::{Statement, StatementBox},
};

#[derive(Debug, PartialEq, Clone)]
pub struct GmlSwitch {
    matching_value: ExpressionBox,
    cases: Vec<GmlSwitchCase>,
    default_case: Option<Vec<StatementBox>>,
}
impl GmlSwitch {
    pub fn new(
        matching_value: ExpressionBox,
        cases: Vec<GmlSwitchCase>,
        default_case: Option<Vec<StatementBox>>,
    ) -> Self {
        Self {
            matching_value,
            cases,
            default_case,
        }
    }

    /// Returns if all the cases of this enum are dot-access expressions.
    /// Given valid GML, this is a guarentee that they are all enums. With
    /// invalid GML, these could be ANY dot-access expressions.
    pub fn all_case_members_dot_access(&self) -> bool {
        !self
            .cases
            .iter()
            .any(|case| !matches!(case.identity(), Expression::Access(Scope::Dot(_), _)))
    }

    /// Returns the name of the enum this switch statement matches over, if any.
    /// Please note that this check is done just by reading the first case member.
    /// It is possible that the user is doing something cursed, such as...
    /// ```gml
    /// switch foo {
    ///     case Foo.Bar: break;
    ///     case Buzz.Baz: break;
    /// }
    /// ```
    pub fn potential_enum_type(&self) -> Option<&str> {
        self.cases.first().and_then(|case| {
            case.identity()
                .as_dot_access()
                .and_then(|(left, _)| left.as_identifier())
        })
    }

    /// Get a reference to the gml switch's matching value.
    pub fn matching_value(&self) -> &ExpressionBox {
        &self.matching_value
    }

    /// Get a reference to the gml switch's cases.
    pub fn cases(&self) -> &[GmlSwitchCase] {
        self.cases.as_ref()
    }

    /// Get a reference to the gml switch's default case.
    pub fn default_case(&self) -> Option<&Vec<StatementBox>> {
        self.default_case.as_ref()
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct GmlSwitchCase(ExpressionBox, Vec<StatementBox>); // kinda a block?
impl GmlSwitchCase {
    pub fn new(identity: ExpressionBox, body: Vec<StatementBox>) -> Self {
        Self(identity, body)
    }

    pub fn identity(&self) -> &Expression {
        self.0.expression()
    }

    pub fn identity_box(&self) -> &ExpressionBox {
        &self.0
    }

    pub fn iter_body_statements(&self) -> impl Iterator<Item = &Statement> {
        self.1.iter().map(|stmt| stmt.statement())
    }

    pub fn iter_body_statement_boxes(&self) -> impl Iterator<Item = &StatementBox> {
        self.1.iter()
    }
}
