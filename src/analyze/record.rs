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

    pub fn concrete(fields: Vec<(String, Field)>, solver: &mut Solver) -> Result<Self, TypeError> {
        let mut record = Self::extendable();
        for (name, field) in fields {
            record.apply_field(&name, field)?.commit(solver)?;
        }
        record.state = State::Concrete;
        Ok(record)
    }

    pub fn contains(&self, key: &str) -> bool {
        self.fields.contains_key(key)
    }

    pub fn get(&self, key: &str) -> Option<&Field> {
        self.fields.get(key)
    }

    pub fn get_mut(&mut self, key: &str) -> Option<&mut Field> {
        self.fields.get_mut(key)
    }

    pub fn set_state(&mut self, state: State) {
        self.state = state;
    }

    pub fn apply_field(&mut self, name: &str, mut field: Field) -> Result<FieldOp, TypeError> {
        if let Some(registered_field) = self.fields.get_mut(name) {
            // Ensure the registered field is mutable
            if !registered_field.mutable && field.op == RecordOp::Write {
                return duck_error!("Attempted to write to an immutable value: {name}");
            }

            // If the registered field has a pending promise, we can fulfill it on the following conditions
            // 1. The incoming field is a write operation
            // 2. The incoming field comes from a different origin, UNLESS the incoming field is a constant
            if registered_field.promise_pending
                && field.op == RecordOp::Write
                && (registered_field.origin != field.origin || !field.mutable)
            {
                registered_field.promise_pending = false;
            }

            // If the registration is null, we will just directly override it
            if registered_field.ty == Ty::Null {
                registered_field.ty = field.ty;
                Ok(FieldOp::NewValue)
            } else {
                Ok(FieldOp::Unification {
                    previous: registered_field.ty.clone(),
                    new: field.ty,
                })
            }
        } else {
            match self.state {
                State::Inferred => {
                    self.fields.insert(name.into(), field);
                }
                State::Extendable => {
                    if field.op == RecordOp::Read {
                        field.promise_pending = true;
                    }
                    self.fields.insert(name.into(), field);
                }
                _ => {
                    return Err(codespan_reporting::diagnostic::Diagnostic::error()
                        .with_message(format!(
                            "Attempted to declare {name} into the registry after it had been locked.",
                        ))
                        .with_labels(vec![codespan_reporting::diagnostic::Label::primary(
                            field.location.0,
                            field.location.1,
                        )]));
                }
            };

            // return duck_error!("Attempted to declare `{name}` into the registry after it had
            // been locked.");
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

#[derive(Debug, PartialEq, Clone)]
pub struct Field {
    ty: Ty,
    location: Location,
    op: RecordOp,
    promise_pending: bool,
    origin: Var,
    mutable: bool,
}
impl Field {
    pub fn read(ty: Ty, location: Location, origin: Var) -> Self {
        Self {
            ty,
            location,
            op: RecordOp::Read,
            promise_pending: false,
            origin,
            mutable: true,
        }
    }

    pub fn write(ty: Ty, location: Location, origin: Var) -> Self {
        Self {
            ty,
            location,
            op: RecordOp::Write,
            promise_pending: false,
            origin,
            mutable: true,
        }
    }

    pub fn constant(ty: Ty, location: Location, origin: Var) -> Self {
        Self {
            ty,
            location,
            op: RecordOp::Write,
            promise_pending: false,
            origin,
            mutable: false,
        }
    }

    pub fn promise(ty: Ty, location: Location, origin: Var) -> Self {
        Self {
            ty,
            location,
            op: RecordOp::Write,
            promise_pending: true,
            origin,
            mutable: true,
        }
    }

    /// Get a reference to the field's ty.
    pub fn ty(&self) -> &Ty {
        &self.ty
    }

    /// Get a mutable reference to the field's ty.
    #[must_use]
    pub fn ty_mut(&mut self) -> &mut Ty {
        &mut self.ty
    }

    /// Get the field's promise pending.
    pub fn promise_pending(&self) -> bool {
        self.promise_pending
    }

    /// Get the field's origin.
    pub fn origin(&self) -> Var {
        self.origin
    }
}

#[must_use]
#[allow(clippy::large_enum_variant)]
#[derive(Debug, PartialEq, Clone)]
pub enum FieldOp {
    NewValue,
    Unification { previous: Ty, new: Ty },
}
impl FieldOp {
    pub fn commit(mut self, tw: &mut Solver) -> Result<(), TypeError> {
        let result = match &mut self {
            FieldOp::NewValue => Ok(()),
            FieldOp::Unification { previous, new } => tw.unify_tys(previous, new),
        };
        std::mem::forget(self);
        result
    }
}
impl std::ops::Drop for FieldOp {
    fn drop(&mut self) {
        if !std::thread::panicking() {
            panic!(
                "Failed to commit {} to a Solver!",
                match self {
                    FieldOp::NewValue => "a new value".into(),
                    FieldOp::Unification { previous, new } =>
                        format!("{} ≟ {}", Printer::ty(previous), Printer::ty(new)),
                }
            );
        }
    }
}

#[derive(Debug, PartialEq, Copy, Clone)]
enum RecordOp {
    Read,
    Write,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Promise(Var);

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum State {
    /// A generic recred from context.
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
