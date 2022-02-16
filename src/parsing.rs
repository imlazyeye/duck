use crate::gml::GmlEnum;
use crate::gml::GmlSwitchStatement;
use crate::gml::GmlSwitchStatementDefault;
use crate::Clippie;
use crate::ClippieIssue;
use crate::ClippieIssueTag;
use crate::ClippieLevel;
use crate::GmlComment;
use crate::GmlConstructor;
use crate::GmlKeywords;
use crate::GmlMacro;
use std::collections::HashMap;
use std::path::PathBuf;
use std::vec::IntoIter;
use std::{
    iter::{Enumerate, Peekable},
    str::Chars,
};

use regex::Regex;

#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    Switch,
    Case,
    Break,
    Return,
    Colon,
    Dot,
    Enum,
    LeftBrace,
    RightBrace,
    LeftParenthesis,
    RightParenthesis,
    LeftSquareBracket,
    RightSquareBracket,
    Default,
    Comma,
    AndKeyword,
    OrKeyword,
    Equals,
    Macro,
    Function,
    Constructor,
    Comment(String),
    Identifier(String),
    Real(f32),
    StringLiteral(String),
    ClippieIssueTag(String),
    Eof,
}

/// Takes a mist source file and converts it into tokens
/// as an iterator.
struct Lexer<'a> {
    input_characters: Peekable<Enumerate<Chars<'a>>>,
    cursor: usize,
}
impl<'a> Lexer<'a> {
    /// Creates a new Lexer, taking a string of gml source.
    pub fn new(content: &'a str) -> Self {
        Lexer {
            input_characters: content.chars().enumerate().peekable(),
            cursor: 0,
        }
    }

    /// Consumes the Lexer's source code until it identifies the next Token.
    fn lex(&mut self) -> (usize, Token) {
        if let Some((start_index, chr)) = self.take() {
            let token_type = match chr {
                // Match single tokens
                ':' => Some(Token::Colon),
                '.' => Some(Token::Dot),
                '{' => Some(Token::LeftBrace),
                '}' => Some(Token::RightBrace),
                '(' => Some(Token::LeftParenthesis),
                ')' => Some(Token::RightParenthesis),
                '[' => Some(Token::LeftSquareBracket),
                ']' => Some(Token::RightSquareBracket),
                ',' => Some(Token::Comma),
                '=' => Some(Token::Equals),
                '"' => {
                    let mut lexeme = String::new();
                    loop {
                        match self.take() {
                            Some((_, chr)) => {
                                if chr == '"' {
                                    break;
                                } else {
                                    lexeme.push(chr);
                                }
                            }
                            None => return (start_index, Token::Eof),
                        }
                    }
                    Some(Token::StringLiteral(lexeme))
                }

                // Regions / Macros
                '#' => {
                    let mut lexeme = chr.into();
                    self.try_construct_word(&mut lexeme);
                    return match lexeme.as_ref() {
                        "#macro" => (start_index, Token::Macro),
                        "#region" => {
                            self.discard_rest_of_line();
                            self.lex()
                        }
                        _ => self.lex(),
                    };
                }

                // Single line comments or issue tags
                '/' if self.match_take('/') => {
                    // Eat up the whitespace first...
                    let mut comment_lexeme = String::from("//");
                    self.consume_whitespace(&mut comment_lexeme);

                    // See if this is an issue tag...
                    if self.match_take('#') && self.match_take('[') {
                        // Looking promising!!
                        let mut lexeme = String::new();
                        self.try_construct_word(&mut lexeme);
                        match lexeme.as_ref() {
                            "allow" | "warn" | "deny" => Some(Token::ClippieIssueTag(lexeme)),
                            _ => return self.lex(),
                        }
                    } else {
                        // It's just a comment -- eat it up
                        self.consume_rest_of_line(&mut comment_lexeme);
                        Some(Token::Comment(comment_lexeme))
                    }
                }

                // Multi line comments
                '/' if self.match_take('*') => {
                    let mut comment_lexeme = String::from("//");
                    loop {
                        match self.take() {
                            Some((_, chr)) => {
                                comment_lexeme.push(chr);
                                if chr == '*' && self.match_take('/') {
                                    break;
                                }
                            }
                            None => return (start_index, Token::Eof),
                        }
                    }
                    Some(Token::Comment(comment_lexeme))
                }

                // Check for whitespace
                id if id.is_whitespace() => return self.lex(),

                // Check for numbers
                '0'..='9' => {
                    let mut lexeme = String::from(chr);
                    while self.peek().map(|chr| chr.is_numeric()).unwrap_or(false) {
                        lexeme.push(self.take().unwrap().1);
                    }
                    // Floats!
                    if self.peek() == Some('.') {
                        lexeme.push('.');
                        while self.peek().map(|chr| chr.is_numeric()).unwrap_or(false) {
                            lexeme.push(self.take().unwrap().1);
                        }
                    }
                    Some(Token::Real(lexeme.parse().unwrap()))
                }

                // Check for keywords / identifiers
                id if id.is_alphabetic() || id == '_' => {
                    let mut lexeme = chr.into();
                    self.try_construct_word(&mut lexeme);

                    // Now let's check for keywords
                    match lexeme.as_ref() {
                        "switch" => Some(Token::Switch),
                        "case" => Some(Token::Case),
                        "break" => Some(Token::Break),
                        "return" => Some(Token::Return),
                        "default" => Some(Token::Default),
                        "enum" => Some(Token::Enum),
                        "and" => Some(Token::AndKeyword),
                        "or" => Some(Token::OrKeyword),
                        "function" => Some(Token::Function),
                        "constructor" => Some(Token::Constructor),
                        _ => Some(Token::Identifier(lexeme)),
                    }
                }

                // Literally anything else!
                _ => None,
            };

            if let Some(token_type) = token_type {
                (start_index, token_type)
            } else {
                self.lex()
            }
        } else {
            (
                0, // a lie, for the good of the people
                Token::Eof,
            )
        }
    }

