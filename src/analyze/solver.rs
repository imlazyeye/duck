use super::*;
use crate::{
    array, duck_bug, duck_error, duck_error_unwrapped,
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
    return_stack: Vec<Var>,
}

// General
impl Solver {
    pub fn canonize(&mut self, expr: &Expr) -> Result<Ty, TypeError> {
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
                Ok(field.ty().clone())
            } else {
                let var = self.var_for_expr(expr);
                let local_scope_var = self.local_var();
                self.self_scope_mut()
                    .apply_field(
                        &iden.lexeme,
                        Field::promise(Ty::Var(var), expr.location(), local_scope_var),
                    )?
                    .commit(self)?;
                Ok(Ty::Var(var))
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
            field.ty().clone()
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

    pub fn check_promises(&mut self) -> Result<(), Vec<TypeError>> {
        let mut errors: Vec<TypeError> = vec![];
        for record in self
            .subs
            .clone() // UH OH
            .iter_mut()
            .filter_map(|(_, ty)| if let Ty::Record(r) = ty { Some(r) } else { None })
        {
            for (name, field) in record.fields.iter_mut() {
                if field.promise_pending() {
                    // If this promise is still pending, it means the record never recieved a definition. It's
                    // possible that the global scope contains a value though...
                    // lol but what if this is FROM the global scope dummy
                    if let Some(global_field) = self.global_scope().clone().get_mut(&name) {
                        if !global_field.promise_pending() {
                            if let Err(e) = self.unify_tys(field.ty_mut(), global_field.ty_mut()) {
                                errors.push(e);
                            }
                            continue;
                        }
                    }
                    errors.push(duck_error_unwrapped!("Unrecognized variable: {name}"))
                }
            }
        }
        if errors.is_empty() { Ok(()) } else { Err(errors) }
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
            .get(&self.local_var())
            .and_then(|ty| match ty {
                Ty::Record(record) => Some(record),
                _ => None,
            })
            .unwrap_or_else(|| unreachable!())
    }

    pub fn local_scope_mut(&mut self) -> &mut Record {
        self.subs
            .get_mut(&self.local_var())
            .and_then(|ty| match ty {
                Ty::Record(record) => Some(record),
                _ => None,
            })
            .unwrap_or_else(|| unreachable!())
    }

    pub fn self_scope(&self) -> &Record {
        self.subs
            .get(&self.self_var())
            .and_then(|ty| match ty {
                Ty::Record(record) => Some(record),
                _ => None,
            })
            .unwrap_or_else(|| unreachable!())
    }

    pub fn self_scope_mut(&mut self) -> &mut Record {
        self.subs
            .get_mut(&self.self_var())
            .and_then(|ty| match ty {
                Ty::Record(record) => Some(record),
                ty => {
                    println!("foo: {}", Printer::ty(&ty));
                    None
                }
            })
            .unwrap_or_else(|| unreachable!())
    }

    pub fn enter_new_constructor_scope(&mut self) -> Var {
        let var = Var::Generated(rand::random());
        self.subs.insert(var, Ty::Record(Record::extendable()));
        self.enter_self_scope(var);
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
                sequence_instance: Any, /* it's some struct and look I just don't care
                                         * todo: we don't support the physics system */
            )
        };
        let var = Var::Generated(rand::random());
        self.subs.insert(var, record);
        self.enter_self_scope(var);
        var
    }

    pub fn enter_self_scope(&mut self, var: Var) {
        self.self_stack.push(var);
    }

    pub fn enter_new_local_scope(&mut self) -> Var {
        let var = Var::Generated(rand::random());
        self.subs.insert(var, Ty::Record(Record::extendable()));
        self.local_stack.push(var);
        var
    }

    pub fn enter_new_return_body(&mut self) -> Var {
        let var = Var::Generated(rand::random());
        self.subs.insert(var, Ty::Null);
        self.return_stack.push(var);
        var
    }

    pub fn depart_self_scope(&mut self) {
        self.self_stack.pop();
        assert!(!self.self_stack.is_empty(), "Cannot depart the root scope!");
    }

    pub fn depart_local_scope(&mut self) {
        self.local_stack.pop();
        assert!(!self.self_stack.is_empty(), "Cannot depart the root scope!");
    }

    pub fn retrieve_return_value(&mut self) -> Result<Ty, TypeError> {
        let var = self.return_stack.pop().unwrap_or_else(|| unreachable!());
        self.resolve_var(&var)
    }

    pub fn self_var(&self) -> Var {
        *self.self_stack.last().unwrap()
    }

    pub fn local_var(&self) -> Var {
        *self.local_stack.last().unwrap()
    }

    pub fn return_var(&self) -> Var {
        *self.return_stack.last().unwrap()
    }
}

impl Default for Solver {
    fn default() -> Self {
        let mut solver = Self {
            subs: HashMap::default(),
            self_stack: vec![],
            local_stack: vec![],
            return_stack: vec![],
        };
        solver.new_substitution(GLOBAL_SCOPE_VAR, Ty::Record(Record::extendable()));
        solver.self_stack.push(GLOBAL_SCOPE_VAR);
        solver.enter_new_local_scope();
        solver.enter_new_return_body();
        solver
    }
}

pub type TypeError = Diagnostic<FileId>;

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
pub enum Var {
    Expr(ExprId),
    Generated(u64),
}

const GLOBAL_SCOPE_VAR: Var = Var::Generated(u64::MAX);
