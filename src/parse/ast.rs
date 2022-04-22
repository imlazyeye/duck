use crate::parse::ParseVisitor;

use super::{Expr, Stmt};

/// A collection of statements.
#[derive(Debug, Clone)]
pub struct Ast {
    stmts: Vec<Stmt>,
}
impl Ast {
    /// Creates a new Ast with the given statements.
    pub fn new(stmts: Vec<Stmt>) -> Self {
        Self { stmts }
    }

    /// Consumes the Ast into its inner collection of statements.
    pub fn unpack(self) -> Vec<Stmt> {
        self.stmts
    }

    /// Get a reference to the ast's statements.
    pub fn stmts(&self) -> &[Stmt] {
        self.stmts.as_ref()
    }

    /// Get a mutable reference to the ast's statements.
    pub fn stmts_mut(&mut self) -> &mut Vec<Stmt> {
        &mut self.stmts
    }

    /// Returns all nodes in the ast that match the provided tag.
    pub fn tagged_nodes(&self, tag: &Tag) -> Vec<Node> {
        fn visit_expr(expr: &Expr, nodes: &mut Vec<Node>, tag: &Tag) {
            if expr.tag() == Some(tag) {
                nodes.push(Node::Expr(expr.clone()));
            }
            expr.visit_child_stmts(|stmt| visit_stmt(stmt, nodes, tag));
            expr.visit_child_exprs(|expr| visit_expr(expr, nodes, tag));
        }
        fn visit_stmt(stmt: &Stmt, nodes: &mut Vec<Node>, tag: &Tag) {
            if stmt.tag() == Some(tag) {
                nodes.push(Node::Stmt(stmt.clone()));
            }
            stmt.visit_child_stmts(|stmt| visit_stmt(stmt, nodes, tag));
            stmt.visit_child_exprs(|expr| visit_expr(expr, nodes, tag));
        }
        let mut nodes = vec![];
        for stmt in self.stmts() {
            visit_stmt(stmt, &mut nodes, tag);
        }
        nodes
    }
}

/// An identifier for an individual Ast.
#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy, Default)]
pub struct AstId(u64);

/// The data from a user-written tag (ie: #[allow(draw_text)])
#[derive(Debug, Clone, PartialEq)]
pub struct Tag(pub String, pub Option<String>);
impl PartialEq<(&str, Option<&str>)> for Tag {
    fn eq(&self, other: &(&str, Option<&str>)) -> bool {
        self.0 == other.0 && self.1.as_ref().map(|v| v.as_str()) == other.1
    }
}

/// A container for both expressions and statements.
#[derive(Debug, Clone, PartialEq)]
pub enum Node {
    Stmt(Stmt),
    Expr(Expr),
}
