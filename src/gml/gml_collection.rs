use std::collections::HashMap;

use super::GmlEnum;

#[derive(Debug, Default)]
pub struct GmlCollection {
    enums: HashMap<String, GmlEnum>,
}
impl GmlCollection {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn register_enum(&mut self, gml_enum: GmlEnum) {
        self.enums.insert(gml_enum.name().to_string(), gml_enum);
    }

    pub fn find_enum(&self, name: impl Into<String>) -> Option<&GmlEnum> {
        self.enums.get(&name.into())
    }
}
