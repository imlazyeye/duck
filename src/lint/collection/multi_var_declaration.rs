use codespan_reporting::diagnostic::{Diagnostic, Label};

use crate::{
    lint::{EarlyStatementPass, Lint, LintLevel},
    parse::{LocalVariableSeries, Statement, StatementBox},
    FileId,
};

#[derive(Debug, PartialEq)]
pub struct MultiVarDeclaration;
impl Lint for MultiVarDeclaration {
    fn explanation() -> &'static str {
        "While GML allows you to create multiple local variables at once, it can often lead to confusing syntax that would read better with each variable seperated."
    }

    fn default_level() -> LintLevel {
        LintLevel::Allow
    }

    fn tag() -> &'static str {
        "multi_var_declaration"
    }
}

impl EarlyStatementPass for MultiVarDeclaration {
    fn visit_statement_early(
        statement_box: &StatementBox,
        config: &crate::Config,
        reports: &mut Vec<Diagnostic<FileId>>,
    ) {
        if let Statement::LocalVariableSeries(LocalVariableSeries { declarations }) = statement_box.statement() {
            if declarations.len() > 1 {
                reports.push(
                    Self::diagnostic(config)
                        .with_message("Multiple local variables declared at once")
                        .with_labels(vec![
                            Label::primary(statement_box.file_id(), statement_box.span())
                                .with_message("seperate these into different declarations"),
                        ]),
                );
            }
        }
    }
}
