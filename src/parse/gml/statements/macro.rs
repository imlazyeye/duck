use crate::parse::{Expr, Identifier, IntoStmt, ParseVisitor, Stmt, StmtType};

/// Representation of a macro declaration in gml.
///
/// We currently don't do much with these, as their bodies can be *anything*, including
/// invalid gml. For example:
/// ```gml
/// #macro unsafe if true
/// ```
/// This is a perfectly valid macro in gml since their bodies are just pasted over their references
/// early in the compilation process. In the future, we may add macro unfolding to the parsing
/// process, but for now, they exist in this form mostly just to inform us of their existence.
#[derive(Debug, PartialEq, Clone)]
pub struct Macro {
    /// The name this macro was declared with.
    pub name: Identifier,
    /// The config (if any) the macro is bound to.
    pub config: Option<String>,
    /// The body of the macro, in raw gml.
    pub body: String,
}
impl Macro {
    /// Creates a new macro with the given name and body.
    pub fn new(name: Identifier, body: impl Into<String>) -> Self {
        Self {
            name,
            config: None,
            body: body.into(),
        }
    }

    /// Creates a new configuration-bound macro with the given name and body.
    pub fn new_with_config(name: Identifier, body: impl Into<String>, config: impl Into<String>) -> Self {
        Self {
            name,
            config: Some(config.into()),
            body: body.into(),
        }
    }
}
impl From<Macro> for StmtType {
    fn from(mac: Macro) -> Self {
        Self::MacroDeclaration(mac)
    }
}
impl IntoStmt for Macro {}
impl ParseVisitor for Macro {
    fn visit_child_exprs<E: FnMut(&Expr)>(&self, mut _visitor: E) {}
    fn visit_child_exprs_mut<E: FnMut(&mut Expr)>(&mut self, _visitor: E) {}
    fn visit_child_stmts<S: FnMut(&Stmt)>(&self, mut _visitor: S) {}
    fn visit_child_stmts_mut<S: FnMut(&mut Stmt)>(&mut self, _visitor: S) {}
}
