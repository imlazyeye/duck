use super::Marker;
use crate::{
    parse::{Expr, ExprId, Identifier},
    FileId,
};
use codespan_reporting::diagnostic::{Diagnostic, Label};
use hashbrown::HashMap;

#[derive(Debug, Default)]
pub struct Scope {
    pub fields: HashMap<String, Marker>,
    pub markers: HashMap<ExprId, Marker>,
    pub marker_iter: u64,
    pub file_id: FileId,
}
impl Scope {
    pub fn new(file_id: FileId) -> Self {
        Self {
            file_id,
            ..Default::default()
        }
    }

    pub fn has_field(&self, name: &str) -> bool {
        self.fields.contains_key(name)
    }

    /// ### Errors
    /// Returns an error if the field is not in scope.
    pub fn field_marker(&self, identifier: &Identifier) -> Result<Marker, Diagnostic<FileId>> {
        match self.fields.get(&identifier.lexeme).copied() {
            Some(marker) => Ok(marker),
            None => Err(Diagnostic::error()
                .with_message(format!("Unrecognized variable: {}", identifier.lexeme))
                .with_labels(vec![
                    Label::primary(self.file_id, identifier.span).with_message("not found in current scope"),
                ])),
        }
    }

    pub fn new_field(&mut self, name: impl Into<String>, expr: &Expr) {
        let marker = self.new_marker(expr);
        self.fields.insert(name.into(), marker);
        println!("{marker}: {expr}");
    }

    fn new_marker(&mut self, expr: &Expr) -> Marker {
        let marker = Marker(self.marker_iter);
        self.alias_expr_to_marker(expr, marker);
        self.marker_iter += 1;
        marker
    }

    pub fn alias_expr_to_marker(&mut self, expr: &Expr, marker: Marker) {
        self.markers.insert(expr.id, marker);
    }

    pub(super) fn get_expr_marker(&mut self, expr: &Expr) -> Marker {
        match self.markers.get(&expr.id).copied() {
            Some(marker) => marker,
            None => {
                let marker = self.new_marker(expr);
                self.markers.insert(expr.id, marker);
                println!("{marker}: {expr}");
                marker
            }
        }
    }
}
