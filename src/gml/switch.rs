use crate::{
    parsing::{Expression, ExpressionBox, Scope, StatementBox},
    prelude::{IntoStatementBox, ParseVisitor, Statement},
};

/// Representation of a gml switch statement.
#[derive(Debug, PartialEq, Clone)]
pub struct Switch {
    matching_value: ExpressionBox,
    cases: Vec<SwitchCase>,
    default_case: Option<Vec<StatementBox>>,
}
impl Switch {
    /// Creates a new switch with the provided matching value, cases, and
    /// optionally a default case.
    pub fn new(matching_value: ExpressionBox, cases: Vec<SwitchCase>, default_case: Option<Vec<StatementBox>>) -> Self {
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
            .any(|case| !matches!(case.identity().expression(), Expression::Access(Scope::Dot(_), _)))
    }

    /// Returns the name of the enum this switch statement matches over, if any.
    /// Please note that this check is done just by reading the first case
    /// member. It is possible that the user is doing something cursed, such
    /// as...
    /// ```gml
    /// switch foo {
    ///     case Foo.Bar: break;
    ///     case Buzz.Baz: break;
    /// }
    /// ```
    pub fn potential_enum_type(&self) -> Option<&str> {
        self.cases.first().and_then(|case| {
            case.identity()
                .expression()
                .as_dot_access()
                .and_then(|(left, _)| left.as_identifier())
                .map(|iden| iden.name.as_ref())
        })
    }

    /// Get a reference to the switch's matching value. Ie: foo in `switch foo`
    pub fn matching_value(&self) -> &ExpressionBox {
        &self.matching_value
    }

    /// Get a reference to the switch's cases.
    pub fn cases(&self) -> &[SwitchCase] {
        self.cases.as_ref()
    }

    /// Get a reference to the switch's default case.
    pub fn default_case(&self) -> Option<&Vec<StatementBox>> {
        self.default_case.as_ref()
    }
}
impl From<Switch> for Statement {
    fn from(switch: Switch) -> Self {
        Self::Switch(switch)
    }
}
impl IntoStatementBox for Switch {}
impl ParseVisitor for Switch {
    fn visit_child_expressions<E: FnMut(&ExpressionBox)>(&self, mut expression_visitor: E) {
        expression_visitor(self.matching_value());
        for case in self.cases() {
            expression_visitor(case.identity());
        }
    }

    fn visit_child_statements<S: FnMut(&StatementBox)>(&self, mut statement_visitor: S) {
        for case in self.cases() {
            for statement in case.iter_body_statements() {
                statement_visitor(statement);
            }
        }
        if let Some(default) = self.default_case() {
            for statement in default.iter() {
                statement_visitor(statement);
            }
        }
    }
}

/// Representation of a single switch case in a [GmlSwitch].
///
/// FIXME: Case bodies are currently a Vec<StatementBox> which is essentially
/// a [Statement::Block]. I originally chose to do this because I was concerned
/// that lints would want to safely assume that a `Block` meant something with
/// curly braces, but I now believe that `Blocks` should instead be better
/// equipped to express whether or not they contain curly braces, and that case
/// bodies should be made into `Block`s. While its not of huge concern right
/// now, it will be an issue when static analyis is added, as case bodies won't
/// properly create a new scope.
#[derive(Debug, PartialEq, Clone)]
pub struct SwitchCase(ExpressionBox, Vec<StatementBox>);
impl SwitchCase {
    /// Creates a new GmlSwitchCase with the given identity and body.
    pub fn new(identity: ExpressionBox, body: Vec<StatementBox>) -> Self {
        Self(identity, body)
    }

    /// Returns a reference to the case's identity.
    pub fn identity(&self) -> &ExpressionBox {
        &self.0
    }

    /// Returns an iterator over the body of the case.
    ///
    /// NOTE: In the future, case bodies may be changed into a
    /// `Statement::Block`. When this happens, this function may be changed or
    /// removed.
    pub fn iter_body_statements(&self) -> impl Iterator<Item = &StatementBox> {
        self.1.iter()
    }
}
