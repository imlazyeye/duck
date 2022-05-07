use crate::{duck_error, parse::Identifier};

use super::*;
use colored::Colorize;
use hashbrown::HashMap;

#[derive(Debug, PartialEq, Clone)]
pub struct Adt {
    pub fields: HashMap<String, Field>,
    pub bounties: HashMap<String, Bounty>,
    pub state: AdtState,
}
impl Adt {
    pub fn new(state: AdtState, fields: Vec<(Identifier, Ty)>) -> Self {
        Self {
            fields: fields
                .into_iter()
                .chain([(Identifier::lazy("self"), Ty::Identity)].into_iter())
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

    pub fn write_constant(&mut self, name: &str, ty: &Ty) -> Result<FieldUpdate, TypeError> {
        println!(
            "{}        {name}: {}",
            "WRITE".bright_cyan(),
            Printer::ty(ty).blue().bold()
        );
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

    pub fn write(&mut self, name: &str, ty: &Ty) -> Result<FieldUpdate, TypeError> {
        self.update(name, ty, true, false)
    }

    pub fn read(&mut self, name: &str, ty: &Ty) -> Result<FieldUpdate, TypeError> {
        self.update(name, ty, false, false)
    }

    fn update<'adt>(
        &'adt mut self,
        name: &str,
        ty: &Ty,
        resolved: bool,
        constant: bool,
    ) -> Result<FieldUpdate, TypeError> {
        println!(
            "{}       {name}: {}",
            "UPDATE".bright_cyan(),
            Printer::ty(ty).blue().bold()
        );
        if !self.fields.contains_key(name) {
            if self.state == AdtState::Concrete {
                duck_error!("cannot find a value for `{name}`")
            } else {
                self.fields.insert(
                    name.into(),
                    Field {
                        value: FieldValue::Initialized(ty.clone()), // todo: make it a real ref
                        constant,
                        resolved,
                    },
                );
                Ok(FieldUpdate::None)
            }
        } else {
            let field = self.fields.get_mut(name).unwrap();
            // HACK: this is a total bodge but I don't really mind because its us artifically representing a
            // limitation in GML. GameMaker fails to compile if you double-declare a global named function, so
            // we have to as well
            // if self.id == Var::GlobalAdt && matches!((&field.ty, &ty), (Ty::Func(_), Ty::Func(_))) {
            //     return duck_error!("cannot declare a global function more than once");
            // }
            if !field.resolved && resolved {
                field.resolved = true;
            }
            if let FieldValue::Initialized(field_ty) = &field.value {
                Ok(FieldUpdate::Some(field_ty.clone(), ty.clone())) // todo: this too
            } else {
                field.value = FieldValue::Initialized(ty.clone());
                Ok(FieldUpdate::None)
            }
        }
    }
}

#[derive(Debug)]
#[allow(clippy::large_enum_variant)]
pub enum FieldUpdate {
    Some(Ty, Ty),
    None,
}
impl FieldUpdate {
    pub fn commit(mut self, session: &mut Session) -> Result<(), TypeError> {
        match &mut self {
            FieldUpdate::Some(lhs, rhs) => session.unify(lhs, rhs)?.commit(session)?,
            FieldUpdate::None => {}
        }
        std::mem::forget(self);
        Ok(())
    }
}
impl Drop for FieldUpdate {
    fn drop(&mut self) {
        if !std::thread::panicking() {
            panic!("Failed to commit a unification request!");
        }
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
