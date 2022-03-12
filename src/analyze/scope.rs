use crate::parse::Location;
use fnv::FnvHashMap;

#[derive(Debug, Clone, Default, PartialEq)]
pub struct Scope {
    values: FnvHashMap<&'static str, Value>,
    block_to_parent: FnvHashMap<Block, Block>,
}

#[derive(Debug, Clone)]
pub struct ScopeWriter {
    scope: Scope,
    current_block: Block,
}
impl ScopeWriter {
    pub fn new() -> Self {
        Self {
            scope: Scope::default(),
            current_block: Block::new(),
        }
    }
    pub fn declare(&mut self, name: &'static str, ownership: Ownership, location: Location) {
        self.scope.values.insert(
            name,
            Value {
                ownership,
                declaration_location: location,
                block: self.current_block,
            },
        );
    }
    pub fn enter_block(&mut self) {
        let new_block = Block::new();
        self.scope.block_to_parent.insert(new_block, self.current_block);
        self.current_block = new_block;
    }
    pub fn depart_block(&mut self) {
        self.current_block = *self
            .scope
            .block_to_parent
            .get(&self.current_block)
            .expect("Cannot depart the base scope!");
    }
    pub fn snapshot(&self) -> Scope {
        self.scope.clone()
    }
}
impl Default for ScopeWriter {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Value {
    ownership: Ownership,
    declaration_location: Location,
    block: Block,
}

#[derive(Debug, Hash, Clone, Copy, PartialEq, Eq)]
pub struct Block(u64);
impl Block {
    pub fn new() -> Self {
        Self(rand::random())
    }
}
impl Default for Block {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Ownership {
    File,
    Namespace,
    Global,
}
