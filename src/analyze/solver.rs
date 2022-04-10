use super::*;
use crate::{
    array, duck_bug, duck_error,
    parse::{Expr, ExprId},
    record, FileId,
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
                .or_else(|| self.global_scope().get(&iden.lexeme))
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
        let mut ty = match self.subs.get(var).cloned() {
            Some(ty) => ty,
            None => return duck_bug!("Var {} did not have a substitution!", Printer::var(var)),
        };
        self.normalize(&mut ty);
        Ok(ty)
    }

    pub fn resolve_name(&self, name: &str) -> Result<Ty, TypeError> {
        let mut ty = if let Some(field) = self
            .local_scope()
            .get(name)
            .or_else(|| self.global_scope().get(name))
            .or_else(|| self.self_scope().get(name))
        {
            field.ty.clone()
        } else {
            return duck_error!("Could not resolve a type for `{name}`");
        };
        self.normalize(&mut ty);
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
    pub fn global_scope(&self) -> &Record {
        self.subs
            .get(&GLOBAL_SCOPE_VAR)
            .and_then(|ty| match ty {
                Ty::Record(record) => Some(record),
                _ => None,
            })
            .unwrap_or_else(|| unreachable!())
    }

    pub fn global_scope_mut(&mut self) -> &mut Record {
        self.subs
            .get_mut(&GLOBAL_SCOPE_VAR)
            .and_then(|ty| match ty {
                Ty::Record(record) => Some(record),
                _ => None,
            })
            .unwrap_or_else(|| unreachable!())
    }

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

    pub fn enter_new_constructor_scope(&mut self) -> Var {
        let var = Var::Scope(rand::random());
        self.subs.insert(var, Ty::Record(Record::extendable()));
        self.self_stack.push(var);
        var
    }

    pub fn enter_new_object_scope(&mut self) -> Var {
        let record = {
            use Ty::*;
            record!(
                id: Real,
                visible: Bool,
                solid: Bool,
                persistent: Bool,
                depth: Real,
                layer: Real,
                alarm: array!(Real),
                direction: Real,
                friction: Real,
                gravity: Real,
                gravity_direction: Real,
                hspeed: Real,
                vspeed: Real,
                speed: Real,
                xstart: Real,
                ystart: Real,
                x: Real,
                y: Real,
                xprevious: Real,
                yprevious: Real,
                object_index: Real,
                sprite_index: Real,
                sprite_width: Real,
                sprite_height: Real,
                sprite_xoffset: Real,
                sprite_yoffset: Real,
                image_alpha: Real,
                image_angle: Real,
                image_blend: Real,
                image_index: Real,
                image_number: Real,
                image_speed: Real,
                image_xscale: Real,
                image_yscale: Real,
                mask_index: Real,
                bbox_bottom: Real,
                bbox_left: Real,
                bbox_right: Real,
                bbox_top: Real,
                path_index: Real,
                path_position: Real,
                path_positionprevious: Real,
                path_speed: Real,
                path_scale: Real,
                path_orientation: Real,
                path_endaction: Real, // todo: its a collection of constants
                timeline_index: Real,
                timeline_running: Bool,
                timeline_speed: Real,
                timeline_position: Real,
                timeline_loop: Bool,
                in_sequence: Bool,
                sequence_instance: Any, // it's some struct and look I just don't care
                // todo: we don't support the physics system
            )
        };
        let var = Var::Scope(rand::random());
        self.subs.insert(var, record);
        self.self_stack.push(var);
        var
    }

    pub fn enter_new_local_scope(&mut self) -> Var {
        let var = Var::Scope(rand::random());
        self.subs.insert(var, Ty::Record(Record::extendable()));
        self.local_stack.push(var);
        var
    }

    pub fn depart_self_scope(&mut self) {
        self.self_stack.pop().expect("Cannot depart the root scope!");
    }

    pub fn depart_local_scope(&mut self) {
        self.local_stack.pop().expect("Cannot depart the root scope!");
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
        solver.new_substitution(GLOBAL_SCOPE_VAR, Ty::Record(Record::extendable()));
        solver.self_stack.push(GLOBAL_SCOPE_VAR);
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

const GLOBAL_SCOPE_VAR: Var = Var::Scope(u64::MAX);
