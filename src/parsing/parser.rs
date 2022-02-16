use std::{collections::HashMap, path::PathBuf};

use crate::{
    Duck, GmlComment, GmlConstructor, GmlEnum, GmlKeywords, GmlMacro, GmlSwitchStatement,
    GmlSwitchStatementDefault, Lint, LintLevel, LintTag,
};

use super::{lexer::Lexer, token::Token, token_pilot::TokenPilot, utils::ParseError};

pub struct Parser {
    source_code: String,
    tokens: Vec<(usize, Token)>,
    resource_path: PathBuf,
}
impl Parser {
    pub fn new(source_code: String, resource_path: PathBuf) -> Self {
        let tokens = Lexer::new(&source_code).collect::<Vec<(usize, Token)>>();
        Self {
            source_code,
            tokens,
            resource_path,
        }
    }

    fn token_pilot(&self) -> TokenPilot {
        TokenPilot::new(
            // THIS SUCKS
            self.tokens
                .clone()
                .into_iter()
                .filter(|(_, t)| !matches!(t, Token::Comment(_)))
                .collect::<Vec<(usize, Token)>>()
                .into_iter()
                .peekable(),
        )
    }

    fn token_pilot_with_comments_included(&self) -> TokenPilot {
        TokenPilot::new(self.tokens.clone().into_iter().peekable())
    }

    pub fn collect_lint_tags(
        &mut self,
    ) -> Result<HashMap<(String, usize), LintTag>, ParseError> {
        let mut collection = HashMap::new();
        let mut pilot = self.token_pilot();
        while let Ok(token) = pilot.take() {
            if let Token::LintTag(tag) = &token {
                let level = LintLevel::from_str(tag).ok_or_else(|| {
                    ParseError::InvalidLintLevel(pilot.cursor(), tag.to_string())
                })?;
                pilot.require(Token::LeftParenthesis)?;
                let lint_name = pilot.require_identifier()?;
                let lint = Lint::from_str(&lint_name).ok_or_else(|| {
                    ParseError::InvalidLint(pilot.cursor(), lint_name.to_string())
                })?;
                pilot.require(Token::RightParenthesis)?;
                pilot.require(Token::RightSquareBracket)?;
                let position = Duck::create_file_position_string(
                    &self.source_code,
                    self.resource_path.to_str().unwrap(),
                    pilot.cursor(),
                );

                // Register this tag for the line BELOW this line...
                collection.insert(
                    (position.file_name, position.line + 1),
                    LintTag(lint, level),
                );
            }
        }
        Ok(collection)
    }

