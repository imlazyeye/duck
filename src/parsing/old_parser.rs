use std::{collections::HashMap, path::PathBuf};

use crate::{GmlComment, GmlConstructor, GmlEnum, GmlMacro, GmlSwitchStatement, LintTag, Position};

use super::{lexer::Lexer, token::Token, token_pilot::TokenPilot, utils::ParseError};

pub struct OldParser {
    source_code: String,
    tokens: Vec<(usize, Token)>,
    resource_path: PathBuf,
}
impl OldParser {
    pub fn new(source_code: String, resource_path: PathBuf) -> Self {
        let tokens = Lexer::new(&source_code).collect::<Vec<(usize, Token)>>();
        Self {
            source_code,
            tokens,
            resource_path,
        }
    }

    fn token_pilot_with_comments_included(&self) -> TokenPilot {
        todo!()
    }

    pub fn collect_lint_tags(&mut self) -> Result<HashMap<(String, usize), LintTag>, ParseError> {
        todo!()
    }

    pub fn collect_gml_switch_statements(&mut self) -> Result<Vec<GmlSwitchStatement>, ParseError> {
        todo!()
    }

    pub fn collect_gml_enums(&mut self) -> Result<Vec<GmlEnum>, ParseError> {
        todo!()
    }

    pub fn collect_gml_comments(&mut self) -> Result<Vec<GmlComment>, ParseError> {
        todo!()
    }

    pub fn collect_gml_macros(&mut self) -> Result<Vec<GmlMacro>, ParseError> {
        todo!()
    }

    pub fn collect_gml_constructors(&mut self) -> Result<Vec<GmlConstructor>, ParseError> {
        todo!()
    }

    pub fn collect_gml_keywords(&mut self) -> Result<Vec<(Token, Position)>, ParseError> {
        todo!()
    }
}
