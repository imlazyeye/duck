use crate::parse::{Access, Expr, ExprKind, IntoStmt, ParseVisitor, Stmt, StmtKind};

/// Representation of a gml switch statement.
#[derive(Debug, PartialEq, Clone, serde::Serialize)]
pub struct Switch {
    /// The value this switch statement is matching over.
    pub identity: Expr,
    /// The various cases in this switch statement.
    pub cases: Vec<SwitchCase>,
    /// The default case body, if any.
    pub default_case: Option<Vec<Stmt>>,
}
impl Switch {
    /// Creates a new switch with the provided matching value, cases, and
    /// optionally a default case.
    pub fn new(identity: Expr, cases: Vec<SwitchCase>, default_case: Option<Vec<Stmt>>) -> Self {
        Self {
            identity,
            cases,
            default_case,
        }
    }

    /// Returns if all the cases of this enum are dot-access expressions.
    pub fn all_case_members_dot_access(&self) -> bool {
        !self
            .cases
            .iter()
            .any(|case| !matches!(case.identity().kind(), ExprKind::Access(Access::Dot { .. },)))
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
                .kind()
                .as_dot_access()
                .and_then(|(left, _)| left.as_identifier())
                .map(|iden| iden.lexeme.as_ref())
        })
    }

    /// Get a reference to the switch's matching value. Ie: foo in `switch foo`
    pub fn matching_value(&self) -> &Expr {
        &self.identity
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
impl From<Switch> for StmtKind {
    fn from(switch: Switch) -> Self {
        Self::Switch(switch)
    }
}
impl IntoStmt for Switch {}
impl ParseVisitor for Switch {
    fn visit_child_exprs<E: FnMut(&Expr)>(&self, mut visitor: E) {
        visitor(&self.identity);
        for case in self.cases.iter() {
            visitor(&case.identity);
        }
    }
    fn visit_child_exprs_mut<E: FnMut(&mut Expr)>(&mut self, mut visitor: E) {
        visitor(&mut self.identity);
        for case in self.cases.iter_mut() {
            visitor(&mut case.identity);
        }
    }
    fn visit_child_stmts<S: FnMut(&Stmt)>(&self, mut visitor: S) {
        for case in self.cases.iter() {
            for stmt in case.body.iter() {
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
            for stmt in case.body.iter_mut() {
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

/// Representation of a single switch case.
#[derive(Debug, PartialEq, Clone, serde::Serialize)]
pub struct SwitchCase {
    identity: Expr,
    body: Vec<Stmt>,
}
impl SwitchCase {
    /// Creates a new GmlSwitchCase with the given identity and body.
    pub fn new(identity: Expr, body: Vec<Stmt>) -> Self {
        Self { identity, body }
    }

    /// Returns a reference to the case's identity.
    pub fn identity(&self) -> &Expr {
        &self.identity
    }

    /// Returns an iterator over the body of the case.
    pub fn iter_body_statements(&self) -> impl Iterator<Item = &Stmt> {
        self.body.iter()
    }
}
