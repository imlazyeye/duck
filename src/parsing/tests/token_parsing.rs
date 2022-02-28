use crate::parsing::{
    lexer::{Lexer, MISC_GML_CONSTANTS, MISC_GML_VARIABLES},
    Token,
};
use colored::Colorize;

#[allow(dead_code)]
fn harness_tokens(source: &str, expected: impl Into<Vec<Token>>) {
    let outputed = Lexer::new(source)
        .map(|(_, token)| token)
        .collect::<Vec<Token>>();
    let expected = expected.into();
    if outputed != expected {
        panic!(
            "\n{}\n\n{}\n\n{}: {:?}\n\n{}: {:?}\n",
            "Failed a test on the following gml: ".yellow().bold(),
            source,
            "Expected".green().bold(),
            expected,
            "Outputed".red().bold(),
            outputed,
        )
    }
}

#[test]
fn overlaps() {
    for constant in MISC_GML_CONSTANTS.iter() {
        assert_eq!(
            Lexer::new(constant).next().map(|(_, t)| t),
            Some(Token::MiscConstant(constant.to_string()))
        );
    }
    for constant in MISC_GML_VARIABLES.iter() {
        if constant == &"self" {
            // self is a little strange... FIXME...
            continue;
        }
        assert_eq!(
            Lexer::new(constant).next().map(|(_, t)| t),
            Some(Token::Identifier(constant.to_string()))
        );
    }
}
