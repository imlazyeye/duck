use std::path::{Path, PathBuf};

#[derive(Debug, Default)]
pub struct GmlEnum(String, Vec<GmlEnumMember>);
impl GmlEnum {
    /// Creates a new GmlEnum.
    pub fn new(name: String) -> Self {
        Self(name, vec![])
    }

    /// Adds a new member with the given name to the enum.
    pub fn add_member(&mut self, name: String) {
        self.1.push(GmlEnumMember::new(name));
    }

    /// Returns a reference to the GmlEnumMembers belonging to this enum.
    pub fn members(&self) -> &[GmlEnumMember] {
        &self.1
    }

    /// Returns an iterator the fully constructed names of each GmlEnumMember in this enum.
    /// For example, if our enum's name is "Foo", and our member is "Bar", returns "Foo.Bar".
    pub fn iter_constructed_names(&self) -> impl Iterator<Item = String> + '_ {
        self.1.iter().map(|v| format!("{}.{}", self.0, v.0))
    }

    /// Returns a reference to the enum's name.
    pub fn name(&self) -> &str {
        &self.0
    }
}

#[derive(Debug)]
pub struct GmlEnumMember(String);
impl GmlEnumMember {
    /// Creates a new GmlEnum.
    pub fn new(name: String) -> Self {
        Self(name)
    }
}

#[derive(Debug)]
pub struct GmlSwitchStatement {
    /// The default case of this switch.
    default_case: GmlSwitchStatementDefault,

    /// The individual cases expressed in the switch statement. Ie: `foo` for `case foo:`
    cases: Vec<String>,

    /// The path this switch comes from.
    resource_path: PathBuf,
}
impl GmlSwitchStatement {
    pub fn new(
        default_case: GmlSwitchStatementDefault,
        resource_path: PathBuf,
        cases: Vec<String>,
    ) -> Self {
        Self {
            default_case,
            cases,
            resource_path,
        }
    }

    pub fn add_case(&mut self, case_literal: String) {
        self.cases.push(case_literal);
    }

    /// Get a reference to the gml switch statement's cases.
    pub fn cases(&self) -> &[String] {
        self.cases.as_ref()
    }

    /// Get a reference to the gml switch statement's parent resource name.
    pub fn resource_path(&self) -> &Path {
        &self.resource_path
    }

    /// Get a reference to the gml switch statement's default case.
    pub fn default_case(&self) -> &GmlSwitchStatementDefault {
        &self.default_case
    }
}

#[derive(Debug)]
pub enum GmlSwitchStatementDefault {
    None,
    Some,
    TypeAssert(String),
}
