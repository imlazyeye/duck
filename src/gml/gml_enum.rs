use crate::parsing::expression::ExpressionBox;

#[derive(Debug, PartialEq, Clone)]
pub struct GmlEnum {
    name: String,
    members: Vec<GmlEnumMember>,
}
impl GmlEnum {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            members: vec![],
        }
    }

    pub fn new_with_members(name: impl Into<String>, members: Vec<GmlEnumMember>) -> Self {
        Self {
            name: name.into(),
            members,
        }
    }

    pub fn register_member(&mut self, name: String, initializer: Option<ExpressionBox>) {
        self.members.push(GmlEnumMember { name, initializer })
    }

    /// Returns an iterator the fully constructed names of each GmlEnumMember in this enum.
    /// For example, if our enum's name is "Foo", and our member is "Bar", returns "Foo.Bar".
    pub fn iter_constructed_names(&self) -> impl Iterator<Item = String> + '_ {
        self.members
            .iter()
            .map(|v| format!("{}.{}", self.name, v.name()))
    }

    /// Get a reference to the gml enum's name.
    pub fn name(&self) -> &str {
        self.name.as_ref()
    }

    /// Get a reference to the gml enum's members.
    pub fn members(&self) -> &[GmlEnumMember] {
        self.members.as_ref()
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct GmlEnumMember {
    name: String,
    initializer: Option<ExpressionBox>,
}

impl GmlEnumMember {
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