    /// Consumes the rest of the line into the stirng.
    fn consume_rest_of_line(&mut self, lexeme: &mut String) {
        while self
            .peek()
            .map(|chr| chr != '\r' && chr != '\n')
            .unwrap_or(false)
        {
            lexeme.push(self.take().unwrap().1);
        }
    }

    /// Discards the remainder of the line.
    fn discard_rest_of_line(&mut self) {
        while self
            .peek()
            .map(|chr| chr != '\r' && chr != '\n')
            .unwrap_or(false)
        {
            self.take();
        }
    }

    /// Will keep eating characters into the given string until it reaches a charcter that
    /// can't be used in an identifier.
    fn try_construct_word(&mut self, lexeme: &mut String) {
        while let Some(chr) = self.peek() {
            match chr {
                '_' | 'A'..='Z' | 'a'..='z' | '0'..='9' => {
                    lexeme.push(self.take().unwrap().1);
                }
                _ => break,
            }
        }
    }

    /// Returns the next character in the source code.
    fn peek(&mut self) -> Option<char> {
        self.input_characters.peek().map(|(_, chr)| *chr)
    }

    /// Consumes and returns the next character in the source code.
    fn take(&mut self) -> Option<(usize, char)> {
        self.input_characters.next()
    }

    /// Consumes the next character in the source code if it matches the given character.
    /// Returns if it succeeds.
    fn match_take(&mut self, chr: char) -> bool {
        if self.peek() == Some(chr) {
            self.take();
            return true;
        }
        false
    }

    /// Consumes the next character in the source code if it matches the given character.
    #[allow(dead_code)]
    fn optional_take(&mut self, chr: char) {
        self.match_take(chr);
    }

    /// Consumes all upcoming characters that are whitespace into the string.
    fn consume_whitespace(&mut self, lexeme: &mut String) {
        while self.peek().filter(|c| c.is_whitespace()).is_some() {
            lexeme.push(self.take().unwrap().1);
        }
    }
}

impl<'a> Iterator for Lexer<'a> {
    type Item = (usize, Token);
    /// Returns the next Token in the Lexer.
    fn next(&mut self) -> Option<Self::Item> {
        let (position, token) = self.lex();
        self.cursor = position;
        if token == Token::Eof {
            None
        } else {
            Some((position, token))
        }
    }
}

pub struct GmlTokens {
    tokens: Peekable<IntoIter<(usize, Token)>>,
    cursor: usize,
}
impl GmlTokens {
    pub fn new(tokens: Peekable<IntoIter<(usize, Token)>>) -> Self {
        Self { tokens, cursor: 0 }
    }
}
impl GmlTokens {
    /// Returns the type of the next Token, or returns an error if there
    /// is none.
    fn peek(&mut self) -> Result<&Token, ClippieParseError> {
        if let Some((_, token)) = self.tokens.peek() {
            Ok(token)
        } else {
            Err(ClippieParseError::UnexpectedEnd)
        }
    }

