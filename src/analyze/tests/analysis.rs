use crate::{
    analyze::{Constraint, Marker, Symbol, TypeWriter},
    parse::*,
};
use colored::Colorize;
use hashbrown::HashSet;
use pretty_assertions::assert_eq;

fn harness_type(source: &'static str) {
    let parser = Parser::new(source, 0);
    let mut type_writer = TypeWriter::default();
    let mut ast = parser.into_ast().unwrap();
    type_writer.write_types(&mut ast);

    let mut reported: HashSet<Marker> = HashSet::new();
    fn visit_expr(expr: &Expr, reported: &mut HashSet<Marker>, type_writer: &TypeWriter, source: &'static str) {
        let str = format!(
            "{} {}",
            format!("'{}:", expr.marker.0).bright_black(),
            expr.to_string().bright_white(),
        );
        let whitespace = String::from_utf8(vec![b' '; 75 - str.len()]).unwrap();
        let tpe = type_writer
            .constraints
            .iter()
            .filter_map(|Constraint::Eq(marker, symbol)| if marker == &expr.marker { Some(symbol) } else { None })
            .flat_map(|symbol| match symbol {
                Symbol::Constant(t) => Some(t),
                _ => None,
            })
            .next()
            .unwrap();
        println!("{str}{whitespace}{:?}\n", tpe,);

        expr.visit_child_stmts(|stmt| visit_stmt(stmt, reported, type_writer, source));
        expr.visit_child_exprs(|expr| visit_expr(expr, reported, type_writer, source));
    }
    fn visit_stmt(stmt: &Stmt, reported: &mut HashSet<Marker>, type_writer: &TypeWriter, source: &'static str) {
        stmt.visit_child_stmts(|stmt| visit_stmt(stmt, reported, type_writer, source));
        stmt.visit_child_exprs(|expr| visit_expr(expr, reported, type_writer, source));
    }
    println!("Result for: {source}");
    for stmt in ast.stmts() {
        visit_stmt(stmt, &mut reported, &type_writer, source);
    }
    todo!();
}

#[test]
fn test() {
    harness_type(
        "
        var foo = [0, 1, 2];
        var bar = foo[0];
    ",
    )
}
