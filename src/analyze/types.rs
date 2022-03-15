use crate::parse::{Assignment, AssignmentOp, Ast, Expr, ExprType, Identifier, ParseVisitor, Stmt, StmtType};
use hashbrown::HashMap;

#[derive(Debug, Default)]
pub struct TypeWriter {
    scope: HashMap<String, Marker>,
    mappings: HashMap<Marker, Type>,
}
impl TypeWriter {
    pub fn write_types(&mut self, ast: &mut Ast) {
        for stmt in ast.stmts_mut() {
            self.visit_stmt(stmt);
        }
    }

    fn visit_stmt(&mut self, stmt: &mut Stmt) {
        stmt.visit_child_stmts_mut(|stmt| self.visit_stmt(stmt));
        stmt.visit_child_exprs_mut(|expr| self.visit_expr(expr));

        if let StmtType::Assignment(Assignment {
            left,
            operator: AssignmentOp::Identity(_),
            ..
        }) = stmt.inner_mut()
        {
            if let ExprType::Identifier(iden) = left.inner_mut() {
                if let Some(t) = self.scope.get(&iden.lexeme) {
                    // Map this expr's type to the already listed one
                    left.tpe = Type::Marked(*t);
                } else {
                    // This is new, so add it to the scope
                    let marker = Marker::new();
                    self.scope.insert(iden.lexeme.clone(), marker);
                    left.tpe = Type::Marked(marker);
                }
            }
        }
    }

    fn visit_expr(&mut self, expr: &mut Expr) {
        expr.visit_child_stmts_mut(|stmt| self.visit_stmt(stmt));
        expr.visit_child_exprs_mut(|expr| self.visit_expr(expr));

        match expr.inner() {
            ExprType::Identifier(iden) => {}

            // Literals can have their types inferenced
            ExprType::Literal(literal) => {
                expr.tpe = if let Some(core) = literal.as_core_type() {
                    core
                } else {
                    Type::Marked(Marker::new())
                };
            }
            _ => {
                expr.tpe = Type::Marked(Marker::new());
            }
        }
    }
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
pub struct Marker(u64);
impl Marker {
    pub fn new() -> Self {
        Self(rand::random())
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Type {
    Unknown,
    Marked(Marker),

    /// The GM constant value of `undefined`
    Undefined,
    /// The GM constant value of `noone`
    Noone,
    /// True or false
    Bool,
    /// A number
    Real,
    /// A string of text
    String,
    /// An array containing values of the nested type
    Array(Box<Type>),
    /// A struct with the given fields
    Object(HashMap<String, Type>),
}
