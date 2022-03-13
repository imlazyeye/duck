use crate::parse::{Access, Expr, ExprType, IntoStmt, ParseVisitor, Stmt, StmtType};

/// Representation of a gml switch statement.
#[derive(Debug, PartialEq, Clone)]
pub struct Switch {
    /// The value this switch statement is matching over.
    pub matching_value: Expr,
    /// The various cases in this switch statement.
    pub cases: Vec<SwitchCase>,
    /// The default case body, if any.
    pub default_case: Option<Vec<Stmt>>,
}
impl Switch {
    /// Creates a new switch with the provided matching value, cases, and
    /// optionally a default case.
    pub fn new(matching_value: Expr, cases: Vec<SwitchCase>, default_case: Option<Vec<Stmt>>) -> Self {
        Self {
            matching_value,
            cases,
            default_case,
        }
    }

    /// Returns if all the cases of this enum are dot-access expressions.
    pub fn all_case_members_dot_access(&self) -> bool {
        !self
            .cases
            .iter()
            .any(|case| !matches!(case.identity().inner(), ExprType::Access(Access::Dot { .. },)))
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
                .inner()
                .as_dot_access()
                .and_then(|(left, _)| left.as_identifier())
                .map(|iden| iden.lexeme.as_ref())
        })
    }

    /// Get a reference to the switch's matching value. Ie: foo in `switch foo`
    pub fn matching_value(&self) -> &Expr {
        &self.matching_value
    }

    /// Get a reference to the switch's cases.
    pub fn cases(&self) -> &[SwitchCase] {
        self.cases.as_ref()
    }

    /// Get a reference to the switch's default case.
    pub fn default_case(&self) -> Option<&Vec<Stmt>> {
        self.default_case.as_ref()
    }
}
impl From<Switch> for StmtType {
    fn from(switch: Switch) -> Self {
        Self::Switch(switch)
    }
}
impl IntoStmt for Switch {}
impl ParseVisitor for Switch {
    fn visit_child_exprs<E: FnMut(&Expr)>(&self, mut visitor: E) {
        visitor(&self.matching_value);
        for case in self.cases.iter() {
            visitor(&case.0);
        }
    }
    fn visit_child_exprs_mut<E: FnMut(&mut Expr)>(&mut self, mut visitor: E) {
        visitor(&mut self.matching_value);
        for case in self.cases.iter_mut() {
            visitor(&mut case.0);
        }
    }
    fn visit_child_stmts<S: FnMut(&Stmt)>(&self, mut visitor: S) {
        for case in self.cases.iter() {
            for stmt in case.1.iter() {
                visitor(stmt);
            }
        }
        if let Some(default) = self.default_case.as_ref() {
            for stmt in default.iter() {
                visitor(stmt);
            }
        }
    }
    fn visit_child_stmts_mut<S: FnMut(&mut Stmt)>(&mut self, mut visitor: S) {
        for case in self.cases.iter_mut() {
            for stmt in case.1.iter_mut() {
                visitor(stmt);
            }
        }
        if let Some(default) = self.default_case.as_mut() {
            for stmt in default.iter_mut() {
                visitor(stmt);
            }
        }
    }
}

/// Representation of a single switch case in a [GmlSwitch].
///
/// FIXME: Case bodies are currently a Vec<Stmt> which is essentially
/// a [Stmt::Block]. I originally chose to do this because I was concerned
/// that lints would want to safely assume that a `Block` meant something with
/// curly braces, but I now believe that `Blocks` should instead be better
/// equipped to express whether or not they contain curly braces, and that case
/// bodies should be made into `Block`s. While its not of huge concern right
/// now, it will be an issue when static analyisis is added, as case bodies won't
/// properly create a new scope.
#[derive(Debug, PartialEq, Clone)]
pub struct SwitchCase(Expr, Vec<Stmt>);
impl SwitchCase {
    /// Creates a new GmlSwitchCase with the given identity and body.
    pub fn new(identity: Expr, body: Vec<Stmt>) -> Self {
        Self(identity, body)
    }

    /// Returns a reference to the case's identity.
    pub fn identity(&self) -> &Expr {
        &self.0
    }

    /// Returns an iterator over the body of the case.
    ///
    /// NOTE: In the future, case bodies may be changed into a
    /// `Stmt::Block`. When this happens, this function may be changed or
    /// removed.
    pub fn iter_body_statements(&self) -> impl Iterator<Item = &Stmt> {
        self.1.iter()
    }
}
