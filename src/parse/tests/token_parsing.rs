use crate::parse::{
    lexer::{Lexer, MISC_GML_CONSTANTS, MISC_GML_VARIABLES},
    TokenType,
};
use pretty_assertions::assert_eq;

fn harness_multi(source: &'static str, expected: impl Into<Vec<TokenType>>) {
    let outputed = Lexer::new(source)
        .map(|token| token.token_type)
        .collect::<Vec<TokenType>>();
    let expected = expected.into();
    assert_eq!(*outputed, expected)
}

fn harness_single(source: &'static str, expected: TokenType) {
    harness_multi(source, vec![expected]);
}

#[test]
fn whitespace() {
    harness_multi(" \n\t", []);
}

#[test]
fn float() {
    harness_single("0.2", TokenType::Real(0.2));
}

#[test]
fn float_no_prefix_digit() {
    harness_single(".2", TokenType::Real(0.2));
}

#[test]
fn int() {
    harness_single("1", TokenType::Real(1.0));
}

#[test]
fn hex_0x() {
    harness_single("0x1", TokenType::Hex("1"));
}

#[test]
fn hex_dollar_sign() {
    harness_single("$1", TokenType::Hex("1"));
}

#[test]
fn invalid_hex() {
    harness_multi("$q", [TokenType::DollarSign, TokenType::Identifier("q")]);
}

#[test]
fn string() {
    harness_single("\"foo\"", TokenType::StringLiteral("foo"));
}

#[test]
fn multiline_string() {
    harness_single("@\"\n\nfoo\"", TokenType::StringLiteral("\n\nfoo"));
}

#[test]
fn multiline_string_alt() {
    harness_single("@'\n\nfoo'", TokenType::StringLiteral("\n\nfoo"));
}

#[test]
fn identifier() {
    harness_single("foo", TokenType::Identifier("foo"));
}

#[test]
fn macro_declaration() {
    harness_single("#macro foo 0", TokenType::Macro("foo", None, "0"));
}

#[test]
fn macro_declaration_with_config() {
    harness_single("#macro bar:foo 0", TokenType::Macro("foo", Some("bar"), "0"));
}

#[test]
fn comments() {
    // Note: comments are currently disabled!
    harness_multi("// comment", [TokenType::Comment("// comment")]);
    harness_multi("/// comment", [TokenType::Comment("/// comment")]);
    harness_multi("/* comment */", [TokenType::Comment("/* comment */")]);
    harness_multi("/*\n comment \n*/", [TokenType::Comment("/*\n comment \n*/")]);
}

#[test]
fn lint_tags() {
    harness_single("// #[allow(and_keyword)]", TokenType::Tag("allow", Some("and_keyword")));
    harness_single("// #[warn(and_keyword)]", TokenType::Tag("warn", Some("and_keyword")));
    harness_single("// #[deny(and_keyword)]", TokenType::Tag("deny", Some("and_keyword")));
}

#[test]
fn keywords() {
    harness_multi(
        "switch case break return enum default and or function constructor exit global
        div new mod globalvar try self catch with true false if else while for do until
        repeat var continue static then finally undefined noone not xor other delete 
        begin end throw",
        [
            TokenType::Switch,
            TokenType::Case,
            TokenType::Break,
            TokenType::Return,
            TokenType::Enum,
            TokenType::Default,
            TokenType::And,
            TokenType::Or,
            TokenType::Function,
            TokenType::Constructor,
            TokenType::Exit,
            TokenType::Global,
            TokenType::Div,
            TokenType::New,
            TokenType::Mod,
            TokenType::Globalvar,
            TokenType::Try,
            TokenType::SelfKeyword,
            TokenType::Catch,
            TokenType::With,
            TokenType::True,
            TokenType::False,
            TokenType::If,
            TokenType::Else,
            TokenType::While,
            TokenType::For,
            TokenType::Do,
            TokenType::Until,
            TokenType::Repeat,
            TokenType::Var,
            TokenType::Continue,
            TokenType::Static,
            TokenType::Then,
            TokenType::Finally,
            TokenType::Undefined,
            TokenType::Noone,
            TokenType::Not,
            TokenType::Xor,
            TokenType::Other,
            TokenType::Delete,
            TokenType::Begin,
            TokenType::End,
            TokenType::Throw,
        ],
    )
}

#[test]
fn symbols() {
    harness_multi(
        ":.{}()[],& &&||^ = ==!=%%=/ *+-!? ????=> >=< <= |;#+=-=*=/=++--$|=&=^=~<<>>@",
        [
            TokenType::Colon,
            TokenType::Dot,
            TokenType::LeftBrace,
            TokenType::RightBrace,
            TokenType::LeftParenthesis,
            TokenType::RightParenthesis,
            TokenType::LeftSquareBracket,
            TokenType::RightSquareBracket,
            TokenType::Comma,
            TokenType::Ampersand,
            TokenType::DoubleAmpersand,
            TokenType::DoublePipe,
            TokenType::Caret,
            TokenType::Equal,
            TokenType::DoubleEqual,
            TokenType::BangEqual,
            TokenType::Percent,
            TokenType::PercentEqual,
            TokenType::Slash,
            TokenType::Star,
            TokenType::Plus,
            TokenType::Minus,
            TokenType::Bang,
            TokenType::Hook,
            TokenType::DoubleHook,
            TokenType::DoubleHookEquals,
            TokenType::GreaterThan,
            TokenType::GreaterThanOrEqual,
            TokenType::LessThan,
            TokenType::LessThanOrEqual,
            TokenType::Pipe,
            TokenType::SemiColon,
            TokenType::Hash,
            TokenType::PlusEqual,
            TokenType::MinusEqual,
            TokenType::StarEqual,
            TokenType::SlashEqual,
            TokenType::DoublePlus,
            TokenType::DoubleMinus,
            TokenType::DollarSign,
            TokenType::PipeEqual,
            TokenType::AmpersandEqual,
            TokenType::CaretEquals,
            TokenType::Tilde,
            TokenType::BitShiftLeft,
            TokenType::BitShiftRight,
            TokenType::AtSign,
        ],
    )
}

#[test]
fn non_standard_utf8() {
    harness_single("🦆", TokenType::Invalid("🦆"))
}

#[test]
fn non_standard_utf8_ending_comment() {
    harness_multi("// 🦆", [TokenType::Comment("// 🦆")])
}

#[test]
fn empty_comment() {
    harness_multi("//", [TokenType::Comment("//")])
}

#[test]
fn constants() {
    for constant in MISC_GML_CONSTANTS.iter() {
        assert_eq!(
            Lexer::new(constant).next().map(|t| t.token_type),
            Some(TokenType::MiscConstant(constant))
        );
    }
}

#[test]
fn builtin_variables() {
    for var in MISC_GML_VARIABLES.iter() {
        assert_eq!(
            Lexer::new(var).next().map(|t| t.token_type),
            Some(TokenType::Identifier(var))
        );
    }
}
