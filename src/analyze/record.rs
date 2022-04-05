use super::*;
use crate::{
    duck_error,
    parse::{Expr, ExprId, Location},
};
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

    pub fn apply_field(&mut self, name: &str, field: Field, value: Marker) -> Result<FieldWriteToken, TypeError> {
        let marker = if let Some(registration) = self.fields.get(name) {
            registration.marker
        } else {
            let can_extend = match self.state {
                State::Inferred => true,
                State::Extendable => field.op == RecordOp::Write,
                State::Concrete => false,
            };
            if can_extend {
                // let marker = Marker::new();
                self.fields.insert(name.into(), field);
                field.marker
            } else {
                // TODO: this should be a special record error
                return duck_error!("Attempted to declare `{name}` into the registry after it had been locked.");
            }
        };

        Ok(FieldWriteToken {
            marker,
            value,
            applied: false,
        })
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

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Field {
    pub expr_id: ExprId,
    pub marker: Marker,
    pub location: Location,
    pub op: RecordOp,
}
impl Field {
    pub fn new(declaration_expr: &Expr, op: RecordOp) -> Self {
        Self {
            expr_id: declaration_expr.id(),
            marker: Marker::new(),
            location: declaration_expr.location(),
            op,
        }
    }
}

#[must_use]
#[derive(Debug, PartialEq, Clone)]
pub struct FieldWriteToken {
    marker: Marker,
    value: Marker,
    applied: bool,
}
impl FieldWriteToken {
    pub fn apply(mut self, tw: &mut Typewriter) -> Result<Marker, TypeError> {
        self.applied = true;
        tw.unify_marker(&self.marker, &mut Term::Marker(self.value))
            .map(|_| self.marker)
    }
}
impl std::ops::Drop for FieldWriteToken {
    fn drop(&mut self) {
        if !self.applied {
            panic!("Failed to apply FieldWriteToken to a Typewriter!");
        }
    }
}