    /// Consumes and returns the next token if it is the given type.
    fn match_take(&mut self, token: Token) -> Option<Token> {
        if self.peek() == Ok(&token) {
            Some(self.take().unwrap())
        } else {
            None
        }
    }

    /// Returns the next Token, returning an error if there is none, or if it is
    /// not of the required type.
    fn require(&mut self, token: Token) -> Result<Token, ClippieParseError> {
        let found_token = self.take()?;
        if found_token == token {
            Ok(found_token)
        } else {
            Err(ClippieParseError::ExpectedToken(token))
        }
    }

    /// Returns the inner field of the next Token, requiring it to be an Identifier.
    fn require_identifier(&mut self) -> Result<String, ClippieParseError> {
        let next = self.take()?;
        if let Token::Identifier(v) = next {
            Ok(v)
        } else {
            Err(ClippieParseError::UnexpectedToken(self.cursor, next))
        }
    }

    /// Returns the next Token, returning an error if there is none.
    fn take(&mut self) -> Result<Token, ClippieParseError> {
        if let Some((position, token)) = self.tokens.next() {
            self.cursor = position;
            Ok(token)
        } else {
            Err(ClippieParseError::UnexpectedEnd)
        }
    }

    /// Takes until it takes a token matching one passed in.
    /// Om nom nom.
    fn take_through(&mut self, ending_tokens: &[Token]) -> Result<Token, ClippieParseError> {
        loop {
            match self.peek()? {
                token if ending_tokens.contains(token) => break self.take(),
                _ => {
                    self.take()?;
                }
            }
        }
    }