    pub fn collect_gml_switch_statements(
        &mut self,
    ) -> Result<Vec<GmlSwitchStatement>, ParseError> {
        let mut collection = vec![];
        let mut pilot = self.token_pilot();
        while let Ok(token) = pilot.take() {
            if token == Token::Switch {
                // Get the position
                let switch_position = Duck::create_file_position_string(
                    &self.source_code,
                    self.resource_path.to_str().unwrap(),
                    pilot.cursor(),
                );

                // Nom nom until we get the right brace
                pilot.take_through(&[Token::LeftBrace])?;

                // We need to keep track of any right braces we encounter so that
                // we can accurately know which left brace is ours
                let mut extra_brace_counter = 0;

                // Now we loop over all our case statements
                let mut cases: Vec<String> = vec![];
                let mut default_case = GmlSwitchStatementDefault::None;
                loop {
                    match pilot.take()? {
                        Token::Case => {
                            // Fetch the thing being matched over...
                            match pilot.take()? {
                                Token::Real(real) => {
                                    cases.push(real.to_string());
                                }
                                Token::StringLiteral(lexeme) => {
                                    cases.push(lexeme.clone());
                                }
                                Token::Identifier(lexeme) => {
                                    // Is it an enum?
                                    if pilot.match_take(Token::Dot).is_some() {
                                        match pilot.take()? {
                                            Token::Identifier(suffix) => {
                                                cases.push(format!("{}.{}", lexeme, suffix));
                                            }
                                            token => {
                                                return Err(ParseError::UnexpectedToken(
                                                    pilot.cursor(),
                                                    token,
                                                ));
                                            }
                                        }
                                    } else {
                                        // Okay it's just some thing
                                        cases.push(lexeme.clone());
                                    }
                                }
                                token => {
                                    return Err(ParseError::UnexpectedToken(
                                        pilot.cursor(),
                                        token,
                                    ));
                                }
                            }

                            // Grab the colon...
                            pilot.require(Token::Colon)?;

                            // Now consume this whole block until we're done with this case...
                            loop {
                                // Breaks and returns don't actually mean much to us here -- they could be internal
                                // ie:
                                // ```gml
                                // switch foo {
                                //    case bar:
                                //        if buzz break;
                                //        // more logic
                                //        break;
                                // }
                                // ```
                                // The only things that actually demonstrate a case ending in GML are
                                // 1. Another case declaration
                                // 2. A default decalaration
                                // 3. A right brace, if it is this switch's right brace
                                match pilot.peek()? {
                                    // Case fall through. Leave these for the next case iteration
                                    Token::Case => break,
                                    Token::Default => break,
                                    // If we find a left brace, take it, and log it so we don't mistake the
                                    // next right brace as our own
                                    Token::LeftBrace => {
                                        pilot.take()?;
                                        extra_brace_counter += 1;
                                    }
                                    // If we find a right brace, check if its ours, and break if it is. Otherwise, eat
                                    Token::RightBrace => {
                                        if extra_brace_counter == 0 {
                                            break;
                                        } else {
                                            extra_brace_counter -= 1;
                                            pilot.take()?;
                                        }
                                    }
                                    // Continue to eat the block
                                    _ => {
                                        pilot.take()?;
                                    }
                                }
                            }
                        }
                        Token::Default => {
                            // Take the colon
                            pilot.require(Token::Colon)?;

                            // Update our default case
                            default_case = GmlSwitchStatementDefault::Some;

                            // Check for a duck style default case. If we don't find it, we continue.
                            if pilot
                                .match_take(Token::Identifier("IMPOSSIBLE".to_string()))
                                .is_some()
                                && pilot.match_take(Token::LeftParenthesis).is_some()
                            {
                                if let Token::StringLiteral(error_message) = pilot.take()? {
                                    let re = regex::Regex::new(r"Unexpected (\w+):").unwrap();
                                    if let Some(capture) = re.captures(&error_message) {
                                        default_case = GmlSwitchStatementDefault::TypeAssert(
                                            capture.get(1).map(|v| v.as_str().to_string()).unwrap(),
                                        );
                                    }
                                }
                            }

                            // Now just keep consuming until we get to the right brace, then leave
                            pilot.take_until(&[Token::RightBrace])?;
                        }
                        Token::RightBrace => {
                            // We are now done. Collect the finished switch!
                            collection.push(GmlSwitchStatement::new(
                                default_case,
                                cases,
                                switch_position,
                            ));
                            break;
                        }
                        token => {
                            return Err(ParseError::UnexpectedToken(pilot.cursor(), token));
                        }
                    }
                }
            }
        }
        Ok(collection)
    }

