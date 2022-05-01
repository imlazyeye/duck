use crate::{duck_error, parse::Identifier};

use super::*;
use hashbrown::HashMap;

#[derive(Debug, PartialEq, Clone)]
pub struct Adt {
    pub id: Var,
    pub fields: HashMap<String, Field>,
    pub bounties: HashMap<String, Bounty>,
    pub state: AdtState,
}
impl Adt {
    pub fn new(state: AdtState, fields: Vec<(Identifier, Ty)>) -> Self {
        Self {
            id: Var::Generated(rand::random()),
            fields: fields
                .into_iter()
                .chain([(Identifier::lazy("self"), Ty::Identity)].into_iter())
                .map(|(iden, ty)| {
                    (
                        iden.lexeme,
                        Field {
                            ty,
                            resolved: true,
                            constant: false,
                        },
                    )
                })
                .collect(),
            bounties: HashMap::default(),
            state,
        }
    }

    pub fn contains(&self, key: &str) -> bool {
        self.fields.contains_key(key)
    }

    pub fn get(&self, key: &str) -> Option<&Ty> {
        self.fields.get(key).map(|f| &f.ty)
    }

    pub fn get_mut(&mut self, key: &str) -> Option<&mut Ty> {
        self.fields.get_mut(key).map(|f| &mut f.ty)
    }

    pub fn set_state(&mut self, state: AdtState) {
        self.state = state;
    }

    pub fn write_constant(&mut self, name: &str, ty: Ty) -> Result<FieldUpdate, TypeError> {
        self.update(name, ty, true, true)
    }

    pub fn write(&mut self, name: &str, ty: Ty) -> Result<FieldUpdate, TypeError> {
        self.update(name, ty, true, false)
    }

    pub fn read(&mut self, name: &str, ty: Ty) -> Result<FieldUpdate, TypeError> {
        self.update(name, ty, false, false)
    }

    fn update<'adt>(
        &'adt mut self,
        name: &str,
        ty: Ty,
        resolved: bool,
        constant: bool,
    ) -> Result<FieldUpdate, TypeError> {
        if !self.fields.contains_key(name) {
            if self.state == AdtState::Concrete {
                duck_error!("cannot find a value for `{name}`")
            } else {
                self.fields.insert(name.into(), Field { ty, constant, resolved });
                Ok(FieldUpdate::None)
            }
        } else {
            let field = self.fields.get_mut(name).unwrap();
            // HACK: this is a total bodge but I don't really mind because its us artifically representing a
            // limitation in GML. GameMaker fails to compile if you double-declare a global named function, so
            // we have to as well
            if self.id == Var::GlobalAdt && matches!((&field.ty, &ty), (Ty::Func(_), Ty::Func(_))) {
                return duck_error!("cannot declare a global function more than once");
            }
            if !field.resolved && resolved {
                field.resolved = true;
            }
            Ok(FieldUpdate::Some(field.ty.clone(), ty))
        }
    }
}

pub enum FieldUpdate {
    Some(Ty, Ty),
    None,
}
impl FieldUpdate {
    pub fn commit(mut self, solver: &mut Solver) -> Result<(), TypeError> {
        match &mut self {
            FieldUpdate::Some(lhs, rhs) => solver.unify_tys(lhs, rhs)?,
            FieldUpdate::None => {}
        }
        std::mem::forget(self);
        Ok(())
    }
}
impl Drop for FieldUpdate {
    fn drop(&mut self) {
        if !std::thread::panicking() {
            panic!("Failed to commit a unification request!");
        }
    }
}

// Eventually we will have real ribs, but for now we cheat and use local adtids
pub type Rib = AdtId;

impl Solver {
    pub fn adt(&self, var: &Var) -> &Adt {
        match self.subs.get(var).unwrap() {
            Ty::Adt(adt) => adt,
            _ => panic!("Failed to find an adt at {}", Printer::var(var)),
        }
    }

    pub fn adt_mut(&mut self, var: &Var) -> &mut Adt {
        match self.subs.get_mut(var).unwrap() {
            Ty::Adt(adt) => adt,
            _ => panic!("Failed to find an adt at {}", Printer::var(var)),
        }
    }
}
