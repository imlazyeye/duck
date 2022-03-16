use crate::parse::Location;

#[derive(Debug, Clone, PartialEq)]
pub struct Scope {
    fields: HashSet<String>,
    bound_type: TypeId,
}
impl Scope {
    pub fn new(bound_type: TypeId) -> Self {
        Self {
            fields: Fields::default(),
            bound_type,
        }
    }

    pub fn declare(&mut self, name: impl Into<String>, type_id: TypeId) {
        self.fields.insert(name.into(), type_id);
    }

    pub fn contains(&self, name: &str, environment: Environment) -> bool {
        self.fields
            .get(name)
            .or_else(move || environment.read_from(&self.bound_type, name))
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Declaration {
    pub location: Location,
}

pub struct Environment {
    types: FnvHashMap<TypeId, Type>,
}
impl Environment {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn declare_for(&mut self, type_id: TypeId, name: impl Into<String>, value_type_id: TypeId) {
        self.types
            .get_mut(&type_id)
            .unwrap()
            .fields_mut()
            .unwrap()
            .insert(name, value_type_id);
    }
    pub fn read_from(&self, type_id: &TypeId, name: &str) -> Option<TypeId> {
        self.types
            .get(type_id)
            .unwrap()
            .fields()
            .and_then(|fields| fields.get(name))
    }
}
impl Default for Environment {
    fn default() -> Self {
        let mut types = FnvHashMap::default();
        types.insert(TypeId::ANY, Type::Any);
        types.insert(TypeId::GLOBAL_SCOPE, Type::GlobalScope(Fields::default()));
        types.insert(TypeId::UNDEFINED, Type::Undefined);
        types.insert(TypeId::REAL, Type::Real);
        types.insert(TypeId::STRING, Type::String);
        types.insert(TypeId::ARRAY, Type::Array);
        Self {
            types: Default::default(),
        }
    }
}

pub enum Type {
    Any,
    Undefined,
    Real,
    String,
    Array,
    GlobalScope(Fields),
    Object(Fields),
    Struct(Fields),
    Enum(Fields),
    Derived(TypeId, String),
}
impl Type {
    pub fn fields(&self) -> Option<&Fields> {
        match self {
            Type::Any | Type::Undefined | Type::Real | Type::String | Type::Array | Type::Derived(..) => None,
            Type::GlobalScope(fields) | Type::Object(fields) | Type::Struct(fields) | Type::Enum(fields) => {
                Some(fields)
            }
        }
    }
    pub fn fields_mut(&mut self) -> Option<&mut Fields> {
        match self {
            Type::Any | Type::Undefined | Type::Real | Type::String | Type::Array | Type::Derived(..) => None,
            Type::GlobalScope(fields) | Type::Object(fields) | Type::Struct(fields) | Type::Enum(fields) => {
                Some(fields)
            }
        }
    }
}
