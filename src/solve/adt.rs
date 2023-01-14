use crate::{duck_error, parse::Identifier};

use super::*;
use colored::Colorize;
use hashbrown::HashMap;

#[derive(Debug, PartialEq, Clone)]
pub struct Adt {
    pub id: AdtId,
    pub fields: HashMap<String, Field>,
    pub bounties: HashMap<String, Bounty>,
    pub state: AdtState,
}
impl Adt {
    pub fn new(state: AdtState, fields: Vec<(Identifier, Ty)>) -> Self {
        Self {
            id: AdtId::new(),
            fields: fields
                .into_iter()
                // .chain([(Identifier::lazy("self"), Ty::Identity)].into_iter())
                .map(|(iden, ty)| {
                    (
                        iden.lexeme,
                        Field {
                            value: FieldValue::Initialized(ty),
                            resolved: true,
                            constant: false,
                        },
                    )
                })
                .collect(),
            bounties: HashMap::default(),
            state,
        }
    }
    pub fn contains(&self, key: &str) -> bool {
        self.fields.contains_key(key)
    }

    pub fn ty(&self, key: &str) -> Option<&Ty> {
        self.fields.get(key).map(|f| &f.value).and_then(|v| v.ty())
    }

    pub fn ty_mut(&mut self, key: &str) -> Option<&mut Ty> {
        self.fields.get_mut(key).map(|f| &mut f.value).and_then(|v| v.ty_mut())
    }

    pub fn set_state(&mut self, state: AdtState) {
        self.state = state;
    }

    pub fn write_constant(&mut self, name: &str, ty: Ty) -> Result<Substitution, TypeError> {
        self.update(name, ty, true, true)
    }

    pub fn write_unitialized(&mut self, name: &str) -> Result<(), TypeError> {
        println!("{}        {name}: <null>", "WRITE".bright_cyan());
        if !self.fields.contains_key(name) && self.state == AdtState::Concrete {
            duck_error!("cannot find a value for `{name}`")
        } else {
            self.fields.insert(
                name.into(),
                Field {
                    value: FieldValue::Uninitialized,
                    constant: false,
                    resolved: true,
                },
            );
            Ok(())
        }
    }

    pub fn write(&mut self, name: &str, ty: Ty) -> Result<Substitution, TypeError> {
        self.update(name, ty, true, false)
    }

    pub fn read(&mut self, name: &str, ty: Ty) -> Result<Substitution, TypeError> {
        self.update(name, ty, false, false)
    }

    fn update(&mut self, name: &str, mut ty: Ty, is_write: bool, constant: bool) -> Result<Substitution, TypeError> {
        println!(
            "{}       {name}: {}",
            "UPDATE".bright_cyan(),
            Printer::ty(&ty).blue().bold()
        );
        if !self.fields.contains_key(name) {
            if self.state == AdtState::Concrete {
                duck_error!("cannot find a value for `{name}`")
            } else {
                self.fields.insert(
                    name.into(),
                    Field {
                        value: FieldValue::Initialized(ty),
                        constant,
                        resolved: is_write,
                    },
                );
                Ok(Substitution::None)
            }
        } else {
            let field = self.fields.get_mut(name).unwrap();
            // HACK: this is a total bodge but I don't really mind because its us artifically representing a
            // limitation in GML. GameMaker fails to compile if you double-declare a global named function, so
            // we have to as well
            // if self.id == Var::GlobalAdt && matches!((&field.ty, &ty), (Ty::Func(_), Ty::Func(_))) {
            //     return duck_error!("cannot declare a global function more than once");
            // }
            if !field.resolved && is_write {
                field.resolved = true;
            }
            if field.value == FieldValue::Uninitialized {
                field.value = FieldValue::Initialized(ty.clone());
                Ok(Substitution::None)
            } else {
                Unification::unify(field.value.ty_mut().unwrap(), &mut ty)
            }
        }
    }
}

/// A unique id that each [Adt] has.
#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy, Default)]
pub struct AdtId(u64);
impl AdtId {
    /// Creates a new, random ExprId.
    pub fn new() -> Self {
        Self(rand::random())
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Field {
    pub value: FieldValue,
    pub resolved: bool,
    pub constant: bool,
}

#[derive(Debug, PartialEq, Clone)]
pub enum FieldValue {
    Uninitialized,
    Initialized(Ty),
}
impl FieldValue {
    pub fn ty(&self) -> Option<&Ty> {
        if let Self::Initialized(ty) = self {
            Some(ty)
        } else {
            None
        }
    }

    pub fn ty_mut(&mut self) -> Option<&mut Ty> {
        if let Self::Initialized(ty) = self {
            Some(ty)
        } else {
            None
        }
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Bounty {
    pub offerer: Var,
    pub origin: Var,
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
