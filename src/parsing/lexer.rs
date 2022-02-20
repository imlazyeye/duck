use std::{
    iter::{Enumerate, Peekable},
    str::Chars,
};

use super::token::Token;

/// Takes gml  and converts it into tokens as an iterator.
pub struct Lexer<'a> {
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
                '{' => Some(Token::LeftBrace),
                '}' => Some(Token::RightBrace),
                '(' => Some(Token::LeftParenthesis),
                ')' => Some(Token::RightParenthesis),
                '[' => Some(Token::LeftSquareBracket),
                ']' => Some(Token::RightSquareBracket),
                ',' => Some(Token::Comma),
                ';' => Some(Token::SemiColon),
                '%' => {
                    if self.match_take('=') {
                        Some(Token::PercentEqual)
                    } else {
                        Some(Token::Percent)
                    }
                }
                '?' => {
                    if self.match_take('?') {
                        if self.match_take('=') {
                            Some(Token::DoubleInterrobangEquals)
                        } else {
                            Some(Token::DoubleInterrobang)
                        }
                    } else {
                        Some(Token::Interrobang)
                    }
                }
                '^' => {
                    if self.match_take('=') {
                        Some(Token::CirumflexEqual)
                    } else {
                        Some(Token::Circumflex)
                    }
                }
                '!' => {
                    if self.match_take('=') {
                        Some(Token::BangEqual)
                    } else {
                        Some(Token::Bang)
                    }
                }
                '+' => {
                    if self.match_take('=') {
                        Some(Token::PlusEqual)
                    } else if self.match_take('+') {
                        Some(Token::DoublePlus)
                    } else {
                        Some(Token::Plus)
                    }
                }
                '-' => {
                    if self.match_take('=') {
                        Some(Token::MinusEqual)
                    } else if self.match_take('-') {
                        Some(Token::DoubleMinus)
                    } else {
                        Some(Token::Minus)
                    }
                }
                '*' => {
                    if self.match_take('=') {
                        Some(Token::StarEqual)
                    } else {
                        Some(Token::Star)
                    }
                }
                '=' => {
                    if self.match_take('=') {
                        Some(Token::DoubleEqual)
                    } else {
                        Some(Token::Equal)
                    }
                }
                '"' => {
                    let mut lexeme = String::new();
                    let mut in_escape = false;
                    loop {
                        match self.take() {
                            Some((_, chr)) => {
                                if in_escape {
                                    lexeme.push(chr);
                                    in_escape = false;
                                } else {
                                    match chr {
                                        '"' if !in_escape => break,
                                        '\\' => {
                                            in_escape = true;
                                        }
                                        _ => lexeme.push(chr),
                                    }
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
                    } else if self.match_take('<') {
                        Some(Token::BitShiftLeft)
                    } else {
                        Some(Token::LessThan)
                    }
                }
                '>' => {
                    if self.match_take('=') {
                        Some(Token::GreaterThanOrEqual)
                    } else if self.match_take('>') {
                        Some(Token::BitShiftRight)
                    } else {
                        Some(Token::GreaterThan)
                    }
                }
                '&' => {
                    if self.match_take('&') {
                        Some(Token::DoubleAmpersand)
                    } else if self.match_take('=') {
                        Some(Token::AmpersandEqual)
                    } else {
                        Some(Token::Ampersand)
                    }
                }
                '|' => {
                    if self.match_take('|') {
                        Some(Token::DoublePipe)
                    } else if self.match_take('=') {
                        Some(Token::PipeEqual)
                    } else {
                        Some(Token::Pipe)
                    }
                }
                '$' => {
                    let mut lexeme = String::new();
                    while let Some(chr) = self.peek() {
                        match chr {
                            'A'..='F' | 'a'..='f' | '0'..='9' => {
                                lexeme.push(self.take().unwrap().1);
                            }
                            _ => break,
                        }
                    }
                    if lexeme.len() == 6 {
                        Some(Token::Hex(lexeme))
                    } else {
                        Some(Token::DollarSign)
                    }
                }

                // Regions / Macros
                '#' => {
                    let mut lexeme = chr.into();
                    self.construct_word(&mut lexeme);
                    return match lexeme.as_ref() {
                        "#macro" => {
                            self.discard_whitespace();
                            let mut iden_one = String::new();
                            self.construct_word(&mut iden_one);
                            let (name, config) = if self.match_take(':') {
                                let mut name = String::new();
                                self.construct_word(&mut name);
                                (name, Some(iden_one))
                            } else {
                                (iden_one, None)
                            };
                            self.discard_whitespace();
                            let mut body = String::new();
                            self.consume_rest_of_line(&mut body);
                            (start_index, Token::Macro(name, config, body))
                        }
                        "#region" | "#endregion" => {
                            self.discard_rest_of_line();
                            self.lex()
                        }
                        "#" => (start_index, Token::Hash),
                        _ => todo!("We don't have a good set up for this error right now!"),
                    };
                }

                // Slashes can be SO MANY THINGS
                '/' => {
                    if self.match_take('=') {
                        Some(Token::SlashEqual)
                    } else if self.match_take('/') {
                        // Eat up the whitespace first...
                        let mut comment_lexeme = String::from("//");
                        self.consume_whitespace_on_line(&mut comment_lexeme);

                        // See if this is an lint tag...
                        if self.match_take('#') && self.match_take('[') {
                            // Looking promising!!
                            let mut lexeme = String::new();
                            self.construct_word(&mut lexeme);
                            match lexeme.as_ref() {
                                "allow" | "warn" | "deny" => Some(Token::LintTag(lexeme)),
                                _ => return self.lex(),
                            }
                        } else {
                            // It's just a comment -- eat it up
                            self.consume_rest_of_line(&mut comment_lexeme);
                            // Some(Token::Comment(comment_lexeme))
                            return self.lex();
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
                        //Some(Token::Comment(comment_lexeme))
                        return self.lex();
                    } else {
                        // Just a slash
                        Some(Token::Slash)
                    }
                }

                // Check for whitespace
                id if id.is_whitespace() => return self.lex(),

                // Check for numbers
                '.' => {
                    if self.peek().map(|c| matches!(c, '0'..='9')).unwrap_or(false) {
                        let mut lexeme = String::from(chr);
                        self.construct_number(&mut lexeme);
                        Some(Token::Real(lexeme.parse().unwrap()))
                    } else {
                        Some(Token::Dot)
                    }
                }
                '0'..='9' => {
                    let mut lexeme = String::from(chr);
                    self.construct_number(&mut lexeme);
                    Some(Token::Real(lexeme.parse().unwrap()))
                }

                // Check for keywords / identifiers
                id if id.is_alphabetic() || id == '_' => {
                    let mut lexeme = chr.into();
                    self.construct_word(&mut lexeme);

                    // Now let's check for keywords
                    match lexeme.as_ref() {
                        "switch" => Some(Token::Switch),
                        "case" => Some(Token::Case),
                        "break" => Some(Token::Break),
                        "return" => Some(Token::Return),
                        "default" => Some(Token::Default),
                        "enum" => Some(Token::Enum),
                        "and" => Some(Token::And),
                        "or" => Some(Token::Or),
                        "function" => Some(Token::Function),
                        "constructor" => Some(Token::Constructor),
                        "exit" => Some(Token::Exit),
                        "new" => Some(Token::New),
                        "global" => Some(Token::Global),
                        "globalvar" => Some(Token::Globalvar),
                        "mod" => Some(Token::Mod),
                        "try" => Some(Token::Try),
                        "with" => Some(Token::With),
                        "true" => Some(Token::True),
                        "false" => Some(Token::False),
                        "div" => Some(Token::Div),
                        "if" => Some(Token::If),
                        "else" => Some(Token::Else),
                        "for" => Some(Token::For),
                        "while" => Some(Token::While),
                        "do" => Some(Token::Do),
                        "until" => Some(Token::Until),
                        "repeat" => Some(Token::Repeat),
                        "var" => Some(Token::Var),
                        "self" => Some(Token::SelfKeyword),
                        "xor" => Some(Token::Xor),
                        "catch" => Some(Token::Catch),
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
    fn construct_word(&mut self, lexeme: &mut String) {
        while let Some(chr) = self.peek() {
            match chr {
                '_' | 'A'..='Z' | 'a'..='z' | '0'..='9' => {
                    lexeme.push(self.take().unwrap().1);
                }
                _ => break,
            }
        }
    }

    /// Will keep eating characters into the given string until it reaches a character that
    /// can't be used in an identifier.
    fn construct_number(&mut self, lexeme: &mut String) {
        while self.peek().map(|chr| chr.is_numeric()).unwrap_or(false) {
            lexeme.push(self.take().unwrap().1);
        }
        // Floats!
        if self.match_take('.') {
            lexeme.push('.');
            while self.peek().map(|chr| chr.is_numeric()).unwrap_or(false) {
                lexeme.push(self.take().unwrap().1);
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

    /// Consumes all upcoming characters that are whitespace into the string, stopping at
    /// the end of the line.
    fn consume_whitespace_on_line(&mut self, lexeme: &mut String) {
        while self
            .peek()
            .filter(|c| c.is_whitespace() && c != &'\n')
            .is_some()
        {
            lexeme.push(self.take().unwrap().1);
        }
    }

    /// Discards all upcoming characters that are whitespace.
    fn discard_whitespace(&mut self) {
        while self.peek().filter(|c| c.is_whitespace()).is_some() {
            self.take();
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
