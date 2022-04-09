use super::*;
use crate::{duck_error, parse::Location};
use hashbrown::HashMap;

#[derive(Debug, Default, PartialEq, Clone)]
pub struct Record {
    pub fields: HashMap<String, Field>,
    pub state: State,
}
impl Record {
    pub fn inferred() -> Self {
        Self {
            state: State::Inferred,
            ..Default::default()
        }
    }

    pub fn extendable() -> Self {
        Self {
            state: State::Extendable,
            ..Default::default()
        }
    }

    pub fn concrete() -> Self {
        Self {
            state: State::Concrete,
            ..Default::default()
        }
    }

    pub fn contains(&self, key: &str) -> bool {
        self.fields.contains_key(key)
    }

    pub fn get(&self, key: &str) -> Option<&Field> {
        self.fields.get(key)
    }

    pub fn set_state(&mut self, state: State) {
        self.state = state;
    }

    pub fn apply_field(&mut self, name: &str, field: Field) -> Result<FieldOp, TypeError> {
        if let Some(registration) = self.fields.get(name) {
            Ok(FieldOp::Unification {
                previous: registration.ty.clone(),
                new: field.ty,
            })
        } else {
            let can_extend = match self.state {
                State::Inferred => true,
                State::Extendable => field.op == RecordOp::Write,
                State::Concrete => false,
            };
            if can_extend {
                self.fields.insert(name.into(), field);
            } else {
                // TODO: this should be a special record error
                return duck_error!("Attempted to declare `{name}` into the registry after it had been locked.");
            }
            Ok(FieldOp::NewValue)
        }
    }
}
impl From<HashMap<String, Field>> for Record {
    fn from(fields: HashMap<String, Field>) -> Self {
        Self {
            fields,
            state: State::Concrete,
        }
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum State {
    /// A generic record inferred from context.
    Inferred,
    /// A record that can have new fields added to it.
    Extendable,
    /// A record that cannot have new fields added to it.
    Concrete,
}
impl Default for State {
    fn default() -> Self {
        State::Concrete
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Field {
    pub ty: Ty,
    pub location: Location,
    pub op: RecordOp,
}

#[must_use]
#[allow(clippy::large_enum_variant)]
#[derive(Debug, PartialEq, Clone)]
pub enum FieldOp {
    NewValue,
    Unification { previous: Ty, new: Ty },
}
impl FieldOp {
    pub fn apply(mut self, tw: &mut Solver) -> Result<(), TypeError> {
        match &mut self {
            FieldOp::NewValue => {}
            FieldOp::Unification { previous, new } => tw.unify_tys(previous, new)?,
        };
        std::mem::forget(self);
        Ok(())
    }
}
impl std::ops::Drop for FieldOp {
    fn drop(&mut self) {
        if !std::thread::panicking() {
            panic!("Failed to apply FieldOp to a Solver!");
        }
    }
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum RecordOp {
    Read,
    Write,
}
