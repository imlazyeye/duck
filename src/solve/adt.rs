use crate::{duck_error, parse::Identifier};

use super::*;
use hashbrown::HashMap;

#[derive(Debug, PartialEq, Clone)]
pub struct Adt {
    pub id: AdtId,
    pub fields: HashMap<String, Field>,
    pub bounties: HashMap<String, Bounty>,
    pub state: AdtState,
}
impl Adt {
    pub fn contains(&self, key: &str) -> bool {
        self.fields.contains_key(key)
    }

    pub fn get(&self, key: &str) -> Option<&Ty> {
        self.fields.get(key).map(|f| &f.ty)
    }

    pub fn get_mut(&mut self, key: &str) -> Option<&mut Ty> {
        self.fields.get_mut(key).map(|f| &mut f.ty)
    }

    pub fn mark_as_constant(&mut self, key: &str) {
        self.fields.get_mut(key).unwrap().constant = true;
    }

    pub fn set_state(&mut self, state: AdtState) {
        self.state = state;
    }
}

// Eventually we will have real ribs, but for now we cheat and use local adtids
pub type Rib = AdtId;

impl Solver {
    pub fn write_adt(&mut self, adt_id: AdtId, iden: &Identifier, mut ty: Ty) -> Result<(), TypeError> {
        println!("{}", Printer::write(iden, &ty, self));
        if let Some(bounty) = self.get_adt(adt_id).bounties.get(&iden.lexeme).copied() {
            // TODO: it'd be a lot nicer if bounties couldn't survive if the offer was "closed"...
            if !self.get_adt(bounty.offerer).fields.get(&iden.lexeme).unwrap().safe {
                self.maybe_update_adt(bounty.offerer, iden, &mut ty)?;
            }
        }
        if !self.maybe_update_adt(adt_id, iden, &mut ty)? {
            self.declare_into_adt(adt_id, iden, ty, true);
        }
        Ok(())
    }

    pub fn read_adt(&mut self, adt_id: AdtId, iden: &Identifier, mut expected: Ty) -> Result<(), TypeError> {
        if !self.maybe_update_adt(adt_id, iden, &mut expected)? {
            let bounty = Bounty {
                offerer: adt_id,
                origin: self.local_id(),
            };
            if adt_id != self.self_id() {
                self.get_adt_mut(self.self_id())
                    .bounties
                    .insert(iden.lexeme.clone(), bounty);
            }
            if adt_id != AdtId::GLOBAL {
                self.get_adt_mut(AdtId::GLOBAL)
                    .bounties
                    .insert(iden.lexeme.clone(), bounty);
            }
            self.declare_into_adt(adt_id, iden, expected, false);
        }
        Ok(())
    }

    pub fn maybe_update_adt(&mut self, adt_id: AdtId, iden: &Identifier, ty: &mut Ty) -> Result<bool, TypeError> {
        let mut adt = self.get_adt_mut(adt_id).clone(); // todo: horribly hot clone
        let state = adt.state;
        let result = if let Some(field) = adt.fields.get_mut(&iden.lexeme) {
            // HACK: this is a total bodge but I don't really mind because its us artifically representing a
            // limitation in GML. GameMaker fails to compile if you double-declare a global named function, so
            // we have to as well
            if adt_id == AdtId::GLOBAL && matches!((&field.ty, &ty), (Ty::Func(_), Ty::Func(_))) {
                return duck_error!("cannot declare a global function more than once");
            }
            field.safe = true;
            self.unify_tys(&mut field.ty, ty)?;
            self.adts.insert(adt_id, adt); // jesus christ
            Ok(true)
        } else if state == AdtState::Concrete {
            duck_error!("cannot find a value for `{}`", &iden.lexeme)
        } else {
            Ok(false)
        };
        result
    }

    pub fn declare_into_adt(&mut self, adt_id: AdtId, iden: &Identifier, ty: Ty, safe: bool) {
        self.get_adt_mut(adt_id).fields.insert(
            iden.lexeme.clone(),
            Field {
                ty,
                constant: false,
                safe,
            },
        );
    }

    pub fn new_adt(&mut self, state: AdtState, fields: Vec<(Identifier, Ty)>) -> AdtId {
        let id = AdtId::new();
        self.adts.insert(
            id,
            Adt {
                id,
                fields: fields
                    .into_iter()
                    .chain([(Identifier::lazy("self"), Ty::Identity)].into_iter())
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
}