    /// Takes until it peeks a token matching one passed in.
    /// Om nom nom.
    fn take_until(&mut self, ending_tokens: &[Token]) -> Result<&Token, ClippieParseError> {
        loop {
            match self.peek()? {
                token if ending_tokens.contains(token) => break self.peek(),
                _ => {
                    self.take()?;
                }
            }
        }
    }
}

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

    pub fn gml_tokens(&self) -> GmlTokens {
        GmlTokens::new(
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

    pub fn gml_tokens_with_comments_included(&self) -> GmlTokens {
        GmlTokens::new(self.tokens.clone().into_iter().peekable())
    }

    pub fn collect_issue_tags(
        &mut self,
    ) -> Result<HashMap<(String, usize), ClippieIssueTag>, ClippieParseError> {
        let mut collection = HashMap::new();
        let mut gml = self.gml_tokens();
        while let Ok(token) = gml.take() {
            if let Token::ClippieIssueTag(tag) = &token {
                let level = ClippieLevel::from_str(tag).ok_or_else(|| {
                    ClippieParseError::InvalidClippieLevel(gml.cursor, tag.to_string())
                })?;
                gml.require(Token::LeftParenthesis)?;
                let issue_name = gml.require_identifier()?;
                let issue = ClippieIssue::from_str(&issue_name).ok_or_else(|| {
                    ClippieParseError::InvalidClippieIssue(gml.cursor, issue_name.to_string())
                })?;
                gml.require(Token::RightParenthesis)?;
                gml.require(Token::RightSquareBracket)?;
                let position = Clippie::create_file_position_string(
                    &self.source_code,
                    self.resource_path.to_str().unwrap(),
                    gml.cursor,
                );

                // Register this tag for the line BELOW this line...
                collection.insert(
                    (position.file_name, position.line + 1),
                    ClippieIssueTag(issue, level),
                );
            }
        }
        Ok(collection)
    }

    pub fn collect_gml_switch_statements(
        &mut self,
    ) -> Result<Vec<GmlSwitchStatement>, ClippieParseError> {
        let mut collection = vec![];
        let mut gml = self.gml_tokens();
        while let Ok(token) = gml.take() {
            if token == Token::Switch {
                // Get the position
                let switch_position = Clippie::create_file_position_string(
                    &self.source_code,
                    self.resource_path.to_str().unwrap(),
                    gml.cursor,
                );

                // Nom nom until we get the right brace
                gml.take_through(&[Token::LeftBrace])?;

                // We need to keep track of any right braces we encounter so that
                // we can accurately know which left brace is ours
                let mut extra_brace_counter = 0;

                // Now we loop over all our case statements
                let mut cases: Vec<String> = vec![];
                let mut default_case = GmlSwitchStatementDefault::None;
                loop {
                    match gml.take()? {
                        Token::Case => {
                            // Fetch the thing being matched over...
                            match gml.take()? {
                                Token::Real(real) => {
                                    cases.push(real.to_string());
                                }
                                Token::StringLiteral(lexeme) => {
                                    cases.push(lexeme.clone());
                                }
                                Token::Identifier(lexeme) => {
                                    // Is it an enum?
                                    if gml.match_take(Token::Dot).is_some() {
                                        match gml.take()? {
                                            Token::Identifier(suffix) => {
                                                cases.push(format!("{}.{}", lexeme, suffix));
                                            }
                                            token => {
                                                return Err(ClippieParseError::UnexpectedToken(
                                                    gml.cursor, token,
                                                ));
                                            }
                                        }
                                    } else {
                                        // Okay it's just some thing
                                        cases.push(lexeme.clone());
                                    }
                                }
                                token => {
                                    return Err(ClippieParseError::UnexpectedToken(
                                        gml.cursor, token,
                                    ));
                                }
                            }

                            // Grab the colon...
                            gml.require(Token::Colon)?;

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
                                match gml.peek()? {
                                    // Case fall through. Leave these for the next case iteration
                                    Token::Case => break,
                                    Token::Default => break,
                                    // If we find a left brace, take it, and log it so we don't mistake the
                                    // next right brace as our own
                                    Token::LeftBrace => {
                                        gml.take()?;
                                        extra_brace_counter += 1;
                                    }
                                    // If we find a right brace, check if its ours, and break if it is. Otherwise, eat
                                    Token::RightBrace => {
                                        if extra_brace_counter == 0 {
                                            break;
                                        } else {
                                            extra_brace_counter -= 1;
                                            gml.take()?;
                                        }
                                    }
                                    // Continue to eat the block
                                    _ => {
                                        gml.take()?;
                                    }
                                }
                            }
                        }
                        Token::Default => {
                            // Take the colon
                            gml.require(Token::Colon)?;

                            // Update our default case
                            default_case = GmlSwitchStatementDefault::Some;

                            // Check for a clippie style default case. If we don't find it, we continue.
                            if gml
                                .match_take(Token::Identifier("IMPOSSIBLE".to_string()))
                                .is_some()
                                && gml.match_take(Token::LeftParenthesis).is_some()
                            {
                                if let Token::StringLiteral(error_message) = gml.take()? {
                                    let re = Regex::new(r"Unexpected (\w+):").unwrap();
                                    if let Some(capture) = re.captures(&error_message) {
                                        default_case = GmlSwitchStatementDefault::TypeAssert(
                                            capture.get(1).map(|v| v.as_str().to_string()).unwrap(),
                                        );
                                    }
                                }
                            }

                            // Now just keep consuming until we get to the right brace, then leave
                            gml.take_until(&[Token::RightBrace])?;
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
                            return Err(ClippieParseError::UnexpectedToken(gml.cursor, token));
                        }
                    }
                }
            }
        }
        Ok(collection)
    }

    pub fn collect_gml_enums(&mut self) -> Result<Vec<GmlEnum>, ClippieParseError> {
        let mut collection = vec![];
        let mut gml = self.gml_tokens();
        while let Ok(token) = gml.take() {
            if token == Token::Enum {
                match gml.take()? {
                    Token::Identifier(name) => {
                        let mut gml_enum = GmlEnum::new(
                            name.to_string(),
                            Clippie::create_file_position_string(
                                &self.source_code,
                                self.resource_path.to_str().unwrap(),
                                gml.cursor,
                            ),
                        );
                        gml.require(Token::LeftBrace)?;
                        'member_reader: loop {
                            match gml.take()? {
                                Token::Identifier(name) => {
                                    gml_enum.add_member(name);
                                    // If there's an equal sign, nom nom anything that isn't a comma or right brace
                                    if gml.match_take(Token::Equals).is_some() {
                                        gml.take_until(&[Token::Comma, Token::RightBrace])?;
                                    }
                                    // They *might* have a comma, but GML doesn't require trailing commas,
                                    // so this is optional
                                    gml.match_take(Token::Comma);

                                    // If there's a right brace, we're done!
                                    if gml.match_take(Token::RightBrace).is_some() {
                                        break 'member_reader;
                                    }
                                }
                                token => {
                                    return Err(ClippieParseError::UnexpectedToken(
                                        gml.cursor, token,
                                    ))
                                }
                            }
                        }
                        collection.push(gml_enum);
                    }
                    token => return Err(ClippieParseError::UnexpectedToken(gml.cursor, token)),
                }
            }
        }
        Ok(collection)
    }

    pub fn collect_gml_comments(&mut self) -> Result<Vec<GmlComment>, ClippieParseError> {
        let mut collection = vec![];
        let mut gml = self.gml_tokens_with_comments_included();
        while let Ok(token) = gml.take() {
            if let Token::Comment(lexeme) = token {
                collection.push(GmlComment::new(
                    lexeme,
                    Clippie::create_file_position_string(
                        &self.source_code,
                        self.resource_path.to_str().unwrap(),
                        gml.cursor,
                    ),
                ));
            }
        }
        Ok(collection)
    }

    pub fn collect_gml_macros(&mut self) -> Result<Vec<GmlMacro>, ClippieParseError> {
        let mut collection = vec![];
        let mut gml = self.gml_tokens();
        while let Ok(token) = gml.take() {
            if token == Token::Macro {
                match gml.take()? {
                    Token::Identifier(name) => {
                        collection.push(GmlMacro::new(
                            name,
                            Clippie::create_file_position_string(
                                &self.source_code,
                                self.resource_path.to_str().unwrap(),
                                gml.cursor,
                            ),
                        ));
                    }
                    token => return Err(ClippieParseError::UnexpectedToken(gml.cursor, token)),
                }
            }
        }
        Ok(collection)
    }

    pub fn collect_gml_constructors(&mut self) -> Result<Vec<GmlConstructor>, ClippieParseError> {
        let mut collection = vec![];
        let mut gml = self.gml_tokens();
        while let Ok(token) = gml.take() {
            if token == Token::Function {
                // If the next token is a parenthesis, this is anonymous
                let constructor_name = if gml.match_take(Token::LeftParenthesis).is_some() {
                    None
                } else {
                    // Otherwise, it must be a name
                    let name = match gml.take()? {
                        Token::Identifier(name) => name,
                        token => return Err(ClippieParseError::UnexpectedToken(gml.cursor, token)),
                    };

                    // Eat the left parenthesis
                    gml.require(Token::LeftParenthesis)?;
                    Some(name)
                };

                // Om nom nom until the right parenthesis
                loop {
                    if let Token::RightParenthesis = gml.take()? {
                        // There might be inheritance -- if there is, eat the start of it and continue looping
                        if gml.match_take(Token::Colon).is_some() {
                            // Okay, eat the name of it
                            match gml.take()? {
                                Token::Identifier(_) => {}
                                token => {
                                    return Err(ClippieParseError::UnexpectedToken(
                                        gml.cursor, token,
                                    ))
                                }
                            }
                            // Now eat its opening paren...
                            gml.require(Token::LeftParenthesis)?;

                            // Now we're back where we started -- continue the loop
                            // Note: This allows `foo() : foo() : foo()`, but it's not our job to asser the validity of gml!
                            continue;
                        } else {
                            // Okay, we reached the end -- is `constructor` next?
                            if gml.match_take(Token::Constructor).is_some() {
                                // This is a constructor!
                                collection.push(GmlConstructor::new(
                                    constructor_name,
                                    Clippie::create_file_position_string(
                                        &self.source_code,
                                        self.resource_path.to_str().unwrap(),
                                        gml.cursor,
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

    pub fn collect_gml_keywords(&mut self) -> Result<Vec<GmlKeywords>, ClippieParseError> {
        let mut collection = vec![];
        let mut gml = self.gml_tokens();
        while let Ok(token) = gml.take() {
            match token {
                Token::AndKeyword => {
                    collection.push(GmlKeywords::And(Clippie::create_file_position_string(
                        &self.source_code,
                        self.resource_path.to_str().unwrap(),
                        gml.cursor,
                    )))
                }
                Token::OrKeyword => {
                    collection.push(GmlKeywords::Or(Clippie::create_file_position_string(
                        &self.source_code,
                        self.resource_path.to_str().unwrap(),
                        gml.cursor,
                    )))
                }
                _ => {}
            }
        }
        Ok(collection)
    }
}

#[derive(Debug, PartialEq)]
pub enum ClippieParseError {
    UnexpectedToken(usize, Token),
    ExpectedToken(Token),
    InvalidClippieLevel(usize, String),
    InvalidClippieIssue(usize, String),
    UnexpectedEnd,
}
