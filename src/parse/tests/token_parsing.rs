use crate::parse::{
    lexer::{Lexer, MISC_GML_CONSTANTS, MISC_GML_VARIABLES},
    TokenKind,
};
use pretty_assertions::assert_eq;
use TokenKind::*;
macro_rules! token_test {
     ($name:ident: $src:expr => $($should_be:expr), * $(,)?) => {
        #[cfg(test)]
        #[test]
        fn $name() {
            let expected = vec![$($should_be, )*];
            let outputed = Lexer::new($src)
                .map(|token| token.token_type)
                .collect::<Vec<TokenKind>>();
            assert_eq!(*outputed, expected)
        }
    };
}

token_test!(whitepsace: " \n\t" =>);
token_test!(float: "0.2" => Real(0.2));
token_test!(float_no_prefix_digit: ".2" => Real(0.2));
token_test!(int: "1" => Real(1.0));
token_test!(hex_0x: "0x1" => Hex("1"));
token_test!(hex_dollar_sign: "$1" => Hex("1"));
token_test!(invalid_hex: "$q" => DollarSign, Identifier("q"));
token_test!(string: "\"foo\"" => StringLiteral("foo"));
token_test!(multiline_string: "@\"\n\nfoo\"" => StringLiteral("\n\nfoo"));
token_test!(multiline_string_alt: "@'\n\nfoo'" => StringLiteral("\n\nfoo"));
token_test!(identifier: "foo" => Identifier("foo"));
token_test!(macro_declaration: "#macro foo 0" => Macro("foo", None, "0"));
token_test!(macro_declaration_with_config: "#macro bar:foo 0" => Macro("foo", Some("bar"), "0"));
token_test!(comment: "// comment" => Comment("// comment"));
token_test!(doc_comment: "/// comment" => Comment("/// comment"));
token_test!(star_comment: "/* comment */" => Comment("/* comment */"));
token_test!(multiline_star_comment: "/*\n comment \n*/" => Comment("/*\n comment \n*/"));
token_test!(tag: "// #[enum_string]" => Tag("enum_string", None));
token_test!(tag_extra_slashes: "///// #[enum_string]" => Tag("enum_string", None));
token_test!(tag_freeform_param: "///// #[tag(with random stuff inside!)]" => Tag("tag", Some("with random stuff inside!")));
token_test!(parameter_tag: "// #[allow(and_keyword)]" => Tag("allow", Some("and_keyword")));
token_test!(
    keywords:
    "switch case break return enum default and or function constructor exit global
    div new mod globalvar try self catch with true false if else while for do until
    repeat var continue static then finally undefined noone not xor other delete 
    begin end throw" => 
    Switch, Case, Break, Return, Enum, Default, And, Or, Function, Constructor, Exit, Global,
    Div, New, Mod, Globalvar, Try, SelfKeyword, Catch, With, True, False, If, Else, While, For, Do, Until,
    Repeat, Var, Continue, Static, Then, Finally, Undefined, Noone, Not, Xor, Other, Delete,
    Begin, End, Throw,
);
token_test!(
    symbols:
    ":.{}()[],& &&||^ = ==!=%%=/ *+-!? ????=> >=< <= |;#+=-=*=/=++--$|=&=^=~<<>>@" =>
    Colon, Dot, LeftBrace, RightBrace, LeftParenthesis, RightParenthesis, LeftSquareBracket, RightSquareBracket,
    Comma, Ampersand, DoubleAmpersand, DoublePipe, Caret, Equal, DoubleEqual, BangEqual, Percent, PercentEqual,
    Slash, Star, Plus, Minus, Bang, Hook, DoubleHook, DoubleHookEquals, GreaterThan, GreaterThanOrEqual, LessThan,
    LessThanOrEqual, Pipe, SemiColon, Hash, PlusEqual, MinusEqual, StarEqual, SlashEqual, DoublePlus, DoubleMinus,
    DollarSign, PipeEqual, AmpersandEqual, CaretEquals, Tilde, BitShiftLeft, BitShiftRight, AtSign,
);
token_test!(non_standard_utf8: "🦆" => Invalid("🦆"));
token_test!(non_standard_utf8_ending_comment: "// 🦆" => Comment("// 🦆"));
token_test!(empty_comment: "//" => Comment("//"));

#[test]
fn constants() {
    for constant in MISC_GML_CONSTANTS.iter() {
        assert_eq!(
            Lexer::new(constant).next().map(|t| t.token_type),
            Some(MiscConstant(constant))
        );
    }
}

#[test]
fn builtin_variables() {
    for var in MISC_GML_VARIABLES.iter() {
        assert_eq!(Lexer::new(var).next().map(|t| t.token_type), Some(Identifier(var)));
    }
}
