use crate::gml::GmlEnum;
use crate::gml::GmlSwitchStatement;
use crate::gml::GmlSwitchStatementDefault;
use std::path::PathBuf;
use std::{
    iter::{Enumerate, Peekable},
    str::Chars,
};

use regex::Regex;

#[derive(Debug, PartialEq)]
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
    Default,
    Comma,
    Equals,
    Identifier(String),
    Real(f32),
    StringLiteral(String),
    Eof,
}

/// Takes a mist source file and converts it into tokens
/// as an iterator.
struct Lexer<'a> {
    input_characters: Peekable<Enumerate<Chars<'a>>>,
    cursor: usize,
}
impl<'a> Lexer<'a> {
    /// Creates a new Lexer, taking a string of mist source.
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

                // Single line comments
                '/' if self.match_take('/') => {
                    while self
                        .peek()
                        .map(|chr| chr != '\r' && chr != '\n')
                        .unwrap_or(false)
                    {
                        self.take();
                    }
                    return self.lex();
                }

                // We currently discard regions and macros, since we have no use for them.
                '#' => {
                    while self
                        .peek()
                        .map(|chr| chr != '\r' && chr != '\n')
                        .unwrap_or(false)
                    {
                        self.take();
                    }
                    return self.lex();
                }

                // Multi line comments
                '/' if self.match_take('*') => {
                    loop {
                        match self.take() {
                            Some((_, chr)) => {
                                if chr == '*' && self.match_take('/') {
                                    break;
                                }
                            }
                            None => return (start_index, Token::Eof),
                        }
                    }
                    return self.lex();
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
                    let mut lexeme = String::from(chr);
                    while let Some(chr) = self.peek() {
                        match chr {
                            '_' | 'A'..='Z' | 'a'..='z' | '0'..='9' => {
                                lexeme.push(self.take().unwrap().1)
                            }
                            _ => {
                                break;
                            }
                        }
                    }

                    // Now let's check for keywords
                    match lexeme.as_ref() {
                        "switch" => Some(Token::Switch),
                        "case" => Some(Token::Case),
                        "break" => Some(Token::Break),
                        "return" => Some(Token::Return),
                        "default" => Some(Token::Default),
                        "enum" => Some(Token::Enum),
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

    /// Returns the next character in the source code.
    fn peek(&mut self) -> Option<char> {
        self.input_characters.peek().map(|(_, chr)| *chr)
    }

    /// Consumes and returns the next character in the source code.
    fn take(&mut self) -> Option<(usize, char)> {
        self.input_characters.next()
    }

    /// Consumes the next character in the source code if it matches the given character.
    fn match_take(&mut self, chr: char) -> bool {
        if self.peek() == Some(chr) {
            self.take();
            return true;
        }
        false
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

pub struct Parser<'a> {
    source_code: String,
    lexer: Peekable<Lexer<'a>>,
    resource_path: PathBuf,
}
impl<'a> Parser<'a> {
    pub fn new(source_code: &'a str, resource_path: PathBuf) -> Self {
        Self {
            source_code: source_code.to_string(),
            lexer: Lexer::new(source_code).peekable(),
            resource_path,
        }
    }

    pub fn collect_gml_switch_statements(
        &mut self,
    ) -> Result<Vec<GmlSwitchStatement>, ClippieParseError> {
        let mut collection = vec![];
        while let Ok(token) = self.take() {
            if token == Token::Switch {
                // Nom nom until we get the right brace
                self.take_through(&[Token::LeftBrace])?;

                // We need to keep track of any right braces we encounter so that
                // we can accurately know which left brace is ours
                let mut extra_brace_counter = 0;

                // Now we loop over all our case statements
                let mut cases: Vec<String> = vec![];
                let mut default_case = GmlSwitchStatementDefault::None;
                loop {
                    match self.take()? {
                        Token::Case => {
                            // Fetch the thing being matched over...
                            match self.take()? {
                                Token::Real(real) => {
                                    cases.push(real.to_string());
                                }
                                Token::StringLiteral(lexeme) => {
                                    cases.push(lexeme.clone());
                                }
                                Token::Identifier(lexeme) => {
                                    // Is it an enum?
                                    if self.match_take(Token::Dot).is_some() {
                                        match self.take()? {
                                            Token::Identifier(suffix) => {
                                                cases.push(format!("{}.{}", lexeme, suffix));
                                            }
                                            token => {
                                                return Err(ClippieParseError::UnexpectedToken(
                                                    token,
                                                    self.create_error_path(),
                                                ))
                                            }
                                        }
                                    } else {
                                        // Okay it's just some thing
                                        cases.push(lexeme.clone());
                                    }
                                }
                                token => {
                                    return Err(ClippieParseError::UnexpectedToken(
                                        token,
                                        self.create_error_path(),
                                    ))
                                }
                            }

                            // Grab the colon...
                            self.require(Token::Colon)?;

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
                                match self.peek()? {
                                    // Case fall through. Leave these for the next case iteration
                                    Token::Case => break,
                                    Token::Default => break,
                                    // If we find a left brace, take it, and log it so we don't mistake the
                                    // next right brace as our own
                                    Token::LeftBrace => {
                                        self.take()?;
                                        extra_brace_counter += 1;
                                    }
                                    // If we find a right brace, check if its ours, and break if it is. Otherwise, eat
                                    Token::RightBrace => {
                                        if extra_brace_counter == 0 {
                                            break;
                                        } else {
                                            extra_brace_counter -= 1;
                                            self.take()?;
                                        }
                                    }
                                    // Continue to eat the block
                                    _ => {
                                        self.take()?;
                                    }
                                }
                            }
                        }
                        Token::Default => {
                            // Take the colon
                            self.require(Token::Colon)?;

                            // Update our default case
                            default_case = GmlSwitchStatementDefault::Some;

                            // Check for a clippie style default case. If we don't find it, we continue.
                            if self
                                .match_take(Token::Identifier("IMPOSSIBLE".to_string()))
                                .is_some()
                                && self.match_take(Token::LeftParenthesis).is_some()
                            {
                                if let Token::StringLiteral(error_message) = self.take()? {
                                    let re = Regex::new(r"Unexpected (\w+):").unwrap();
                                    if let Some(capture) = re.captures(&error_message) {
                                        default_case = GmlSwitchStatementDefault::TypeAssert(
                                            capture.get(1).map(|v| v.as_str().to_string()).unwrap(),
                                        );
                                    }
                                }
                            }

                            // Now just keep consuming until we get to the right brace, then leave
                            self.take_until(&[Token::RightBrace])?;
                        }
                        Token::RightBrace => {
                            // We are now done. Collect the finished switch!
                            collection.push(GmlSwitchStatement::new(
                                default_case,
                                self.resource_path.to_path_buf(),
                                cases,
                            ));
                            break;
                        }
                        token => {
                            return Err(ClippieParseError::UnexpectedToken(
                                token,
                                self.create_error_path(),
                            ))
                        }
                    }
                }
            }
        }
        Ok(collection)
    }

    pub fn collect_gml_enums_from_gml(&mut self) -> Result<Vec<GmlEnum>, ClippieParseError> {
        let mut collection = vec![];
        while let Ok(token) = self.take() {
            if token == Token::Enum {
                match self.take()? {
                    Token::Identifier(name) => {
                        let mut gml_enum = GmlEnum::new(name.to_string());
                        self.require(Token::LeftBrace)?;
                        'member_reader: loop {
                            match self.take()? {
                                Token::Identifier(name) => {
                                    gml_enum.add_member(name);
                                    // If there's an equal sign, nom nom anything that isn't a comma or right brace
                                    if self.match_take(Token::Equals).is_some() {
                                        self.take_through(&[Token::Comma, Token::RightBrace])?;
                                    }
                                    // They *might* have a comma, but GML doesn't require trailing commas,
                                    // so this is optional
                                    self.match_take(Token::Comma);

                                    // If there's a right brace, we're done!
                                    if self.match_take(Token::RightBrace).is_some() {
                                        break 'member_reader;
                                    }
                                }
                                token => {
                                    return Err(ClippieParseError::UnexpectedToken(
                                        token,
                                        self.create_error_path(),
                                    ))
                                }
                            }
                        }
                        collection.push(gml_enum);
                    }
                    token => {
                        return Err(ClippieParseError::UnexpectedToken(
                            token,
                            self.create_error_path(),
                        ))
                    }
                }
            }
        }
        Ok(collection)
    }

    pub fn create_error_path(&mut self) -> String {
        format!(
            "{}:{}:0",
            self.resource_path.to_str().unwrap(),
            self.source_code[..self.lexer.peek().unwrap_or(&(0, Token::Eof)).0]
                .chars()
                .filter(|x| x == &'\n')
                .count()
                + 1
        )
    }

    /// Returns the type of the next Token, or returns an error if there
    /// is none.
    fn peek(&mut self) -> Result<&Token, ClippieParseError> {
        if let Some((_, token)) = self.lexer.peek() {
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
        if let Some((_, found_token)) = self.lexer.next() {
            if found_token == token {
                Ok(found_token)
            } else {
                Err(ClippieParseError::ExpectedTokenType(token))
            }
        } else {
            Err(ClippieParseError::UnexpectedEnd)
        }
    }

    /// Returns the next Token, returning an error if there is none.
    fn take(&mut self) -> Result<Token, ClippieParseError> {
        if let Some((_, token)) = self.lexer.next() {
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

#[derive(Debug, PartialEq)]
pub enum ClippieParseError {
    UnexpectedToken(Token, String),
    ExpectedTokenType(Token),
    UnexpectedEnd,
}
