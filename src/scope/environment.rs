use crate::gml::GmlEnum;
use hashbrown::HashMap;

#[derive(Debug, PartialEq)]
pub struct GmlEnvironment {
    pub(super) scopes: HashMap<u64, Scope>,
    pub(super) namespaces: HashMap<u64, Namespace>,
    global_id: ScopeId,
}
impl GmlEnvironment {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn new_namespace(&mut self) -> NamespaceId {
        let id = NamespaceId::new();
        self.namespaces.insert(id.0, Namespace::new(id));
        id
    }

    pub fn new_scope(&mut self, upper_scope_id: ScopeId, namespace_id: NamespaceId) -> ScopeId {
        let id = ScopeId::new();
        self.scopes
            .insert(id.0, Scope::new(id, Some(upper_scope_id), namespace_id));
        id
    }

    pub fn drain(&mut self, mut other: Self) {
        // Remove their global scope and drain it into ours.
        let other_global_id = other.global_id().0;
        let other_global = other.scopes.remove(&other_global_id).unwrap();
        self.global_scope_mut().values.extend(other_global.values);

        // Extend the others in!
        self.scopes.extend(other.scopes);
    }

    pub fn get_scope_namespace(&self, scope_id: &ScopeId) -> NamespaceId {
        self.find_scope(scope_id).namespace
    }

    /// Registers a newly encountered global value.
    pub fn register_global_value(&mut self, name: &str, value: GmlValue) {
        self.global_scope_mut().values.insert(name.into(), value);
    }

    /// Registers a local value with the given name.
    pub fn register_local_value(&mut self, scope_id: &ScopeId, name: &str, value: GmlValue) {
        self.find_scope_mut(scope_id).values.insert(name.into(), value);
    }

    /// Registers a namespace value with the given name.
    pub fn register_namespace_value(&mut self, scope_id: &ScopeId, name: &str, value: GmlValue) {
        self.find_scope_mut(scope_id).values.insert(name.into(), value);
    }

    pub fn find_value(&self, scope_id: &ScopeId, name: &str) -> Option<&GmlValue> {
        let mut scope = self.find_scope(scope_id);
        loop {
            if let Some(value) = scope.find(name) {
                return Some(value);
            } else if let Some(upper_scope_id) = scope.upper_scope_id {
                scope = self.find_scope(&upper_scope_id);
            } else {
                return None;
            }
        }
    }
    pub fn global_scope(&self) -> &Scope {
        self.find_scope(&self.global_id)
    }
    pub fn global_scope_mut(&mut self) -> &mut Scope {
        let id = self.global_id;
        self.find_scope_mut(&id)
    }
    fn find_scope(&self, id: &ScopeId) -> &Scope {
        self.scopes
            .get(&id.0)
            .expect("Failed to find an environment with id: {id}")
    }
    fn find_scope_mut(&mut self, id: &ScopeId) -> &mut Scope {
        self.scopes
            .get_mut(&id.0)
            .expect("Failed to find an environment with id: {id}")
    }

    /// Get the gml environment's global id.
    pub fn global_id(&self) -> ScopeId {
        self.global_id
    }
}
impl Default for GmlEnvironment {
    fn default() -> Self {
        let global_id = ScopeId::from(0);
        let global_environment = Scope::new(global_id, None, NamespaceId::from(0));
        Self {
            scopes: HashMap::from([(global_id.0, global_environment)]),
            namespaces: HashMap::new(),
            global_id,
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct Scope {
    pub(super) values: HashMap<String, GmlValue>,
    id: ScopeId,
    namespace: NamespaceId,
    upper_scope_id: Option<ScopeId>,
}
impl Scope {
    pub fn new(id: ScopeId, upper_scope_id: Option<ScopeId>, namespace: NamespaceId) -> Self {
        Self {
            id,
            upper_scope_id,
            namespace,
            values: Default::default(),
        }
    }

    /// Returns an option for a value in the scope with the given name.
    pub fn find(&self, name: &str) -> Option<&GmlValue> {
        self.values.get(name)
    }

    /// Get a reference to the scope's id.
    pub fn id(&self) -> &ScopeId {
        &self.id
    }
}

#[derive(Debug, PartialEq)]
pub struct Namespace {
    pub(super) values: HashMap<String, GmlValue>,
    id: NamespaceId,
}
impl Namespace {
    pub fn new(id: NamespaceId) -> Self {
        Self {
            id,
            values: Default::default(),
        }
    }

    /// Returns an option for a value in the namespace with the given name.
    pub fn find(&self, name: &str) -> Option<&GmlValue> {
        self.values.get(name)
    }

    /// Get a reference to the namespace's id.
    pub fn id(&self) -> &NamespaceId {
        &self.id
    }
}

#[derive(Debug, PartialEq, Default, Clone, Copy)]
pub struct ScopeId(pub u64);
impl ScopeId {
    pub fn new() -> Self {
        Self(fastrand::u64(..))
    }
}
impl From<u64> for ScopeId {
    fn from(u: u64) -> Self {
        Self(u)
    }
}

#[derive(Debug, PartialEq)]
pub enum GmlValue {
    Macro(String),
    GmlEnum(GmlEnum),
    Any,
}

#[derive(Debug, PartialEq, Default, Clone, Copy)]
pub struct NamespaceId(pub u64);
impl NamespaceId {
    pub fn new() -> Self {
        Self(fastrand::u64(..))
    }
}
impl From<u64> for NamespaceId {
    fn from(u: u64) -> Self {
        Self(u)
    }
}
