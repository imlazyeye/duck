use super::{Span, Token, TokenType};
use fnv::FnvHashSet;
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
                        Some(TokenType::Real(lexeme.parse().unwrap()))
                    } else {
                        Some(TokenType::Dot)
                    }
                }
                ',' => Some(TokenType::Comma),
                '(' => Some(TokenType::LeftParenthesis),
                ')' => Some(TokenType::RightParenthesis),
                ';' => Some(TokenType::SemiColon),
                '0' if self.match_take('x') => {
                    let lexeme = self.construct_hex(self.next_char_boundary);
                    if !lexeme.is_empty() {
                        Some(TokenType::Hex(lexeme))
                    } else {
                        Some(TokenType::Invalid(lexeme))
                    }
                }
                '0'..='9' => {
                    let lexeme = self.construct_number(start_index);
                    Some(TokenType::Real(lexeme.parse().unwrap()))
                }
                '=' => {
                    if self.match_take('=') {
                        Some(TokenType::DoubleEqual)
                    } else {
                        Some(TokenType::Equal)
                    }
                }
                '{' => Some(TokenType::LeftBrace),
                '}' => Some(TokenType::RightBrace),
                '-' => {
                    if self.match_take('=') {
                        Some(TokenType::MinusEqual)
                    } else if self.match_take('-') {
                        Some(TokenType::DoubleMinus)
                    } else {
                        Some(TokenType::Minus)
                    }
                }
                '+' => {
                    if self.match_take('=') {
                        Some(TokenType::PlusEqual)
                    } else if self.match_take('+') {
                        Some(TokenType::DoublePlus)
                    } else {
                        Some(TokenType::Plus)
                    }
                }
                '*' => {
                    if self.match_take('=') {
                        Some(TokenType::StarEqual)
                    } else {
                        Some(TokenType::Star)
                    }
                }
                '<' => {
                    if self.match_take('=') {
                        Some(TokenType::LessThanOrEqual)
                    } else if self.match_take('<') {
                        Some(TokenType::BitShiftLeft)
                    } else if self.match_take('>') {
                        Some(TokenType::GreaterThanLessThan)
                    } else {
                        Some(TokenType::LessThan)
                    }
                }
                '>' => {
                    if self.match_take('=') {
                        Some(TokenType::GreaterThanOrEqual)
                    } else if self.match_take('>') {
                        Some(TokenType::BitShiftRight)
                    } else {
                        Some(TokenType::GreaterThan)
                    }
                }
                '&' => {
                    if self.match_take('&') {
                        Some(TokenType::DoubleAmpersand)
                    } else if self.match_take('=') {
                        Some(TokenType::AmpersandEqual)
                    } else {
                        Some(TokenType::Ampersand)
                    }
                }
                '|' => {
                    if self.match_take('|') {
                        Some(TokenType::DoublePipe)
                    } else if self.match_take('=') {
                        Some(TokenType::PipeEqual)
                    } else {
                        Some(TokenType::Pipe)
                    }
                }
                ':' => {
                    if self.match_take('=') {
                        Some(TokenType::ColonEqual)
                    } else {
                        Some(TokenType::Colon)
                    }
                }
                '[' => Some(TokenType::LeftSquareBracket),
                ']' => Some(TokenType::RightSquareBracket),
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
                                    return Token::new(TokenType::Eof, Span::new(start_index, self.next_char_boundary));
                                }
                            }
                        }
                        Some(TokenType::StringLiteral(
                            &self.source[start_position..self.next_char_boundary - 1],
                        ))
                    } else {
                        Some(TokenType::AtSign)
                    }
                }
                '~' => Some(TokenType::Tilde),
                '%' => {
                    if self.match_take('=') {
                        Some(TokenType::PercentEqual)
                    } else {
                        Some(TokenType::Percent)
                    }
                }
                '?' => {
                    if self.match_take('?') {
                        if self.match_take('=') {
                            Some(TokenType::DoubleInterrobangEquals)
                        } else {
                            Some(TokenType::DoubleInterrobang)
                        }
                    } else {
                        Some(TokenType::Interrobang)
                    }
                }
                '^' => {
                    if self.match_take('=') {
                        Some(TokenType::CirumflexEqual)
                    } else {
                        Some(TokenType::Circumflex)
                    }
                }
                '!' => {
                    if self.match_take('=') {
                        Some(TokenType::BangEqual)
                    } else {
                        Some(TokenType::Bang)
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
                            None => return Token::new(TokenType::Eof, Span::new(start_index, self.next_char_boundary)),
                        }
                    }
                    Some(TokenType::StringLiteral(
                        &self.source[start_index + 1..self.next_char_boundary - 1],
                    ))
                }
                '$' => {
                    let lexeme = self.construct_hex(self.next_char_boundary);
                    if !lexeme.is_empty() {
                        Some(TokenType::Hex(lexeme))
                    } else {
                        Some(TokenType::DollarSign)
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
                            TokenType::Macro(name, config, body),
                            Span::new(start_index, self.next_char_boundary),
                        )
                    } else if self.match_take_str("#region", start_index)
                        || self.match_take_str("#endregion", start_index)
                    {
                        self.discard_rest_of_line();
                        self.lex()
                    } else {
                        Token::new(TokenType::Hash, Span::new(start_index, self.next_char_boundary))
                    };
                }

                // Slashes can be SO MANY THINGS
                '/' => {
                    if self.match_take('=') {
                        Some(TokenType::SlashEqual)
                    } else if self.match_take('/') {
                        // Eat up the whitespace first...
                        self.consume_whitespace_on_line(start_index);

                        // See if this is an lint tag...
                        //
                        // FIXME: The parsing of lint tags is pretty gnarly. It should relaly be throwing errors when it
                        // doesn't find the things it wants, but for now it just discards everything.
                        if self.match_take('#') && self.match_take('[') {
                            // Looking promising!!
                            let level = self.construct_word(self.next_char_boundary);
                            if self.match_take('(') {
                                let tag = self.construct_word(self.next_char_boundary);
                                if self.match_take(')') && self.match_take(']') {
                                    self.consume_rest_of_line(self.next_char_boundary);
                                    Some(TokenType::LintTag(level, tag))
                                } else {
                                    return self.lex();
                                }
                            } else {
                                return self.lex();
                            }
                        } else {
                            // It's just a comment -- eat it up, including the `//` and whitespace we ate.
                            let comment_lexeme = self.consume_rest_of_line(start_index);
                            Some(TokenType::Comment(comment_lexeme))
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
                                    return Token::new(TokenType::Eof, Span::new(start_index, self.next_char_boundary));
                                }
                            }
                        }
                        let comment_lexeme = &self.source[start_index..self.next_char_boundary];
                        Some(TokenType::Comment(comment_lexeme))
                    } else {
                        // Just a slash
                        Some(TokenType::Slash)
                    }
                }

                // Check for keywords / identifiers
                id if id.is_alphabetic() || id == '_' => {
                    // Now let's check for keywords
                    match self.construct_word(start_index) {
                        "self" => Some(TokenType::SelfKeyword),
                        "var" => Some(TokenType::Var),
                        "return" => Some(TokenType::Return),
                        "case" => Some(TokenType::Case),
                        "if" => Some(TokenType::If),
                        "else" => Some(TokenType::Else),
                        "undefined" => Some(TokenType::Undefined),
                        "noone" => Some(TokenType::Noone),
                        "break" => Some(TokenType::Break),
                        "new" => Some(TokenType::New),
                        "function" => Some(TokenType::Function),
                        "true" => Some(TokenType::True),
                        "false" => Some(TokenType::False),
                        "static" => Some(TokenType::Static),
                        "for" => Some(TokenType::For),
                        "while" => Some(TokenType::While),
                        "and" => Some(TokenType::And),
                        "or" => Some(TokenType::Or),
                        "not" => Some(TokenType::Not),
                        "switch" => Some(TokenType::Switch),
                        "constructor" => Some(TokenType::Constructor),
                        "default" => Some(TokenType::Default),
                        "continue" => Some(TokenType::Continue),
                        "global" => Some(TokenType::Global),
                        "div" => Some(TokenType::Div),
                        "mod" => Some(TokenType::Mod),
                        "enum" => Some(TokenType::Enum),
                        "exit" => Some(TokenType::Exit),
                        "with" => Some(TokenType::With),
                        "repeat" => Some(TokenType::Repeat),
                        "do" => Some(TokenType::Do),
                        "until" => Some(TokenType::Until),
                        "globalvar" => Some(TokenType::Globalvar),
                        "xor" => Some(TokenType::Xor),
                        "other" => Some(TokenType::Other),
                        "try" => Some(TokenType::Try),
                        "catch" => Some(TokenType::Catch),
                        "finally" => Some(TokenType::Finally),
                        "throw" => Some(TokenType::Throw),
                        "then" => Some(TokenType::Then),
                        "delete" => Some(TokenType::Delete),
                        "begin" => Some(TokenType::Begin),
                        "end" => Some(TokenType::End),
                        id if MISC_GML_CONSTANTS.contains(id) => Some(TokenType::MiscConstant(id)),
                        lexeme => Some(TokenType::Identifier(lexeme)),
                    }
                }

                // Literally anything else!
                invalid => {
                    // this is chill, I promise
                    let tmp = Box::leak(Box::new([0u8; 4]));
                    let invalid = invalid.encode_utf8(tmp);
                    Some(TokenType::Invalid(invalid))
                }
            };

            if let Some(token_type) = token_type {
                Token::new(token_type, Span::new(start_index, self.next_char_boundary))
            } else {
                self.lex()
            }
        } else {
            Token::new(
                TokenType::Eof,
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
        if token.token_type == TokenType::Eof {
            None
        } else {
            Some(token)
        }
    }
}

pub static MISC_GML_CONSTANTS: Lazy<FnvHashSet<&'static str>> =
    Lazy::new(|| serde_json::from_str(include_str!("../../assets/misc_gml_constants.json")).unwrap());

#[allow(dead_code)]
pub static MISC_GML_VARIABLES: Lazy<FnvHashSet<&'static str>> =
    Lazy::new(|| serde_json::from_str(include_str!("../../assets/misc_gml_variables.json")).unwrap());
