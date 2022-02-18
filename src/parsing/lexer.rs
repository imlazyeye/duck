use std::{
    iter::{Enumerate, Peekable},
    str::Chars,
};

use super::token::Token;

/// Takes gml  and converts it into tokens as an iterator.
pub(super) struct Lexer<'a> {
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
                '!' => Some(Token::Bang),
                '?' => Some(Token::Interrobang),
                '-' => Some(Token::Minus),
                '+' => Some(Token::Plus),
                '*' => Some(Token::Star),
                '%' => Some(Token::ModSymbol),
                '=' => {
                    if self.match_take('=') {
                        Some(Token::DoubleEquals)
                    } else {
                        Some(Token::Equals)
                    }
                }
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
                '<' => {
                    if self.match_take('=') {
                        Some(Token::LessThanOrEqual)
                    } else {
                        Some(Token::LessThan)
                    }
                }
                '>' => {
                    if self.match_take('=') {
                        Some(Token::GreaterThanOrEqual)
                    } else {
                        Some(Token::GreaterThan)
                    }
                }
                '&' => {
                    if self.match_take('&') {
                        Some(Token::AndSymbol)
                    } else {
                        Some(Token::BitwiseAnd)
                    }
                }
                '|' => {
                    if self.match_take('|') {
                        Some(Token::OrSymbol)
                    } else {
                        Some(Token::BitwiseOr)
                    }
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

                // Comments, lint tags, oh my
                '/' => {
                    if self.match_take('/') {
                        // Eat up the whitespace first...
                        let mut comment_lexeme = String::from("//");
                        self.consume_whitespace(&mut comment_lexeme);

                        // See if this is an lint tag...
                        if self.match_take('#') && self.match_take('[') {
                            // Looking promising!!
                            let mut lexeme = String::new();
                            self.try_construct_word(&mut lexeme);
                            match lexeme.as_ref() {
                                "allow" | "warn" | "deny" => Some(Token::LintTag(lexeme)),
                                _ => return self.lex(),
                            }
                        } else {
                            // It's just a comment -- eat it up
                            self.consume_rest_of_line(&mut comment_lexeme);
                            Some(Token::Comment(comment_lexeme))
                        }
                    } else if self.match_take('*') {
                        // Multi-line comment
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
                    } else {
                        // Just a slash
                        Some(Token::Slash)
                    }
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
                        "exit" => Some(Token::Exit),
                        "new" => Some(Token::New),
                        "global" => Some(Token::Global),
                        "globalvar" => Some(Token::Globalvar),
                        "mod" => Some(Token::ModKeyword),
                        "try" => Some(Token::Try),
                        "with" => Some(Token::With),
                        "true" => Some(Token::True),
                        "false" => Some(Token::False),
                        "div" => Some(Token::Div),
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
