use super::*;
use crate::{
    adt, array, duck_error,
    parse::{Access, Expr, ExprId, ExprKind, Identifier},
    FileId,
};
use codespan_reporting::diagnostic::Diagnostic;
use hashbrown::HashMap;

#[derive(Debug, Clone)]
pub struct Solver {
    pub subs: HashMap<Var, Ty>,
    pub adts: HashMap<AdtId, Adt>,
    self_stack: Vec<AdtId>,
    local_stack: Vec<AdtId>,
    return_stack: Vec<Var>,
}

// General
impl Solver {
    pub fn find_adt_for_iden(&mut self, iden: &Identifier) -> AdtId {
        if self.get_adt(self.local_id()).contains(&iden.lexeme) {
            self.local_id()
        } else if self.get_adt(AdtId::GLOBAL).contains(&iden.lexeme) {
            AdtId::GLOBAL
        } else {
            self.self_id()
        }
    }

    pub fn expr_to_adt_access<'a>(&mut self, expr: &'a Expr) -> Result<(AdtId, &'a Identifier), TypeError> {
        match expr.kind() {
            ExprKind::Identifier(iden) => Ok((self.find_adt_for_iden(iden), iden)),
            ExprKind::Access(Access::Identity { right }) => Ok((self.self_id(), right)),
            ExprKind::Access(Access::Global { right }) => Ok((self.self_id(), right)),
            ExprKind::Access(Access::Dot { left, right }) => {
                if let Ty::Adt(id) = left.query(self)? {
                    Ok((id, right))
                } else {
                    let id = self.new_adt(AdtState::Inferred, vec![(right.clone(), Ty::Var(expr.var()))]);
                    left.unify(&mut Ty::Adt(id), self)?;
                    Ok((id, right))
                }
            }
            _ => duck_error!("expr does not contain adt"),
        }
    }

    pub fn resolve_name(&mut self, name: &str) -> Result<Ty, TypeError> {
        let mut ty = if let Some(field) = self
            .get_adt(self.local_id())
            .get(name)
            .or_else(|| self.get_adt(AdtId::GLOBAL).get(name))
            .or_else(|| self.get_adt(self.self_id()).get(name))
        {
            field.clone()
        } else {
            return duck_error!("Could not resolve a type for `{name}`");
        };
        self.normalize(&mut ty);
        Ok(ty)
    }

    pub fn emit_uninitialized_variable_errors(&mut self) -> Result<(), Vec<TypeError>> {
        let mut errors = vec![];
        self.adts.iter().for_each(|(_, adt)| {
            adt.fields.iter().filter(|(_, f)| !f.safe).for_each(|(name, _)| {
                errors
                    .push(Diagnostic::error().with_message(format!("cannot find a value for `{name}` in this scope")));
            });
        });
        if errors.is_empty() { Ok(()) } else { Err(errors) }
    }
}

// Adts
impl Solver {
    pub fn new_adt(&mut self, state: AdtState, fields: Vec<(Identifier, Ty)>) -> AdtId {
        let id = AdtId::new();
        self.adts.insert(
            id,
            Adt {
                id,
                fields: fields
                    .into_iter()
                    .map(|(iden, ty)| {
                        (
                            iden.lexeme,
                            Field {
                                ty,
                                safe: true,
                                constant: false,
                            },
                        )
                    })
                    .collect(),
                bounties: HashMap::default(),
                state,
            },
        );
        id
    }

    pub fn get_adt(&self, adt_id: AdtId) -> &Adt {
        self.adts.get(&adt_id).unwrap()
    }

    pub fn get_adt_mut(&mut self, adt_id: AdtId) -> &mut Adt {
        self.adts.get_mut(&adt_id).unwrap()
    }

    pub fn enter_new_object_scope(&mut self) -> AdtId {
        let adt_id = {
            use Ty::*;
            adt!(self => {
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
            })
        };
        self.enter_self_scope(adt_id);
        adt_id
    }

    pub fn enter_self_scope(&mut self, adt_id: AdtId) {
        self.self_stack.push(adt_id);
    }

    pub fn enter_local_scope(&mut self, adt_id: AdtId) {
        self.local_stack.push(adt_id);
    }

    pub fn enter_new_return_body(&mut self) -> Var {
        let var = Var::Generated(rand::random());
        self.subs.insert(var, Ty::Uninitialized);
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

    pub fn retrieve_return_value(&mut self) -> Ty {
        let var = self.return_stack.pop().unwrap_or_else(|| unreachable!());
        self.subs.get(&var).unwrap().clone()
    }

    pub fn self_id(&self) -> AdtId {
        *self.self_stack.last().unwrap()
    }

    pub fn local_id(&self) -> AdtId {
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
            adts: HashMap::default(),
            self_stack: vec![],
            local_stack: vec![],
            return_stack: vec![],
        };
        solver.adts.insert(
            AdtId::GLOBAL,
            Adt {
                id: AdtId::GLOBAL,
                fields: HashMap::default(),
                bounties: HashMap::default(),
                state: AdtState::Extendable,
            },
        );
        solver.self_stack.push(AdtId::GLOBAL);
        let local_scope = solver.new_adt(AdtState::Extendable, vec![]);
        solver.enter_local_scope(local_scope);
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
