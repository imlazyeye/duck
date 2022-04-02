use super::*;
use codespan_reporting::diagnostic::Diagnostic;
use hashbrown::HashMap;

#[derive(Debug, PartialEq, Clone)]
pub enum App {
    Array(Box<Term>),
    Object(Object),
    Function {
        self_fields: Option<Object>,
        parameters: Vec<Term>,
        return_type: Box<Term>,
    },
    Union(Vec<Term>),
}

#[derive(Debug, PartialEq, Clone)]
pub enum Object {
    Concrete(HashMap<String, Term>),
    Inferred(HashMap<String, FieldOp>),
}
impl Object {
    pub fn is_empty(&self) -> bool {
        match self {
            Object::Concrete(fields) => fields.is_empty(),
            Object::Inferred(fields) => fields.is_empty(),
        }
    }

    pub fn get(&self, key: &str) -> Option<&Term> {
        match self {
            Object::Concrete(fields) => fields.get(key),
            Object::Inferred(fields) => fields.get(key).map(|v| v.term()),
        }
    }

    pub fn names(&self) -> Vec<&str> {
        match self {
            Object::Concrete(fields) => fields.iter().map(|(name, _)| name.as_str()).collect(),
            Object::Inferred(fields) => fields.iter().map(|(name, _)| name.as_str()).collect(),
        }
    }

    pub fn fields(&self) -> Vec<(&str, &Term)> {
        match self {
            Object::Concrete(fields) => fields.iter().map(|(name, term)| (name.as_str(), term)).collect(),
            Object::Inferred(fields) => fields.iter().map(|(name, op)| (name.as_str(), op.term())).collect(),
        }
    }

    pub fn as_inferred(&self) -> Option<&HashMap<String, FieldOp>> {
        match self {
            Object::Concrete(_) => None,
            Object::Inferred(fields) => Some(fields),
        }
    }

    pub fn as_inferred_mut(&mut self) -> Option<&mut HashMap<String, FieldOp>> {
        match self {
            Object::Concrete(_) => None,
            Object::Inferred(fields) => Some(fields),
        }
    }

    /// ### Errors
    /// shut up
    pub fn apply_object(
        &mut self,
        object: &mut Object,
        typewriter: &mut Typewriter,
        scope: &mut Scope,
    ) -> Result<(), TypeError> {
        match object {
            Object::Concrete(fields) => {
                for (name, term) in fields {
                    self.apply_field_op(name, &mut FieldOp::Writable(term.clone()), typewriter, scope)?;
                }
            }
            Object::Inferred(fields) => {
                for (name, op) in fields {
                    self.apply_field_op(name, op, typewriter, scope)?;
                }
            }
        }

        Ok(())
    }

    /// ### Errors
    /// shut up
    pub fn apply_field_op(
        &mut self,
        key: &str,
        field_op: &mut FieldOp,
        typewriter: &mut Typewriter,
        scope: &mut Scope,
    ) -> Result<(), TypeError> {
        match self {
            Object::Concrete(fields) => {
                // We'll allow new fields, but anything else must already match.
                // In the future when implementing "purity", we'd deny anything that doesn't match.
                if let Some(field) = fields.get_mut(key) {
                    typewriter.unify_terms(field, field_op.term_mut(), scope)?;
                } else {
                    match field_op {
                        FieldOp::Readable(_) => {
                            return Err(Diagnostic::error().with_message(format!("Missing field in struct: {key}")));
                        }
                        FieldOp::Writable(field) => {
                            fields.insert(key.into(), field.clone());
                        }
                    }
                }
            }
            Object::Inferred(fields) => {
                // This object is inferred, so if the field is not present we'll just add it in
                if let Some(object_field_op) = fields.get_mut(key) {
                    typewriter.unify_terms(object_field_op.term_mut(), field_op.term_mut(), scope)?;
                } else {
                    fields.insert(key.into(), field_op.clone());
                }
            }
        }

        Ok(())
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum FieldOp {
    Readable(Term),
    Writable(Term),
}
impl FieldOp {
    pub fn term(&self) -> &Term {
        match self {
            FieldOp::Readable(term) | FieldOp::Writable(term) => term,
        }
    }

    pub fn term_mut(&mut self) -> &mut Term {
        match self {
            FieldOp::Readable(term) | FieldOp::Writable(term) => term,
        }
    }
}
