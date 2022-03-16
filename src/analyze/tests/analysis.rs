use crate::{analyze::TypeWriter, parse::*};
use colored::Colorize;
use pretty_assertions::assert_eq;

fn harness_type(source: &'static str) {
    let parser = Parser::new(source, 0);
    let mut type_writer = TypeWriter::default();
    let mut ast = parser.into_ast().unwrap();
    type_writer.write_types(&mut ast);
    fn visit_expr(expr: &Expr, type_writer: &TypeWriter, source: &'static str) {
        let str = format!(
            "{} {}",
            format!("'{}:", expr.marker.0).bright_black(),
            expr.to_string().bright_white(),
        );
        let whitespace = String::from_utf8(vec![b' '; 75 - str.len()]).unwrap();
        println!(
            "{str}{whitespace}{}\n",
            type_writer
                .substitutions
                .get(&expr.marker)
                .unwrap()
                .to_string()
                .bright_cyan()
        );
        expr.visit_child_stmts(|stmt| visit_stmt(stmt, type_writer, source));
        expr.visit_child_exprs(|expr| visit_expr(expr, type_writer, source));
    }
    fn visit_stmt(stmt: &Stmt, type_writer: &TypeWriter, source: &'static str) {
        stmt.visit_child_stmts(|stmt| visit_stmt(stmt, type_writer, source));
        stmt.visit_child_exprs(|expr| visit_expr(expr, type_writer, source));
    }
    println!("Result for: {source}");
    for stmt in ast.stmts() {
        visit_stmt(stmt, &type_writer, source);
    }
    todo!();
}

#[test]
fn test() {
    harness_type(
        "
        var foo = 1;
        var bar = 2;
        var fizz = { a: \"hello\", b: [foo, 2], c: bar };
        var buzz = fizz;
    ",
    )
}
