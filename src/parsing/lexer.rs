use std::{iter::Peekable, str::FromStr};

use fnv::{FnvHashMap, FnvHashSet};
use once_cell::sync::Lazy;
use unicode_segmentation::{GraphemeIndices, UnicodeSegmentation};

use super::token::Token;

/// Takes gml  and converts it into tokens as an iterator.
pub struct Lexer<'a> {
    content: &'a str,
    input_characters: Peekable<GraphemeIndices<'a>>,
    cursor: usize,
}
impl<'a> Lexer<'a> {
    /// Creates a new Lexer, taking a string of gml source.
    pub fn new(content: &'a str) -> Self {
        Lexer {
            content,
            input_characters: content.grapheme_indices(true).peekable(),
            cursor: 0,
        }
    }

    /// Consumes the Lexer's source code until it identifies the next Token.
    fn lex(&mut self) -> (usize, Token) {
        if let Some((start_index, chr)) = self.take() {
            let token_type = match chr {
                id if id.is_whitespace() => return self.lex(),
                '.' => {
                    if self.peek().map(|c| matches!(c, '0'..='9')).unwrap_or(false) {
                        let mut lexeme = String::from(chr);
                        self.construct_number(&mut lexeme);
                        Some(Token::Real(lexeme.parse().unwrap()))
                    } else {
                        Some(Token::Dot)
                    }
                }
                ',' => Some(Token::Comma),
                '(' => Some(Token::LeftParenthesis),
                ')' => Some(Token::RightParenthesis),
                ';' => Some(Token::SemiColon),
                '0' if self.match_take('x') => {
                    let mut lexeme = String::new();
                    self.construct_hex(&mut lexeme);
                    if !lexeme.is_empty() {
                        Some(Token::Hex(lexeme))
                    } else {
                        Some(Token::Invalid(lexeme))
                    }
                }
                '0'..='9' => {
                    let mut lexeme = String::from(chr);
                    self.construct_number(&mut lexeme);
                    Some(Token::Real(lexeme.parse().unwrap()))
                }
                '=' => {
                    if self.match_take('=') {
                        Some(Token::DoubleEqual)
                    } else {
                        Some(Token::Equal)
                    }
                }
                '{' => Some(Token::LeftBrace),
                '}' => Some(Token::RightBrace),
                '-' => {
                    if self.match_take('=') {
                        Some(Token::MinusEqual)
                    } else if self.match_take('-') {
                        Some(Token::DoubleMinus)
                    } else {
                        Some(Token::Minus)
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
                '*' => {
                    if self.match_take('=') {
                        Some(Token::StarEqual)
                    } else {
                        Some(Token::Star)
                    }
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
                ':' => Some(Token::Colon),
                '[' => Some(Token::LeftSquareBracket),
                ']' => Some(Token::RightSquareBracket),
                '@' => {
                    let single_quote = self.match_take('\'');
                    let double_quote = self.match_take('"');
                    if single_quote || double_quote {
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
                                            '"' if double_quote && !in_escape => break,
                                            '\'' if single_quote && !in_escape => break,
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
                    } else {
                        Some(Token::AtSign)
                    }
                }
                '~' => Some(Token::Tilde),
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
                '$' => {
                    let mut lexeme = String::new();
                    self.construct_hex(&mut lexeme);
                    if !lexeme.is_empty() {
                        Some(Token::Hex(lexeme))
                    } else {
                        Some(Token::DollarSign)
                    }
                }
                // Regions / Macros
                '#' => {
                    return if self.match_take_str("#macro", start_index) {
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
                    } else if self.match_take_str("#region", start_index)
                        || self.match_take_str("#endregion", start_index)
                    {
                        self.discard_rest_of_line();
                        self.lex()
                    } else {
                        (start_index, Token::Hash)
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

                // Check for keywords / identifiers
                id if id.is_alphabetic() || id == '_' => {
                    let mut lexeme = chr.into();
                    self.construct_word(&mut lexeme);

                    // Now let's check for keywords
                    match lexeme.as_ref() {
                        "self" => Some(Token::SelfKeyword),
                        "var" => Some(Token::Var),
                        "return" => Some(Token::Return),
                        "case" => Some(Token::Case),
                        "if" => Some(Token::If),
                        "undefined" => Some(Token::Undefined),
                        "noone" => Some(Token::Noone),
                        "break" => Some(Token::Break),
                        "new" => Some(Token::New),
                        "function" => Some(Token::Function),
                        "true" => Some(Token::True),
                        "false" => Some(Token::False),
                        "static" => Some(Token::Static),
                        "for" => Some(Token::For),
                        "while" => Some(Token::While),
                        "and" => Some(Token::And),
                        "or" => Some(Token::Or),
                        "not" => Some(Token::Not),
                        "switch" => Some(Token::Switch),
                        "constructor" => Some(Token::Constructor),
                        "default" => Some(Token::Default),
                        "continue" => Some(Token::Continue),
                        "global" => Some(Token::Global),
                        "div" => Some(Token::Div),
                        "mod" => Some(Token::Mod),
                        "enum" => Some(Token::Enum),
                        "exit" => Some(Token::Exit),
                        "repeat" => Some(Token::Repeat),
                        "do" => Some(Token::Do),
                        "until" => Some(Token::Until),
                        "globalvar" => Some(Token::Globalvar),
                        "with" => Some(Token::With),
                        "else" => Some(Token::Else),
                        "xor" => Some(Token::Xor),
                        "try" => Some(Token::Try),
                        "catch" => Some(Token::Catch),
                        "finally" => Some(Token::Finally),
                        "then" => Some(Token::Then),
                        id if MISC_GML_CONSTANTS.contains(id) => {
                            Some(Token::MiscConstant(id.to_string()))
                        }
                        _ => Some(Token::Identifier(lexeme)),
                    }
                }

                // Literally anything else!
                invalid => Some(Token::Invalid(invalid.into())),
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

    /// Checks if the the following characters are upcoming in the source. If they are,
    /// consumes the characters.
    fn match_take_str(&mut self, lexeme: &str, start_pos: usize) -> bool {
        if start_pos + lexeme.len() <= self.content.len() {
            if &self.content[start_pos..start_pos + lexeme.len()] == lexeme {
                for _ in 0..lexeme.len() - 1 {
                    self.take();
                }
                true
            } else {
                false
            }
        } else {
            false
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

    /// Will keep eating charcters into the given string so long as they are valid hex-characters
    /// (ie: 4ab02f)
    fn construct_hex(&mut self, lexeme: &mut String) {
        while let Some(chr) = self.peek() {
            match chr {
                'A'..='F' | 'a'..='f' | '0'..='9' => {
                    lexeme.push(self.take().unwrap().1);
                }
                _ => break,
            }
        }
    }

    /// Returns the next character in the source code.
    fn peek(&mut self) -> Option<char> {
        self.input_characters
            .peek()
            .and_then(|(_, g)| g.chars().next()) // TODO this is terrible!
    }

    /// Consumes and returns the next character in the source code.
    fn take(&mut self) -> Option<(usize, char)> {
        self.input_characters
            .next()
            .map(|(c, g)| (c, g.chars().next().unwrap())) // TODO this is terrible!
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
            .filter(|c| c.is_whitespace() && c != &'\n' && c != &'\r')
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

pub(super) static MISC_GML_CONSTANTS: Lazy<FnvHashSet<&'static str>> =
    Lazy::new(|| serde_json::from_str(&MISC_GML_CONSTANT_FILE_DATA).unwrap());

static MISC_GML_CONSTANT_FILE_DATA: Lazy<String> = Lazy::new(|| {
    std::fs::read_to_string(dbg!(std::env::current_dir()
        .unwrap()
        .join("assets/misc_gml_constants.json")))
    .unwrap()
});

#[allow(dead_code)]
pub(super) static MISC_GML_VARIABLES: Lazy<FnvHashSet<&'static str>> =
    Lazy::new(|| serde_json::from_str(&MISC_GML_VARIABLES_FILE_DATA).unwrap());

#[allow(dead_code)]
static MISC_GML_VARIABLES_FILE_DATA: Lazy<String> = Lazy::new(|| {
    std::fs::read_to_string(dbg!(std::env::current_dir()
        .unwrap()
        .join("assets/misc_gml_variables.json")))
    .unwrap()
});
