use super::*;
use crate::{
    parse::{Expr, ExprId, Identifier},
    FileId,
};
use codespan_reporting::diagnostic::{Diagnostic, Label};
use hashbrown::HashMap;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Scope {
    pub self_scope: Option<Box<Scope>>,
    pub fields: HashMap<String, ExprId>,
    pub markers: HashMap<ExprId, Marker>,
    pub expr_strings: HashMap<Marker, String>,
    pub file_id: FileId,
}
impl Scope {
    pub fn new_persistent_scope(file_id: FileId) -> Self {
        Self {
            file_id,
            self_scope: None,
            fields: HashMap::default(),
            markers: HashMap::default(),
            expr_strings: HashMap::default(),
        }
    }

    pub fn new_local_scope(self_scope: Scope, file_id: FileId) -> Self {
        Self {
            file_id,
            self_scope: Some(Box::new(self_scope)),
            fields: HashMap::default(),
            markers: HashMap::default(),
            expr_strings: HashMap::default(),
        }
    }

    pub fn has_field(&self, name: &str) -> bool {
        self.fields.contains_key(name)
    }

    /// ### Errors
    /// Returns an error if the field is not in scope.
    pub fn field_marker(&self, identifier: &Identifier) -> Result<Marker, Diagnostic<FileId>> {
        match self
            .fields
            .get(&identifier.lexeme)
            .and_then(|expr_id| self.markers.get(expr_id))
            .copied()
        {
            Some(marker) => Ok(marker),
            None => Err(Diagnostic::error()
                .with_message(format!("Unrecognized variable: {}", identifier.lexeme))
                .with_labels(vec![
                    Label::primary(self.file_id, identifier.span).with_message("not found in current scope"),
                ])),
        }
    }

    pub fn new_field(&mut self, name: impl Into<String>, expr: &Expr) {
        let marker = Marker::new();
        self.alias_expr_to_marker(expr, marker);
        self.fields.insert(name.into(), expr.id());
    }

    pub fn alias_expr_to_marker(&mut self, expr: &Expr, marker: Marker) {
        self.markers.insert(expr.id(), marker);
        self.expr_strings.insert(marker, expr.to_string());
    }

    pub fn get_expr_marker(&mut self, expr: &Expr) -> Marker {
        match self.markers.get(&expr.id()).copied() {
            Some(marker) => marker,
            None => {
                let marker = Marker::new();
                self.alias_expr_to_marker(expr, marker);
                marker
            }
        }
    }
}
