use super::*;
use hashbrown::HashMap;

#[derive(Debug, PartialEq, Clone)]
pub struct Adt {
    pub id: AdtId,
    pub fields: HashMap<String, Field>,
    pub state: AdtState,
}
impl Adt {
    pub fn contains(&self, key: &str) -> bool {
        self.fields.contains_key(key)
    }

    pub fn get(&self, key: &str) -> Option<&Field> {
        self.fields.get(key)
    }

    pub fn get_mut(&mut self, key: &str) -> Option<&mut Field> {
        self.fields.get_mut(key)
    }

    pub fn set_state(&mut self, state: AdtState) {
        self.state = state;
    }
}
impl From<HashMap<String, Field>> for Adt {
    fn from(fields: HashMap<String, Field>) -> Self {
        Self {
            id: AdtId::new(),
            fields,
            state: AdtState::Concrete,
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Field {
    pub ty: Ty,
    pub safe: bool,
    pub origin: Rib,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum AdtState {
    /// A generic recred from context.
    Inferred,
    /// A adt that can have new fields added to it.
    Extendable,
    /// A adt that cannot have new fields added to it.
    Concrete,
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
pub struct AdtId(u64);
impl AdtId {
    pub const GLOBAL: Self = Self(u64::MAX);
    pub fn new() -> Self {
        Self(rand::random())
    }
}
impl Default for AdtId {
    fn default() -> Self {
        Self::new()
    }
}

// Eventually we will have real ribs, but for now we cheat and use local adtids
type Rib = AdtId;
