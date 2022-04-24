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
            self.maybe_update_adt(bounty.offerer, iden, &mut ty)?;
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
        let mut adt = self.adts.remove(&adt_id).unwrap(); // hack: oh no
        let state = adt.state;
        let result = if let Some(field) = adt.fields.get_mut(&iden.lexeme) {
            field.safe = true;
            self.unify_tys(&mut field.ty, ty)?;
            Ok(true)
        } else if state == AdtState::Concrete {
            duck_error!("No field found for {}", &iden.lexeme)
        } else {
            Ok(false)
        };
        self.adts.insert(adt_id, adt);
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
}
