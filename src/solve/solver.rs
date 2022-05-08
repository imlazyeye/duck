use super::*;
use crate::{
    duck_error_unwrapped,
    parse::{ExprId, Identifier},
    FileId,
};
use codespan_reporting::diagnostic::Diagnostic;
use hashbrown::HashMap;

pub struct Session<'s> {
    pub subs: &'s mut Subs,
    identity: Vec<Var>,
    local: Vec<Var>,
}

// Misc
impl<'s> Session<'s> {
    pub fn new(subs: &'s mut Subs) -> Self {
        let mut session = Self {
            subs,
            identity: vec![],
            local: vec![],
        };
        session.subs.0.insert(Var::GlobalAdt, global_adt());
        session.identity.push(Var::GlobalAdt);
        session.enter_new_local(vec![]);
        session
    }

    pub fn resolve_name(&self, name: &str) -> Result<Ty, TypeError> {
        self.local()
            .ty(name)
            .or_else(|| self.adt(&Var::GlobalAdt).ty(name))
            .or_else(|| self.identity().ty(name))
            .map(|v| {
                let mut ty = v.clone();
                ty.normalize(self);
                ty
            })
            .ok_or_else(|| duck_error_unwrapped!("could not find a value for `{name}`"))
    }

    pub fn get_normalized_mut(&mut self, mut var: Var) -> Option<&mut Ty> {
        while let Some(ty) = self.subs.get(&var) {
            if let Ty::Var(v) = ty {
                var = *v
            } else {
                return self.subs.get_mut(&var);
            }
        }
        None
    }
}

// Stack
impl<'s> Session<'s> {
    pub fn enter_new_identity(&mut self, fields: Vec<(Identifier, Ty)>) -> Var {
        let var = Var::Generated(rand::random());
        let adt = Adt::new(AdtState::Extendable, fields);
        self.subs.0.insert(var, Ty::Adt(adt));
        self.push_identity(var);
        var
    }

    pub fn enter_new_local(&mut self, fields: Vec<(Identifier, Ty)>) -> Var {
        let var = Var::Generated(rand::random());
        let adt = Adt::new(AdtState::Extendable, fields);
        self.subs.0.insert(var, Ty::Adt(adt));
        self.push_local(var);
        var
    }

    pub fn push_identity(&mut self, id: Var) {
        self.identity.push(id);
    }

    pub fn push_local(&mut self, id: Var) {
        self.local.push(id);
    }

    pub fn pop_identity(&mut self) -> Var {
        let var = self.identity.pop().unwrap();
        assert!(!self.identity.is_empty(), "Cannot depart the root scope!");
        var
    }

    pub fn pop_local(&mut self) -> Var {
        let var = self.local.pop().unwrap();
        assert!(!self.local.is_empty(), "Cannot depart the root scope!");
        var
    }

    pub fn identity_var(&self) -> &Var {
        self.identity.last().unwrap()
    }

    pub fn local_var(&self) -> &Var {
        self.local.last().unwrap()
    }

    pub fn identity(&self) -> &Adt {
        self.adt(self.identity_var())
    }

    pub fn local(&self) -> &Adt {
        self.adt(self.local_var())
    }

    pub fn identity_mut(&mut self) -> &mut Adt {
        let id = *self.identity_var();
        self.adt_mut(&id)
    }

    pub fn local_mut(&mut self) -> &mut Adt {
        let id = *self.local_var();
        self.adt_mut(&id)
    }
}

// Adts
impl<'s> Session<'s> {
    pub fn adt(&self, var: &Var) -> &Adt {
        if let Ty::Var(var) = self.subs.get(var).unwrap().clone() {
            self.adt(&var)
        } else {
            self.subs.get(var).unwrap().adt()
        }
    }

    pub fn adt_mut(&mut self, var: &Var) -> &mut Adt {
        if let Ty::Var(var) = self.subs.get(var).unwrap().clone() {
            self.adt_mut(&var)
        } else {
            self.subs.get_mut(var).unwrap().adt_mut()
        }
    }
}

pub type TypeError = Diagnostic<FileId>;

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
pub enum Var {
    GlobalAdt,
    Return,
    Expr(ExprId),
    Generated(u64),
}

#[derive(Default)]
pub struct Subs(HashMap<Var, Ty>);
impl Subs {
    pub fn register(&mut self, var: Var, mut ty: Ty) -> Result<(), TypeError> {
        if let Some(previous_ty) = &mut self.remove(&var) {
            Unification::unify(previous_ty, &mut ty)?.commit(self)?;
        }
        if !ty.contains(&Ty::Var(var)) {
            println!("{}", Printer::substitution(&var, &ty));
            self.0.insert(var, ty);
        }
        Ok(())
    }

    pub fn remove(&mut self, var: &Var) -> Option<Ty> {
        self.0.remove(var)
    }

    pub fn contains(&self, var: &Var) -> bool {
        self.0.contains_key(var)
    }

    pub fn get(&self, var: &Var) -> Option<&Ty> {
        self.0.get(var)
    }

    pub fn get_mut(&mut self, var: &Var) -> Option<&mut Ty> {
        self.0.get_mut(var)
    }
}

impl<'s> Session<'s> {
    pub fn checkout<R, F: FnOnce(&mut Ty) -> Result<R, TypeError>>(&mut self, var: &Var, f: F) -> Result<R, TypeError> {
        let mut ty = self.subs.remove(var).unwrap_or(Ty::Var(*var));
        ty.normalize(self);
        let result = f(&mut ty);
        self.subs.register(*var, ty)?;
        result
    }
}
