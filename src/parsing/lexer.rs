use super::token::Token;
use fnv::FnvHashSet;
use once_cell::sync::Lazy;
use std::iter::Peekable;
use unicode_segmentation::{GraphemeIndices, UnicodeSegmentation};

/// Takes gml and converts it into tokens as an iterator.
pub struct Lexer {
    source: &'static str,
    input_characters: Peekable<GraphemeIndices<'static>>,
    next_char_boundary: usize,
    token_cursor: usize,
}
impl Lexer {
    /// Creates a new Lexer, taking a string of gml source.
    pub fn new(source: &'static str) -> Self {
        Self {
            source,
            input_characters: source.grapheme_indices(true).peekable(),
            next_char_boundary: 1,
            token_cursor: 0,
        }
    }

    /// Consumes the Lexer's source code until it identifies the next Token.
    #[allow(clippy::too_many_lines)]
    fn lex(&mut self) -> (usize, Token) {
        if let Some((start_index, chr)) = self.take() {
            let token_type = match chr {
                id if id.is_whitespace() => return self.lex(),
                '.' => {
                    if self.peek().map_or(false, |c| matches!(c, '0'..='9')) {
                        let lexeme = self.construct_number(start_index);
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
                    let lexeme = self.construct_hex(self.next_char_boundary);
                    if !lexeme.is_empty() {
                        Some(Token::Hex(lexeme))
                    } else {
                        Some(Token::Invalid(lexeme))
                    }
                }
                '0'..='9' => {
                    let lexeme = self.construct_number(start_index);
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
                                None => return (start_index, Token::Eof),
                            }
                        }
                        Some(Token::StringLiteral(
                            &self.source[start_position..self.next_char_boundary - 1],
                        ))
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
                            None => return (start_index, Token::Eof),
                        }
                    }
                    Some(Token::StringLiteral(
                        &self.source[start_index + 1..self.next_char_boundary - 1],
                    ))
                }
                '$' => {
                    let lexeme = self.construct_hex(self.next_char_boundary);
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
                        let iden_one = self.construct_word(self.next_char_boundary);
                        let (name, config) = if self.match_take(':') {
                            let name = self.construct_word(self.next_char_boundary);
                            (name, Some(iden_one))
                        } else {
                            (iden_one, None)
                        };
                        self.discard_whitespace();
                        let body = self.consume_rest_of_line(self.next_char_boundary);
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
                        self.consume_whitespace_on_line(start_index);

                        // See if this is an lint tag...
                        //
                        // FIXME: The parsing of lint tags is pretty gnarly. It should relaly be throwing errors when it
                        // doesn't find the things it wants, but for now it just discards everything.
                        if self.match_take('#') && self.match_take('[') {
                            // Looking promising!!
                            let level = self.construct_word(self.next_char_boundary);
                            match level {
                                "allow" | "warn" | "deny" => {
                                    if self.match_take('(') {
                                        let tag = self.construct_word(self.next_char_boundary);
                                        if self.match_take(')') && self.match_take(']') {
                                            self.consume_rest_of_line(self.next_char_boundary);
                                            Some(Token::LintTag(level, tag))
                                        } else {
                                            return self.lex();
                                        }
                                    } else {
                                        return self.lex();
                                    }
                                }
                                _ => return self.lex(),
                            }
                        } else {
                            // It's just a comment -- eat it up, including the `//` and whitespace we ate.
                            self.consume_rest_of_line(start_index);
                            // Some(Token::Comment(comment_lexeme))
                            return self.lex();
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
                                None => return (start_index, Token::Eof),
                            }
                        }
                        // Some(Token::Comment(comment_lexeme))
                        return self.lex();
                    } else {
                        // Just a slash
                        Some(Token::Slash)
                    }
                }

                // Check for keywords / identifiers
                id if id.is_alphabetic() || id == '_' => {
                    // Now let's check for keywords
                    match self.construct_word(start_index) {
                        "self" => Some(Token::SelfKeyword),
                        "var" => Some(Token::Var),
                        "return" => Some(Token::Return),
                        "case" => Some(Token::Case),
                        "if" => Some(Token::If),
                        "else" => Some(Token::Else),
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
                        "with" => Some(Token::With),
                        "repeat" => Some(Token::Repeat),
                        "do" => Some(Token::Do),
                        "until" => Some(Token::Until),
                        "globalvar" => Some(Token::Globalvar),
                        "xor" => Some(Token::Xor),
                        "other" => Some(Token::Other),
                        "try" => Some(Token::Try),
                        "catch" => Some(Token::Catch),
                        "finally" => Some(Token::Finally),
                        "throw" => Some(Token::Throw),
                        "then" => Some(Token::Then),
                        "delete" => Some(Token::Delete),
                        "begin" => Some(Token::Begin),
                        "end" => Some(Token::End),
                        id if MISC_GML_CONSTANTS.contains(id) => Some(Token::MiscConstant(id)),
                        lexeme => Some(Token::Identifier(lexeme)),
                    }
                }

                // Literally anything else!
                invalid => {
                    // this is chill, I promise
                    let tmp = Box::leak(Box::new([0u8; 4]));
                    let invalid = invalid.encode_utf8(tmp);
                    Some(Token::Invalid(invalid))
                }
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
    /// example, `🦆 ` will turn into... something else!) Fortunately,
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
    type Item = (usize, Token);
    /// Returns the next Token in the Lexer.
    fn next(&mut self) -> Option<Self::Item> {
        let (position, token) = self.lex();
        self.token_cursor = position;
        if token == Token::Eof {
            None
        } else {
            Some((position, token))
        }
    }
}

pub(super) static MISC_GML_CONSTANTS: Lazy<FnvHashSet<&'static str>> =
    Lazy::new(|| serde_json::from_str(include_str!("../../assets/misc_gml_constants.json")).unwrap());

#[allow(dead_code)]
pub(super) static MISC_GML_VARIABLES: Lazy<FnvHashSet<&'static str>> =
    Lazy::new(|| serde_json::from_str(include_str!("../../assets/misc_gml_variables.json")).unwrap());
