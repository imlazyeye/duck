use super::*;
use crate::{
    parse::{Expr, ExprId, Identifier, Location},
    FileId,
};
use codespan_reporting::diagnostic::{Diagnostic, Label};
use hashbrown::HashMap;

#[derive(Debug, Clone, Default)]
pub struct Fields(HashMap<String, (ExprId, Location)>);
impl std::ops::Deref for Fields {
    type Target = HashMap<String, (ExprId, Location)>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl std::ops::DerefMut for Fields {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[derive(Debug, Clone)]
pub struct Scope {
    pub self_marker: Marker,
    local: Fields,
    markers: HashMap<ExprId, Marker>,
}
impl Scope {
    pub fn new(typewriter: &mut Typewriter) -> Self {
        let self_marker = Marker::new();
        let mut scope = Self {
            self_marker,
            local: Fields::default(),
            markers: HashMap::default(),
        };
        typewriter
            .new_substitution(self_marker, Term::Generic(vec![]), &mut scope)
            .expect("No type error can arrise from declaraing a new self");
        scope
    }

    pub fn has_field(&self, name: &str) -> bool {
        self.has_local_field(name)
    }

    pub fn has_local_field(&self, name: &str) -> bool {
        self.local.contains_key(name)
    }

    /// ### Errors
    /// Returns an error if the field is not in scope.
    pub fn lookup_type(&self, identifier: &Identifier, typewriter: &Typewriter) -> Result<Type, Diagnostic<FileId>> {
        self.lookup_term(identifier, typewriter).map(|v| v.into())
    }

    /// ### Errors
    /// Returns an error if the field is not in scope.
    pub fn lookup_term(&self, identifier: &Identifier, typewriter: &Typewriter) -> Result<Term, Diagnostic<FileId>> {
        match self
            .lookup_marker(identifier)
            .map(|marker| typewriter.find_term(marker))
        {
            Ok(term) => Ok(term),
            Err(e) => {
                let fields = typewriter.scope_self_traits(self);
                fields
                    .iter()
                    .find_map(|trt| match trt {
                        Trait::FieldOp(field_op) => {
                            if identifier.lexeme == field_op.name() {
                                Some(field_op.term().clone())
                            } else {
                                None
                            }
                        }
                        _ => None,
                    })
                    .ok_or(e)
            }
        }
    }

    /// ### Errors
    /// Returns an error if the field is not in scope.
    pub fn lookup_marker(&self, identifier: &Identifier) -> Result<Marker, Diagnostic<FileId>> {
        match self
            .local
            .get(&identifier.lexeme)
            .and_then(|(expr_id, _)| self.markers.get(expr_id))
            .copied()
        {
            Some(marker) => Ok(marker),
            None => Err(Diagnostic::error()
                .with_message(format!("Unrecognized variable: {}", identifier.lexeme))
                .with_labels(vec![
                    Label::primary(0, identifier.span).with_message("not found in current scope"),
                ])),
        }
    }

    pub fn declare_local(&mut self, name: String, expr: &Expr) -> Marker {
        assert!(!self.local.contains_key(&name));
        let marker = self.ensure_alias(expr);
        self.local.insert(name, (expr.id(), expr.location()));
        marker
    }

    pub fn local_fields(&self) -> Vec<String> {
        self.local.iter().map(|(name, _)| name).cloned().collect()
    }

    pub fn alias_expr_to(&mut self, expr: &Expr, marker: Marker) {
        self.markers.insert(expr.id(), marker);
    }

    pub fn ensure_alias(&mut self, expr: &Expr) -> Marker {
        if let Some(iden) = expr.inner().as_identifier() {
            if let Ok(marker) = self.lookup_marker(iden) {
                self.alias_expr_to(expr, marker);
                return marker;
            }
        }
        match self.markers.get(&expr.id()).copied() {
            Some(marker) => marker,
            None => {
                let marker = Marker::new();
                self.alias_expr_to(expr, marker);
                Printer::give_expr_alias(marker, expr.to_string());
                marker
            }
        }
    }
}
