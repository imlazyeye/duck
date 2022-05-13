use crate::parse::ParseVisitor;

use super::{Expr, Stmt};

/// A collection of statements.
#[derive(Debug, Clone, serde::Serialize)]
#[serde(rename_all = "snake_case")]
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

    /// Returns all nodes in the ast that match the provided tag, or have any tag at all if None is
    /// provided.
    pub fn tagged_nodes(&self, tag: Option<&Tag>) -> Vec<Node> {
        fn collect_tagged_exprs(expr: &Expr, nodes: &mut Vec<Node>, tag: Option<&Tag>) {
            let matches = match (tag, expr.tag()) {
                (None, Some(_)) => true,
                (Some(t1), Some(t2)) if t1 == t2 => true,
                _ => false,
            };
            if matches {
                nodes.push(Node::Expr(expr.clone()));
            }
            expr.visit_child_stmts(|stmt| collect_tagged_stmts(stmt, nodes, tag));
            expr.visit_child_exprs(|expr| collect_tagged_exprs(expr, nodes, tag));
        }
        fn collect_tagged_stmts(stmt: &Stmt, nodes: &mut Vec<Node>, tag: Option<&Tag>) {
            let matches = match (tag, stmt.tag()) {
                (None, Some(_)) => true,
                (Some(t1), Some(t2)) if t1 == t2 => true,
                _ => false,
            };
            if matches {
                nodes.push(Node::Stmt(stmt.clone()));
            }
            stmt.visit_child_stmts(|stmt| collect_tagged_stmts(stmt, nodes, tag));
            stmt.visit_child_exprs(|expr| collect_tagged_exprs(expr, nodes, tag));
        }
        let mut nodes = vec![];
        for stmt in self.stmts() {
            collect_tagged_stmts(stmt, &mut nodes, tag);
        }
        nodes
    }
}

/// An identifier for an individual Ast.
#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy, Default, serde::Serialize)]
pub struct AstId(u64);

/// The data from a user-written tag (ie: #[allow(draw_text)])
#[derive(Debug, Clone, PartialEq, serde::Serialize)]
pub struct Tag(pub String, pub Option<String>);
impl PartialEq<(&str, Option<&str>)> for Tag {
    fn eq(&self, other: &(&str, Option<&str>)) -> bool {
        self.0 == other.0 && self.1.as_deref() == other.1
    }
}

/// A container for both expressions and statements.
#[derive(Debug, Clone, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case", tag = "node_kind")]
pub enum Node {
    /// Contains a [Stmt].
    Stmt(Stmt),
    /// Contains an [Expr].
    Expr(Expr),
}
impl Node {
    /// Returns the tag the node contains.
    pub fn tag(&self) -> Option<&Tag> {
        match self {
            Node::Stmt(stmt) => stmt.tag(),
            Node::Expr(expr) => expr.tag(),
        }
    }
}
