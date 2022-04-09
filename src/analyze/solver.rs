use super::*;
use crate::{
    duck_error,
    parse::{Expr, ExprId},
    FileId,
};
use codespan_reporting::diagnostic::Diagnostic;
use hashbrown::HashMap;

#[derive(Debug, Clone)]
pub struct Solver {
    pub subs: HashMap<Var, Ty>,
    self_stack: Vec<Var>,
    local_stack: Vec<Var>,
}

// General
impl Solver {
    pub fn canonize(&self, expr: &Expr) -> Result<Ty, TypeError> {
        if let Some(iden) = expr
            .inner()
            .as_identifier()
            .or_else(|| expr.inner().as_current_access())
        {
            if let Some(field) = self
                .local_scope()
                .get(&iden.lexeme)
                .or_else(|| self.self_scope().get(&iden.lexeme))
            {
                Ok(field.ty.clone())
            } else {
                duck_error!("Unrecognized variable: {iden}")
            }
        } else {
            Ok(Ty::Var(self.var_for_expr(expr)))
        }
    }

    pub fn resolve_var(&self, var: &Var) -> Result<Ty, TypeError> {
        let mut ty = self.subs.get(var).cloned().unwrap();
        self.normalize(&mut ty)?;
        Ok(ty)
    }

    pub fn resolve_name(&self, name: &str) -> Result<Ty, TypeError> {
        let mut ty = if let Some(field) = self.local_scope().get(name).or_else(|| self.self_scope().get(name)) {
            field.ty.clone()
        } else {
            return duck_error!("Could not resolve a type for `{name}`");
        };
        self.normalize(&mut ty)?;
        Ok(ty)
    }

    pub fn var_for_expr(&self, expr: &Expr) -> Var {
        let var = Var::Expr(expr.id());
        Printer::give_expr_alias(var, expr.to_string());
        var
    }
}

// Scope
impl Solver {
    pub fn local_scope(&self) -> &Record {
        self.subs
            .get(&self.current_local_var())
            .and_then(|ty| match ty {
                Ty::Record(record) => Some(record),
                _ => None,
            })
            .unwrap_or_else(|| unreachable!())
    }

    pub fn local_scope_mut(&mut self) -> &mut Record {
        self.subs
            .get_mut(&self.current_local_var())
            .and_then(|ty| match ty {
                Ty::Record(record) => Some(record),
                _ => None,
            })
            .unwrap_or_else(|| unreachable!())
    }

    pub fn self_scope(&self) -> &Record {
        self.subs
            .get(&self.current_self_var())
            .and_then(|ty| match ty {
                Ty::Record(record) => Some(record),
                _ => None,
            })
            .unwrap_or_else(|| unreachable!())
    }

    pub fn self_scope_mut(&mut self) -> &mut Record {
        self.subs
            .get_mut(&self.current_self_var())
            .and_then(|ty| match ty {
                Ty::Record(record) => Some(record),
                _ => None,
            })
            .unwrap_or_else(|| unreachable!())
    }

    pub fn enter_new_self_scope(&mut self) -> Var {
        let var = self.new_scope();
        self.self_stack.push(var);
        var
    }

    pub fn enter_new_local_scope(&mut self) -> Var {
        let var = self.new_scope();
        self.local_stack.push(var);
        var
    }

    pub fn depart_self_scope(&mut self) {
        self.self_stack.pop().expect("Cannot depart the root scope!");
    }

    pub fn depart_local_scope(&mut self) {
        self.local_stack.pop().expect("Cannot depart the root scope!");
    }

    fn new_scope(&mut self) -> Var {
        let var = Var::Scope(rand::random());
        self.subs.insert(var, Ty::Record(Record::extendable()));
        var
    }

    pub fn current_self_var(&self) -> Var {
        *self.self_stack.last().unwrap()
    }

    pub fn current_local_var(&self) -> Var {
        *self.local_stack.last().unwrap()
    }
}

impl Default for Solver {
    fn default() -> Self {
        let mut solver = Self {
            subs: HashMap::default(),
            self_stack: vec![],
            local_stack: vec![],
        };
        solver.enter_new_self_scope();
        solver.enter_new_local_scope();
        solver
    }
}

pub type TypeError = Diagnostic<FileId>;

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
pub enum Var {
    Expr(ExprId),
    Scope(u64),
    Return,
}