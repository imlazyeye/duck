use super::{Span, Token, TokenKind};
use hashbrown::HashSet;
use once_cell::sync::Lazy;
use std::iter::Peekable;
use unicode_segmentation::{GraphemeIndices, UnicodeSegmentation};

/// Takes gml and converts it into tokens as an iterator.
pub struct Lexer {
    source: &'static str,
    input_characters: Peekable<GraphemeIndices<'static>>,
    next_char_boundary: usize,
}
impl Lexer {
    /// Creates a new Lexer, taking a string of gml source.
    pub fn new(source: &'static str) -> Self {
        Self {
            source,
            input_characters: source.grapheme_indices(true).peekable(),
            next_char_boundary: 1,
        }
    }

    /// Consumes the Lexer's source code until it identifies the next Token.
    fn lex(&mut self) -> Token {
        if let Some((start_index, chr)) = self.take() {
            let token_type = match chr {
                id if id.is_whitespace() => return self.lex(),
                '.' => {
                    if self.peek().map_or(false, |c| matches!(c, '0'..='9')) {
                        let lexeme = self.construct_number(start_index);
                        Some(TokenKind::Real(lexeme.parse().unwrap()))
                    } else {
                        Some(TokenKind::Dot)
                    }
                }
                ',' => Some(TokenKind::Comma),
                '(' => Some(TokenKind::LeftParenthesis),
                ')' => Some(TokenKind::RightParenthesis),
                ';' => Some(TokenKind::SemiColon),
                '0' if self.match_take('x') => {
                    let lexeme = self.construct_hex(self.next_char_boundary);
                    if !lexeme.is_empty() {
                        Some(TokenKind::Hex(lexeme))
                    } else {
                        Some(TokenKind::Invalid(lexeme))
                    }
                }
                '0'..='9' => {
                    let lexeme = self.construct_number(start_index);
                    Some(TokenKind::Real(lexeme.parse().unwrap()))
                }
                '=' => {
                    if self.match_take('=') {
                        Some(TokenKind::DoubleEqual)
                    } else {
                        Some(TokenKind::Equal)
                    }
                }
                '{' => Some(TokenKind::LeftBrace),
                '}' => Some(TokenKind::RightBrace),
                '-' => {
                    if self.match_take('=') {
                        Some(TokenKind::MinusEqual)
                    } else if self.match_take('-') {
                        Some(TokenKind::DoubleMinus)
                    } else {
                        Some(TokenKind::Minus)
                    }
                }
                '+' => {
                    if self.match_take('=') {
                        Some(TokenKind::PlusEqual)
                    } else if self.match_take('+') {
                        Some(TokenKind::DoublePlus)
                    } else {
                        Some(TokenKind::Plus)
                    }
                }
                '*' => {
                    if self.match_take('=') {
                        Some(TokenKind::StarEqual)
                    } else {
                        Some(TokenKind::Star)
                    }
                }
                '<' => {
                    if self.match_take('=') {
                        Some(TokenKind::LessThanOrEqual)
                    } else if self.match_take('<') {
                        Some(TokenKind::BitShiftLeft)
                    } else if self.match_take('>') {
                        Some(TokenKind::LessThanGreaterThan)
                    } else {
                        Some(TokenKind::LessThan)
                    }
                }
                '>' => {
                    if self.match_take('=') {
                        Some(TokenKind::GreaterThanOrEqual)
                    } else if self.match_take('>') {
                        Some(TokenKind::BitShiftRight)
                    } else {
                        Some(TokenKind::GreaterThan)
                    }
                }
                '&' => {
                    if self.match_take('&') {
                        Some(TokenKind::DoubleAmpersand)
                    } else if self.match_take('=') {
                        Some(TokenKind::AmpersandEqual)
                    } else {
                        Some(TokenKind::Ampersand)
                    }
                }
                '|' => {
                    if self.match_take('|') {
                        Some(TokenKind::DoublePipe)
                    } else if self.match_take('=') {
                        Some(TokenKind::PipeEqual)
                    } else {
                        Some(TokenKind::Pipe)
                    }
                }
                ':' => {
                    if self.match_take('=') {
                        Some(TokenKind::ColonEqual)
                    } else {
                        Some(TokenKind::Colon)
                    }
                }
                '[' => Some(TokenKind::LeftSquareBracket),
                ']' => Some(TokenKind::RightSquareBracket),
                // FIXME: Rather unfortunately our string parsing here is seperated from our string parsing below.
                // Someone could probably stich them together, or at the very least, create a shared function for them.
                '@' => {
                    let single_quote = self.match_take('\'');
                    let double_quote = self.match_take('"');
                    if single_quote || double_quote {
                        let start_position = self.next_char_boundary;
                        let mut in_escape = false;
                        loop {
                            match self.take() {
                                Some((_, chr)) => {
                                    if in_escape {
                                        in_escape = false;
                                    } else {
                                        match chr {
                                            '"' if double_quote && !in_escape => break,
                                            '\'' if single_quote && !in_escape => break,
                                            '\\' => {
                                                in_escape = true;
                                            }
                                            _ => {}
                                        }
                                    }
                                }
                                None => {
                                    return Token::new(TokenKind::Eof, Span::new(start_index, self.next_char_boundary));
                                }
                            }
                        }
                        Some(TokenKind::StringLiteral(
                            &self.source[start_position..self.next_char_boundary - 1],
                        ))
                    } else {
                        Some(TokenKind::AtSign)
                    }
                }
                '~' => Some(TokenKind::Tilde),
                '%' => {
                    if self.match_take('=') {
                        Some(TokenKind::PercentEqual)
                    } else {
                        Some(TokenKind::Percent)
                    }
                }
                '?' => {
                    if self.match_take('?') {
                        if self.match_take('=') {
                            Some(TokenKind::DoubleHookEquals)
                        } else {
                            Some(TokenKind::DoubleHook)
                        }
                    } else {
                        Some(TokenKind::Hook)
                    }
                }
                '^' => {
                    if self.match_take('=') {
                        Some(TokenKind::CaretEquals)
                    } else {
                        Some(TokenKind::Caret)
                    }
                }
                '!' => {
                    if self.match_take('=') {
                        Some(TokenKind::BangEqual)
                    } else {
                        Some(TokenKind::Bang)
                    }
                }
                '"' => {
                    let mut in_escape = false;
                    loop {
                        match self.take() {
                            Some((_, chr)) => {
                                if in_escape {
                                    in_escape = false;
                                } else {
                                    match chr {
                                        '"' if !in_escape => break,
                                        '\\' => {
                                            in_escape = true;
                                        }
                                        _ => {}
                                    }
                                }
                            }
                            None => return Token::new(TokenKind::Eof, Span::new(start_index, self.next_char_boundary)),
                        }
                    }
                    Some(TokenKind::StringLiteral(
                        &self.source[start_index + 1..self.next_char_boundary - 1],
                    ))
                }
                '$' => {
                    let lexeme = self.construct_hex(self.next_char_boundary);
                    if !lexeme.is_empty() {
                        Some(TokenKind::Hex(lexeme))
                    } else {
                        Some(TokenKind::DollarSign)
                    }
                }
                // Regions / Macros
                '#' => {
                    return if self.match_take_str("#macro", start_index) {
                        self.discard_whitespace();
                        let iden_one = self.construct_word(self.next_char_boundary);
                        let (name, config) = if self.match_take(':') {
                            let name = self.construct_word(self.next_char_boundary);
                            (name, Some(iden_one))
                        } else {
                            (iden_one, None)
                        };
                        self.discard_whitespace();
                        let body = self.consume_rest_of_line(self.next_char_boundary);
                        Token::new(
                            TokenKind::Macro(name, config, body),
                            Span::new(start_index, self.next_char_boundary),
                        )
                    } else if self.match_take_str("#region", start_index)
                        || self.match_take_str("#endregion", start_index)
                    {
                        self.discard_rest_of_line();
                        self.lex()
                    } else {
                        Token::new(TokenKind::Hash, Span::new(start_index, self.next_char_boundary))
                    };
                }

                // Slashes can be SO MANY THINGS
                '/' => {
                    if self.match_take('=') {
                        Some(TokenKind::SlashEqual)
                    } else if self.match_take('/') {
                        // Eat the remaining slashes...
                        loop {
                            if !self.match_take('/') {
                                break;
                            }
                        }
                        
                        // Eat up the whitespace first...
                        self.consume_whitespace_on_line(start_index);

                        // See if this is an lint tag...
                        if self.match_take('#') && self.match_take('[') {
                            let tag = self.construct_word(self.next_char_boundary);
                            let parameter = if self.match_take('(') {
                                let parameter = self.construct_word(self.next_char_boundary);
                                self.match_take(')');
                                Some(parameter)
                            } else {
                                None
                            };
                            if self.match_take(']') {
                                self.consume_rest_of_line(self.next_char_boundary);
                                Some(TokenKind::Tag(tag, parameter))
                            } else {
                                return self.lex();
                            }
                        } else {
                            // It's just a comment -- eat it up, including the `//` and whitespace we ate.
                            let comment_lexeme = self.consume_rest_of_line(start_index);
                            Some(TokenKind::Comment(comment_lexeme))
                        }
                    } else if self.match_take('*') {
                        // Multi-line comment
                        loop {
                            match self.take() {
                                Some((_, chr)) => {
                                    if chr == '*' && self.match_take('/') {
                                        break;
                                    }
                                }
                                None => {
                                    return Token::new(TokenKind::Eof, Span::new(start_index, self.next_char_boundary));
                                }
                            }
                        }
                        let comment_lexeme = &self.source[start_index..self.next_char_boundary];
                        Some(TokenKind::Comment(comment_lexeme))
                    } else {
                        // Just a slash
                        Some(TokenKind::Slash)
                    }
                }

                // Check for keywords / identifiers
                id if id.is_alphabetic() || id == '_' => {
                    // Now let's check for keywords
                    match self.construct_word(start_index) {
                        "self" => Some(TokenKind::SelfKeyword),
                        "var" => Some(TokenKind::Var),
                        "return" => Some(TokenKind::Return),
                        "case" => Some(TokenKind::Case),
                        "if" => Some(TokenKind::If),
                        "else" => Some(TokenKind::Else),
                        "undefined" => Some(TokenKind::Undefined),
                        "noone" => Some(TokenKind::Noone),
                        "break" => Some(TokenKind::Break),
                        "new" => Some(TokenKind::New),
                        "function" => Some(TokenKind::Function),
                        "true" => Some(TokenKind::True),
                        "false" => Some(TokenKind::False),
                        "static" => Some(TokenKind::Static),
                        "for" => Some(TokenKind::For),
                        "while" => Some(TokenKind::While),
                        "and" => Some(TokenKind::And),
                        "or" => Some(TokenKind::Or),
                        "not" => Some(TokenKind::Not),
                        "switch" => Some(TokenKind::Switch),
                        "constructor" => Some(TokenKind::Constructor),
                        "default" => Some(TokenKind::Default),
                        "continue" => Some(TokenKind::Continue),
                        "global" => Some(TokenKind::Global),
                        "div" => Some(TokenKind::Div),
                        "mod" => Some(TokenKind::Mod),
                        "enum" => Some(TokenKind::Enum),
                        "exit" => Some(TokenKind::Exit),
                        "with" => Some(TokenKind::With),
                        "repeat" => Some(TokenKind::Repeat),
                        "do" => Some(TokenKind::Do),
                        "until" => Some(TokenKind::Until),
                        "globalvar" => Some(TokenKind::Globalvar),
                        "xor" => Some(TokenKind::Xor),
                        "other" => Some(TokenKind::Other),
                        "try" => Some(TokenKind::Try),
                        "catch" => Some(TokenKind::Catch),
                        "finally" => Some(TokenKind::Finally),
                        "throw" => Some(TokenKind::Throw),
                        "then" => Some(TokenKind::Then),
                        "delete" => Some(TokenKind::Delete),
                        "begin" => Some(TokenKind::Begin),
                        "end" => Some(TokenKind::End),
                        id if MISC_GML_CONSTANTS.contains(id) => Some(TokenKind::MiscConstant(id)),
                        lexeme => Some(TokenKind::Identifier(lexeme)),
                    }
                }

                invalid => {
                    // this is chill, I promise
                    let tmp = Box::leak(Box::new([0u8; 4]));
                    let invalid = invalid.encode_utf8(tmp);
                    Some(TokenKind::Invalid(invalid))
                }
            };

            if let Some(token_type) = token_type {
                Token::new(token_type, Span::new(start_index, self.next_char_boundary))
            } else {
                self.lex()
            }
        } else {
            Token::new(
                TokenKind::Eof,
                Span::new(self.next_char_boundary, self.next_char_boundary),
            )
        }
    }

