use crate::parse::Location;
use fnv::FnvHashMap;

#[derive(Debug, Clone, PartialEq)]
pub struct Scope {
    declarations: FnvHashMap<String, Declaration>,
}
impl Scope {
    pub fn declaration(&self, name: &str) -> Option<&Declaration> {
        self.declarations.get(name)
    }
}

#[derive(Debug, Clone)]
pub struct ScopeWriter {
    declarations: FnvHashMap<String, Declaration>,
}
impl ScopeWriter {
    pub fn new() -> Self {
        Self {
            declarations: FnvHashMap::default(),
        }
    }
    pub fn declare(&mut self, name: impl Into<String>, location: Location) {
        self.declarations.insert(name.into(), Declaration { location });
    }
    pub fn contains(&self, name: &str) -> bool {
        self.declarations.contains_key(name)
    }
    pub fn snapshot(&self) -> Scope {
        Scope {
            declarations: self.declarations.clone(),
        }
    }
}
impl Default for ScopeWriter {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Declaration {
    pub location: Location,
}