    pub fn collect_gml_enums(&mut self) -> Result<Vec<GmlEnum>, ParseError> {
        let mut collection = vec![];
        let mut pilot = self.token_pilot();
        while let Ok(token) = pilot.take() {
            if token == Token::Enum {
                match pilot.take()? {
                    Token::Identifier(name) => {
                        let mut gml_enum = GmlEnum::new(
                            name.to_string(),
                            Duck::create_file_position_string(
                                &self.source_code,
                                self.resource_path.to_str().unwrap(),
                                pilot.cursor(),
                            ),
                        );
                        pilot.require(Token::LeftBrace)?;
                        'member_reader: loop {
                            match pilot.take()? {
                                Token::Identifier(name) => {
                                    gml_enum.add_member(name);
                                    // If there's an equal sign, nom nom anything that isn't a comma or right brace
                                    if pilot.match_take(Token::Equals).is_some() {
                                        pilot.take_until(&[Token::Comma, Token::RightBrace])?;
                                    }
                                    // They *might* have a comma, but GML doesn't require trailing commas,
                                    // so this is optional
                                    pilot.match_take(Token::Comma);

                                    // If there's a right brace, we're done!
                                    if pilot.match_take(Token::RightBrace).is_some() {
                                        break 'member_reader;
                                    }
                                }
                                token => {
                                    return Err(ParseError::UnexpectedToken(
                                        pilot.cursor(),
                                        token,
                                    ))
                                }
                            }
                        }
                        collection.push(gml_enum);
                    }
                    token => return Err(ParseError::UnexpectedToken(pilot.cursor(), token)),
                }
            }
        }
        Ok(collection)
    }

    pub fn collect_gml_comments(&mut self) -> Result<Vec<GmlComment>, ParseError> {
        let mut collection = vec![];
        let mut pilot = self.token_pilot_with_comments_included();
        while let Ok(token) = pilot.take() {
            if let Token::Comment(lexeme) = token {
                collection.push(GmlComment::new(
                    lexeme,
                    Duck::create_file_position_string(
                        &self.source_code,
                        self.resource_path.to_str().unwrap(),
                        pilot.cursor(),
                    ),
                ));
            }
        }
        Ok(collection)
    }

    pub fn collect_gml_macros(&mut self) -> Result<Vec<GmlMacro>, ParseError> {
        let mut collection = vec![];
        let mut pilot = self.token_pilot();
        while let Ok(token) = pilot.take() {
            if token == Token::Macro {
                match pilot.take()? {
                    Token::Identifier(name) => {
                        collection.push(GmlMacro::new(
                            name,
                            Duck::create_file_position_string(
                                &self.source_code,
                                self.resource_path.to_str().unwrap(),
                                pilot.cursor(),
                            ),
                        ));
                    }
                    token => return Err(ParseError::UnexpectedToken(pilot.cursor(), token)),
                }
            }
        }
        Ok(collection)
    }

    pub fn collect_gml_constructors(&mut self) -> Result<Vec<GmlConstructor>, ParseError> {
        let mut collection = vec![];
        let mut pilot = self.token_pilot();
        while let Ok(token) = pilot.take() {
            if token == Token::Function {
                // If the next token is a parenthesis, this is anonymous
                let constructor_name = if pilot.match_take(Token::LeftParenthesis).is_some() {
                    None
                } else {
                    // Otherwise, it must be a name
                    let name = match pilot.take()? {
                        Token::Identifier(name) => name,
                        token => {
                            return Err(ParseError::UnexpectedToken(pilot.cursor(), token))
                        }
                    };

                    // Eat the left parenthesis
                    pilot.require(Token::LeftParenthesis)?;
                    Some(name)
                };

                // Om nom nom until the right parenthesis
                loop {
                    if let Token::RightParenthesis = pilot.take()? {
                        // There might be inheritance -- if there is, eat the start of it and continue looping
                        if pilot.match_take(Token::Colon).is_some() {
                            // Okay, eat the name of it
                            match pilot.take()? {
                                Token::Identifier(_) => {}
                                token => {
                                    return Err(ParseError::UnexpectedToken(
                                        pilot.cursor(),
                                        token,
                                    ))
                                }
                            }
                            // Now eat its opening paren...
                            pilot.require(Token::LeftParenthesis)?;

                            // Now we're back where we started -- continue the loop
                            // Note: This allows `foo() : foo() : foo()`, but it's not our job to asser the validity of gml!
                            continue;
                        } else {
                            // Okay, we reached the end -- is `constructor` next?
                            if pilot.match_take(Token::Constructor).is_some() {
                                // This is a constructor!
                                collection.push(GmlConstructor::new(
                                    constructor_name,
                                    Duck::create_file_position_string(
                                        &self.source_code,
                                        self.resource_path.to_str().unwrap(),
                                        pilot.cursor(),
                                    ),
                                ));
                            }
                            break;
                        }
                    }
                }
            }
        }
        Ok(collection)
    }

    pub fn collect_gml_keywords(&mut self) -> Result<Vec<GmlKeywords>, ParseError> {
        let mut collection = vec![];
        let mut pilot = self.token_pilot();
        while let Ok(token) = pilot.take() {
            match token {
                Token::AndKeyword => {
                    collection.push(GmlKeywords::And(Duck::create_file_position_string(
                        &self.source_code,
                        self.resource_path.to_str().unwrap(),
                        pilot.cursor(),
                    )))
                }
                Token::OrKeyword => {
                    collection.push(GmlKeywords::Or(Duck::create_file_position_string(
                        &self.source_code,
                        self.resource_path.to_str().unwrap(),
                        pilot.cursor(),
                    )))
                }
                _ => {}
            }
        }
        Ok(collection)
    }
}
