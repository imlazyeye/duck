use crate::prelude::{Expression, ExpressionBox, IntoExpressionBox, ParseVisitor, StatementBox};

/// Representation of a access in gml, such as an array lookup, or dot-notation.
#[derive(Debug, PartialEq, Clone)]
pub enum Access {
    /// Accessing the global scope via `global.`.
    Global { right: ExpressionBox },
    /// Accessing the current scope via `self`. (This would be called `Self`, but its reserved by
    /// rust.)
    Current { right: ExpressionBox },
    /// Dot access with any struct or object.
    Dot { left: ExpressionBox, right: ExpressionBox },
    /// Array access. The bool at the end represents if the `@` accessor is present, which denotes
    /// the access to be direct instead of copy-on-write.
    ///
    /// Please note that this syntax does not ultimately decide what runtime
    /// behavior actually gets applied: GameMaker has added the option to make *all* array
    /// accesses copy-on-write, potentially marking this syntax for deprecation in the future.
    ///
    /// Both variants have an optional second value, since 2d arrays are still supported (though
    /// deprecated).
    Array {
        left: ExpressionBox,
        index_one: ExpressionBox,
        index_two: Option<ExpressionBox>,
        using_accessor: bool,
    },
    /// Ds Map access.
    Map { left: ExpressionBox, key: ExpressionBox },
    /// Ds Grid access. C
    Grid {
        left: ExpressionBox,
        index_one: ExpressionBox,
        index_two: ExpressionBox,
    },
    /// Ds List access.
    List { left: ExpressionBox, index: ExpressionBox },
    /// Struct access. This is not dot-notation, this is specifically when the user uses `foo[$
    /// "bar"]`.
    Struct { left: ExpressionBox, key: ExpressionBox },
}
impl From<Access> for Expression {
    fn from(access: Access) -> Self {
        Self::Access(access)
    }
}
impl IntoExpressionBox for Access {}
impl ParseVisitor for Access {
    fn visit_child_expressions<E: FnMut(&ExpressionBox)>(&self, _expression_visitor: E) {}
    fn visit_child_statements<S: FnMut(&StatementBox)>(&self, _statement_visitor: S) {}
}
