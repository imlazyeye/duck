use crate::parsing::Enum;
use std::collections::HashMap;

/// Tracks globally available symbols in a parsed project.
///
/// This is a stand in for proper static-analysis. The GlobalScope is just a
/// means of seeing what global types exist, as that is enough to perform *some*
/// static analysis in lints, such as the `missing_case_member` lint.
///
/// This will be removed in a future version of duck!
#[derive(Debug, Default)]
pub struct GlobalScope {
    enums: HashMap<String, Enum>,
}
impl GlobalScope {
    /// Creates a new, empty GlobalScope.
    pub fn new() -> Self {
        Self::default()
    }

    /// Returns an option to the [Enum] registered under the provided name.
    pub fn find_enum(&self, name: impl Into<String>) -> Option<&Enum> {
        self.enums.get(&name.into())
    }

    /// Drains a [GlobalScopeBuilder] into this GlobalScope. This is used after
    /// the early pass to consolidate all discovered global symbols into one
    /// collection.
    pub fn drain(&mut self, other: GlobalScopeBuilder) {
        self.enums.extend(other.enums.into_iter())
    }
}

/// Used in the early pass to collect global symbols that are discovered. Late
/// drained into a [GlobalScope]. See documentation for [GlobalScope] for more
/// information.
#[derive(Debug, Default)]
pub struct GlobalScopeBuilder {
    enums: HashMap<String, Enum>,
}
impl GlobalScopeBuilder {
    /// Creates a new, empty GlobalScope.
    pub fn new() -> Self {
        Self::default()
    }

    /// Registers a [GmlEnum] to this GlobalScope.
    pub fn register_enum(&mut self, gml_enum: Enum) {
        self.enums.insert(gml_enum.name.to_string(), gml_enum);
    }
}
