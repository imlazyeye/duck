use crate::parse::{
    ExpressionBox, Identifier, IntoStatementBox, OptionalInitilization, ParseVisitor, Statement, StatementBox,
};

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
impl From<Enum> for Statement {
    fn from(e: Enum) -> Self {
        Self::EnumDeclaration(e)
    }
}
impl IntoStatementBox for Enum {}
impl ParseVisitor for Enum {
    fn visit_child_expressions<E: FnMut(&ExpressionBox)>(&self, mut expression_visitor: E) {
        for member in self.members.iter() {
            match member {
                OptionalInitilization::Uninitialized(expression) => expression_visitor(expression),
                OptionalInitilization::Initialized(_) => {}
            }
        }
    }

    fn visit_child_statements<S: FnMut(&StatementBox)>(&self, mut statement_visitor: S) {
        for member in self.members.iter() {
            match member {
                OptionalInitilization::Uninitialized(_) => {}
                OptionalInitilization::Initialized(statement) => statement_visitor(statement),
            }
        }
    }
}
