use crate::parse::{Expr, ExprType, Identifier, IntoExpr, OptionalInitilization, ParseVisitor, Stmt};

/// Representation of an enum.
#[derive(Debug, PartialEq, Clone)]
pub struct Enum {
    /// The name of the enum.
    pub name: Identifier,
    /// The OptionalInitilization's this enum contains.
    pub members: Vec<OptionalInitilization>,
}
impl Enum {
    /// Creates a new, empty enum with the given name.
    pub fn new(name: Identifier) -> Self {
        Self { name, members: vec![] }
    }

    /// Creates a new enum with the given name and members.
    pub fn new_with_members(name: Identifier, members: Vec<OptionalInitilization>) -> Self {
        Self { name, members }
    }

    /// Returns an iterator the fully constructed names of each GmlEnumMember in
    /// this enum. For example, if our enum's name is "Foo", and our member
    /// is "Bar", returns "Foo.Bar".
    pub fn iter_constructed_names(&self) -> impl Iterator<Item = String> + '_ {
        self.members
            .iter()
            .map(|v| format!("{}.{}", self.name.lexeme, v.name()))
    }
}
impl From<Enum> for ExprType {
    fn from(e: Enum) -> Self {
        Self::Enum(e)
    }
}
impl IntoExpr for Enum {}
impl ParseVisitor for Enum {
    fn visit_child_exprs<E: FnMut(&Expr)>(&self, mut visitor: E) {
        for member in self.members.iter() {
            match member {
                OptionalInitilization::Uninitialized(expr) => visitor(expr),
                OptionalInitilization::Initialized(_) => {}
            }
        }
    }
    fn visit_child_exprs_mut<E: FnMut(&mut Expr)>(&mut self, mut visitor: E) {
        for member in self.members.iter_mut() {
            match member {
                OptionalInitilization::Uninitialized(expr) => visitor(expr),
                OptionalInitilization::Initialized(_) => {}
            }
        }
    }
    fn visit_child_stmts<S: FnMut(&Stmt)>(&self, mut visitor: S) {
        for member in self.members.iter() {
            match member {
                OptionalInitilization::Uninitialized(_) => {}
                OptionalInitilization::Initialized(stmt) => visitor(stmt),
            }
        }
    }
    fn visit_child_stmts_mut<S: FnMut(&mut Stmt)>(&mut self, mut visitor: S) {
        for member in self.members.iter_mut() {
            match member {
                OptionalInitilization::Uninitialized(_) => {}
                OptionalInitilization::Initialized(stmt) => visitor(stmt),
            }
        }
    }
}
