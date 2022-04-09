use super::{Record, Var};
use crate::parse::ExprId;
use hashbrown::HashMap;

#[derive(Debug)]
pub struct Table {
    pub active_self: Record,
    pub locals: Record,
    pub vars: HashMap<ExprId, Var>,
}

// use super::*;
// use crate::{
//     duck_bug, duck_error,
//     parse::{Expr, ExprId, Identifier, Location},
//     FileId,
// };
// use codespan_reporting::diagnostic::{Diagnostic, Label};
// use hashbrown::HashMap;

// #[derive(Debug, PartialEq, Clone)]
// pub struct Scope {
//     pub self_var: Var,
//     local: HashMap<String, (ExprId, Location)>,
//     vars: HashMap<ExprId, Var>,
// }
// impl Scope {
//     pub fn new_child(&self) -> Self {
//         Self {
//             self_var: self.self_var,
//             local: Default::default(),
//             vars: Default::default(),
//         }
//     }

//     pub fn new_inferred(solver: &mut Solver) -> Self {
//         Self::new(solver, Object::Inferred(HashMap::default()))
//     }

//     pub fn new_concrete(solver: &mut Solver) -> Self {
//         Self::new(solver, Object::Concrete(HashMap::default()))
//     }

//     pub fn new(solver: &mut Solver, object: Object) -> Self {
//         let self_var = Var::new();
//         solver
//             .new_substitution(self_var, Ty::App(App::Record(object)))
//             .unwrap();
//         Self {
//             self_var,
//             local: Default::default(),
//             vars: Default::default(),
//         }
//     }

//     pub fn has_local_field(&self, name: &str) -> bool {
//         self.local.contains_key(name)
//     }

//     /// ### Errors
//     /// Returns an error if the field is not in scope.
//     pub fn lookup_type(&self, identifier: &Identifier, solver: &Solver) -> Result<Type,
// Diagnostic<FileId>> {         self.lookup_ty(identifier, solver).map(|v| v.into())
//     }

//     /// ### Errors
//     /// Returns an error if the field is not in scope.
//     pub fn lookup_ty(&self, identifier: &Identifier, solver: &Solver) -> Result<Ty,
// Diagnostic<FileId>> {         match self
//             .lookup_var(identifier)
//             .and_then(|var| {
//                 solver.find_ty(&var).ok_or_else(|| {
//                     Diagnostic::bug().with_message(format!("Variable has no value: {}",
// identifier.lexeme))                 })
//             })
//             .cloned()
//         {
//             Ok(ty) => Ok(ty),
//             Err(e) => solver
//                 .find_ty(&self.self_var)
//                 .and_then(|ty| ty.as_object().and_then(|obj| obj.get(&identifier.lexeme)))
//                 .cloned()
//                 .ok_or(e),
//         }
//     }

//     /// ### Errors
//     /// Returns an error if the field is not in scope.
//     pub fn lookup_var(&self, identifier: &Identifier) -> Result<Var, Diagnostic<FileId>> {
//         match self
//             .local
//             .get(&identifier.lexeme)
//             .and_then(|(expr_id, _)| self.vars.get(expr_id))
//             .copied()
//         {
//             Some(var) => Ok(var),
//             None => Err(Diagnostic::error()
//                 .with_message(format!("Unrecognized variable: {}", identifier.lexeme))
//                 .with_labels(vec![
//                     Label::primary(0, identifier.span).with_message("not found in current
// scope"),                 ])),
//         }
//     }

//     pub fn declare_local(&mut self, name: String, expr: &Expr) -> Var {
//         assert!(!self.local.contains_key(&name));
//         let var = self.ensure_alias(expr);
//         self.local.insert(name, (expr.id(), expr.location()));
//         var
//     }

//     pub fn local_fields(&self) -> Vec<String> {
//         self.local.iter().map(|(name, _)| name).cloned().collect()
//     }

//     pub fn alias_expr_to(&mut self, expr: &Expr, var: Var) {
//         self.vars.insert(expr.id(), var);
//     }

//     pub fn ensure_alias(&mut self, expr: &Expr) -> Var {
//         if let Some(iden) = expr.inner().as_identifier() {
//             if let Ok(var) = self.lookup_var(iden) {
//                 self.alias_expr_to(expr, var);
//                 return var;
//             }
//         }
//         match self.vars.get(&expr.id()).copied() {
//             Some(var) => var,
//             None => {
//                 let var = Var::new();
//                 self.alias_expr_to(expr, var);
//                 Printer::give_expr_alias(var, expr.to_string());
//                 var
//             }
//         }
//     }
// }
