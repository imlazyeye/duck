use crate::parsing::{
    lexer::{Lexer, MISC_GML_CONSTANTS, MISC_GML_VARIABLES},
    Token,
};
use colored::Colorize;
use pretty_assertions::assert_eq;

fn harness_multi(source: &'static str, expected: impl Into<Vec<Token>>) {
    let outputed = Lexer::new(source).map(|(_, token)| token).collect::<Vec<Token>>();
    let expected = expected.into();
    println!("{}: {}", "Source".yellow(), source);
    assert_eq!(*outputed, expected)
}

fn harness_single(source: &'static str, expected: Token) {
    harness_multi(source, vec![expected]);
}

#[test]
fn whitespace() {
    harness_multi(" \n\t", []);
}

#[test]
fn float() {
    harness_single("0.2", Token::Real(0.2));
}

#[test]
fn float_no_prefix_digit() {
    harness_single(".2", Token::Real(0.2));
}

#[test]
fn int() {
    harness_single("1", Token::Real(1.0));
}

#[test]
fn hex_0x() {
    harness_single("0x1", Token::Hex("1"));
}

#[test]
fn hex_dollar_sign() {
    harness_single("$1", Token::Hex("1"));
}

#[test]
fn invalid_hex() {
    harness_multi("$q", [Token::DollarSign, Token::Identifier("q")]);
}

#[test]
fn string() {
    harness_single("\"foo\"", Token::StringLiteral("foo"));
}

#[test]
fn multiline_string() {
    harness_single("@\"\n\nfoo\"", Token::StringLiteral("\n\nfoo"));
}

#[test]
fn multiline_string_alt() {
    harness_single("@'\n\nfoo'", Token::StringLiteral("\n\nfoo"));
}

#[test]
fn identifier() {
    harness_single("foo", Token::Identifier("foo"));
}

#[test]
fn macro_declaration() {
    harness_single("#macro foo 0", Token::Macro("foo", None, "0"));
}

#[test]
fn macro_declaration_with_config() {
    harness_single("#macro bar:foo 0", Token::Macro("foo", Some("bar"), "0"));
}

#[test]
fn comments() {
    // Note: comments are currently disabled!
    harness_multi("// comment", []);
    harness_multi("/// comment", []);
    harness_multi("/* comment */", []);
    harness_multi("/*\n comment \n*/", []);
}

#[test]
fn lint_tags() {
    harness_single("// #[allow(and_keyword)]", Token::LintTag("allow", "and_keyword"));
    harness_single("// #[warn(and_keyword)]", Token::LintTag("warn", "and_keyword"));
    harness_single("// #[deny(and_keyword)]", Token::LintTag("deny", "and_keyword"));
}

#[test]
fn keywords() {
    harness_multi(
        "switch case break return enum default and or function constructor exit global
        div new mod globalvar try self catch with true false if else while for do until
        repeat var continue static then finally undefined noone not xor",
        [
            Token::Switch,
            Token::Case,
            Token::Break,
            Token::Return,
            Token::Enum,
            Token::Default,
            Token::And,
            Token::Or,
            Token::Function,
            Token::Constructor,
            Token::Exit,
            Token::Global,
            Token::Div,
            Token::New,
            Token::Mod,
            Token::Globalvar,
            Token::Try,
            Token::SelfKeyword,
            Token::Catch,
            Token::With,
            Token::True,
            Token::False,
            Token::If,
            Token::Else,
            Token::While,
            Token::For,
            Token::Do,
            Token::Until,
            Token::Repeat,
            Token::Var,
            Token::Continue,
            Token::Static,
            Token::Then,
            Token::Finally,
            Token::Undefined,
            Token::Noone,
            Token::Not,
            Token::Xor,
        ],
    )
}

#[test]
fn symbols() {
    harness_multi(
        ":.{}()[],& &&||^ = ==!=%%=/ *+-!? ????=> >=< <= |;#+=-=*=/=++--$|=&=^=~<<>>@",
        [
            Token::Colon,
            Token::Dot,
            Token::LeftBrace,
            Token::RightBrace,
            Token::LeftParenthesis,
            Token::RightParenthesis,
            Token::LeftSquareBracket,
            Token::RightSquareBracket,
            Token::Comma,
            Token::Ampersand,
            Token::DoubleAmpersand,
            Token::DoublePipe,
            Token::Circumflex,
            Token::Equal,
            Token::DoubleEqual,
            Token::BangEqual,
            Token::Percent,
            Token::PercentEqual,
            Token::Slash,
            Token::Star,
            Token::Plus,
            Token::Minus,
            Token::Bang,
            Token::Interrobang,
            Token::DoubleInterrobang,
            Token::DoubleInterrobangEquals,
            Token::GreaterThan,
            Token::GreaterThanOrEqual,
            Token::LessThan,
            Token::LessThanOrEqual,
            Token::Pipe,
            Token::SemiColon,
            Token::Hash,
            Token::PlusEqual,
            Token::MinusEqual,
            Token::StarEqual,
            Token::SlashEqual,
            Token::DoublePlus,
            Token::DoubleMinus,
            Token::DollarSign,
            Token::PipeEqual,
            Token::AmpersandEqual,
            Token::CirumflexEqual,
            Token::Tilde,
            Token::BitShiftLeft,
            Token::BitShiftRight,
            Token::AtSign,
        ],
    )
}

#[test]
fn non_standard_utf8() {
    harness_single("🦆", Token::Invalid("🦆"))
}

#[test]
fn non_standard_utf8_ending_comment() {
    harness_multi("// 🦆", [])
}

#[test]
fn constants() {
    for constant in MISC_GML_CONSTANTS.iter() {
        assert_eq!(
            Lexer::new(constant).next().map(|(_, t)| t),
            Some(Token::MiscConstant(constant))
        );
    }
}

#[test]
fn builtin_variables() {
    for constant in MISC_GML_VARIABLES.iter() {
        if constant == &"self" {
            // self is a little strange... FIXME...
            continue;
        }
        assert_eq!(
            Lexer::new(constant).next().map(|(_, t)| t),
            Some(Token::Identifier(constant))
        );
    }
}