    /// Consumes the rest of the line into the string.
    fn consume_rest_of_line(&mut self, start_pos: usize) -> &'static str {
        while self.peek().map_or(false, |chr| chr != '\r' && chr != '\n') {
            self.take().unwrap();
        }
        &self.source[start_pos..self.next_char_boundary]
    }

    /// Discards the remainder of the line.
    fn discard_rest_of_line(&mut self) {
        while self.peek().map_or(false, |chr| chr != '\r' && chr != '\n') {
            self.take();
        }
    }

    /// Checks if the the following characters are upcoming in the source. If
    /// they are, consumes the characters.
    fn match_take_str(&mut self, lexeme: &str, start_pos: usize) -> bool {
        if start_pos + lexeme.len() <= self.source.len() {
            if &self.source[start_pos..start_pos + lexeme.len()] == lexeme {
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

    /// Will keep eating characters into the given string until it reaches a
    /// charcter that can't be used in an identifier.
    fn construct_word(&mut self, start_pos: usize) -> &'static str {
        while let Some(chr) = self.peek() {
            match chr {
                '_' | 'A'..='Z' | 'a'..='z' | '0'..='9' => {
                    self.take().unwrap();
                }
                _ => break,
            }
        }
        &self.source[start_pos..self.next_char_boundary]
    }

    /// Will keep eating characters into the given string until it reaches a
    /// character that can't be used in an identifier.
    fn construct_number(&mut self, start_pos: usize) -> &'static str {
        while self.peek().map_or(false, |chr| chr.is_numeric()) {
            self.take().unwrap();
        }
        // Floats!
        if self.match_take('.') {
            while self.peek().map_or(false, |chr| chr.is_numeric()) {
                self.take().unwrap();
            }
        }
        &self.source[start_pos..self.next_char_boundary]
    }

    /// Will keep eating charcters into the given string so long as they are
    /// valid hex-characters (ie: 4ab02f)
    fn construct_hex(&mut self, start_pos: usize) -> &'static str {
        while let Some(chr) = self.peek() {
            match chr {
                'A'..='F' | 'a'..='f' | '0'..='9' => {
                    self.take().unwrap();
                }
                _ => break,
            }
        }
        &self.source[start_pos..self.next_char_boundary]
    }

    /// Consumes all upcoming characters that are whitespace into the string,
    /// stopping at the end of the line.
    fn consume_whitespace_on_line(&mut self, start_pos: usize) -> &'static str {
        while self
            .peek()
            .filter(|c| c.is_whitespace() && c != &'\n' && c != &'\r')
            .is_some()
        {
            self.take().unwrap();
        }
        &self.source[start_pos..self.next_char_boundary]
    }

    /// Discards all upcoming characters that are whitespace.
    fn discard_whitespace(&mut self) {
        while self.peek().filter(|c| c.is_whitespace()).is_some() {
            self.take();
        }
    }

    /// Returns the next character in the source code.
    ///
    /// FIXME: Has bad behavior. Read the docs on [Lexer::take].
    fn peek(&mut self) -> Option<char> {
        self.input_characters.peek().and_then(|(_, g)| g.chars().next())
    }

    /// Consumes the next character in the source code if it matches the given
    /// character. Returns if it succeeds.
    fn match_take(&mut self, chr: char) -> bool {
        if self.peek() == Some(chr) {
            self.take();
            true
        } else {
            false
        }
    }

    /// Consumes and returns the next character in the source code.
    ///
    /// FIXME: This is currently set up poorly for non-latin characters.
    ///
    /// UTF8 "characters" (the way we as people see them) are not all a consistent byte length. A
    /// `char`, however, is. This means that certain characters (most often letters used in
    /// non-english languages, such as `ß`) can be composed of multiple `char`s. This is a big
    /// problem for how we parse things!
    ///
    /// We already utilize the `unicode_segmentation` crate, which as far as my reading showed me,
    /// is the answer to this sort of an issue. Fully adapting us to it though is not super easy
    /// though (instead of parsing over chars, you are now parsing over &str's). When I tried to
    /// throw it together, it was messy, and most importanlty, much slower.
    ///
    /// For now, we simply take the first `char` in any given grapheme, advance our cursor to the
    /// end of the grapheme. As a result, we won't ever break, but we will have junk characters (for
    /// example, `🦆` will turn into... something else!) Fortunately,
    /// these most often come up within strings, not lexical code itself, and duck does not
    /// currently do much with the contents of the strings.
    ///
    /// None the less, this will need to be addressed and fixed in the future!
    fn take(&mut self) -> Option<(usize, char)> {
        self.input_characters.next().map(|(c, g)| {
            self.next_char_boundary = c + g.len(); // advances us to the start of the next graphmee
            (c, g.chars().next().unwrap())
        })
    }
}

impl Iterator for Lexer {
    type Item = Token;
    /// Returns the next Token in the Lexer.
    fn next(&mut self) -> Option<Self::Item> {
        let token = self.lex();
        if token.token_type == TokenKind::Eof {
            None
        } else {
            Some(token)
        }
    }
}

/// Various constants in gml that are not specificlly tracked in duck.
pub static MISC_GML_CONSTANTS: Lazy<HashSet<&'static str>> =
    Lazy::new(|| serde_json::from_str(include_str!("../../assets/misc_gml_constants.json")).unwrap());

/// Various built-in variables in gml that are not specificlly tracked in duck.
pub static MISC_GML_VARIABLES: Lazy<HashSet<&'static str>> =
    Lazy::new(|| serde_json::from_str(include_str!("../../assets/misc_gml_variables.json")).unwrap());
