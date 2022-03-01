use crate::{
    parsing::ExpressionBox,
    prelude::{IntoStatementBox, ParseVisitor, Statement, StatementBox},
};

/// Representation of an enum.
#[derive(Debug, PartialEq, Clone)]
pub struct Enum {
    /// The name of the enum.
    pub name: String,
    /// The EnumMember's this enum contains.
    pub members: Vec<EnumMember>,
}
impl Enum {
    /// Creates a new, empty enum with the given name.
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            members: vec![],
        }
    }

    /// Creates a new enum with the given name and members.
    pub fn new_with_members(name: impl Into<String>, members: Vec<EnumMember>) -> Self {
        Self {
            name: name.into(),
            members,
        }
    }

    /// Creates a new member in this enum with the provided name and optionally
    /// an initilization.
    pub fn register_member(&mut self, name: String, initializer: Option<ExpressionBox>) {
        self.members.push(EnumMember { name, initializer })
    }

    /// Returns an iterator the fully constructed names of each GmlEnumMember in
    /// this enum. For example, if our enum's name is "Foo", and our member
    /// is "Bar", returns "Foo.Bar".
    pub fn iter_constructed_names(&self) -> impl Iterator<Item = String> + '_ {
        self.members.iter().map(|v| format!("{}.{}", self.name, v.name()))
    }
}
impl From<Enum> for Statement {
    fn from(e: Enum) -> Self {
        Statement::EnumDeclaration(e)
    }
}
impl IntoStatementBox for Enum {}
impl ParseVisitor for Enum {
    fn visit_child_expressions<E: FnMut(&ExpressionBox)>(&self, mut expression_visitor: E) {
        self.members
            .iter()
            .flat_map(|member| member.initializer())
            .for_each(|initializer| {
                expression_visitor(initializer);
            })
    }

    fn visit_child_statements<S: FnMut(&StatementBox)>(&self, _statement_visitor: S) {}
}

/// An individual entry into a [Enum].
#[derive(Debug, PartialEq, Clone)]
pub struct EnumMember {
    name: String,
    initializer: Option<ExpressionBox>,
}
impl EnumMember {
    /// Creates a new EnumMember with the given name and optionally an
    /// initializer.
    pub fn new(name: impl Into<String>, initializer: Option<ExpressionBox>) -> Self {
        Self {
            name: name.into(),
            initializer,
        }
    }

    /// Get a reference to the gml enum member's name.
    pub fn name(&self) -> &str {
        self.name.as_ref()
    }

    /// Get a reference to the gml enum member's initializer.
    pub fn initializer(&self) -> Option<&ExpressionBox> {
        self.initializer.as_ref()
    }
}
