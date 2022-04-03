use super::*;
use crate::{
    duck_error,
    parse::{ExprId, Identifier, Location},
};
use hashbrown::HashMap;

#[derive(Debug, PartialEq, Clone)]
pub enum App {
    Array(Box<Term>),
    Record(Record),
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct Record {
    pub fields: HashMap<String, Field>,
    pub locked: bool,
    pub is_writer: bool,
}
impl Record {
    pub fn reader() -> Self {
        Self { ..Default::default() }
    }

    pub fn writer() -> Self {
        Self {
            is_writer: true,
            ..Default::default()
        }
    }

    pub fn contains(&self, key: &str) -> bool {
        self.fields.contains_key(key)
    }

    pub fn get(&self, key: &str) -> Option<&Field> {
        self.fields.get(key)
    }

    pub fn lock(&mut self) {
        self.locked = true;
    }

    pub fn write_field(
        &mut self,
        name: &str,
        expr_id: ExprId,
        location: Location,
        value: Marker,
    ) -> Result<FieldWriteToken, TypeError> {
        let marker = if let Some(registration) = self.fields.get(name) {
            registration.marker
        } else {
            if self.locked {
                return duck_error!("Attempted to declare `{name}` into the registry after it had been locked.");
            }
            let marker = Marker::new(); // todo this will cause issues
            self.fields.insert(
                name.into(),
                Field {
                    expr_id,
                    marker,
                    location,
                },
            );
            marker
        };

        Ok(FieldWriteToken {
            marker,
            value,
            applied: false,
        })
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Field {
    pub expr_id: ExprId,
    pub marker: Marker,
    pub location: Location,
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

#[derive(Debug, PartialEq, Clone)]
pub struct Function {
    pub binding: Option<Marker>,
    pub inheritance: Option<Identifier>,
    pub parameters: Vec<Term>,
    pub return_type: Box<Term>,
}
