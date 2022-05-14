use crate::parse::{Expr, ExprKind, IntoExpr, ParseVisitor, Stmt};

use super::Identifier;

/// Representation of a access in gml, such as an array lookup, or dot-notation.
#[derive(Debug, PartialEq, Clone, serde::Serialize)]
#[serde(rename_all = "snake_case", tag = "access")]
pub enum Access {
    /// Accessing the global scope via `global.`.
    Global {
        /// The value being extracted from the global scope.
        #[serde(flatten)]
        right: Identifier,
    },
    /// Accessing the current scope via `self`. (This would be called `Self`, but its reserved by
    /// rust.)
    Identity {
        /// The value being extracted from the local scope.
        #[serde(flatten)]
        right: Identifier,
    },
    /// Accessing the scope above the current one via `other`.
    Other {
        /// The value being extracted from the other scope.
        #[serde(flatten)]
        right: Identifier,
    },
    /// Dot access with any struct or object.
    Dot {
        /// The value being accessed.
        left: Expr,
        /// The value being extracted from the leftside value.
        #[serde(flatten)]
        right: Identifier,
    },
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
        /// The array being accessed.
        left: Expr,
        /// The first index supplied to the access.
        index_one: Expr,
        /// The second index, if using 2d accessing.
        index_two: Option<Expr>,
        /// Whether or not the `@` was provided.
        using_accessor: bool,
    },
    /// Ds Map access.
    Map {
        /// The map being accessed.
        left: Expr,
        /// The key used to access the map.
        key: Expr,
    },
    /// Ds Grid access.
    Grid {
        /// The grid being accessed.
        left: Expr,
        /// The `x` value being passed in.
        index_one: Expr,
        /// The `y` value being passed in.
        index_two: Expr,
    },
    /// Ds List access.
    List {
        /// The list being accessed.
        left: Expr,
        /// The index being accessed out of the list.
        index: Expr,
    },
    /// Struct access. This is not dot-notation, this is specifically when the user uses `foo[$
    /// "bar"]`.
    Struct {
        /// The struct being accessed.
        left: Expr,
        /// The key being used to access the struct.
        key: Expr,
    },
}
impl From<Access> for ExprKind {
    fn from(access: Access) -> Self {
        Self::Access(access)
    }
}
impl IntoExpr for Access {}
impl ParseVisitor for Access {
    fn visit_child_exprs<E: FnMut(&Expr)>(&self, mut visitor: E) {
        match self {
            Access::Global { .. } | Access::Identity { .. } | Access::Other { .. } => {}
            Access::Dot { left, .. } => visitor(left),
            Access::Map { left, key: right }
            | Access::List { left, index: right }
            | Access::Struct { left, key: right } => {
                visitor(left);
                visitor(right);
            }
            Access::Grid {
                left,
                index_one,
                index_two,
            } => {
                visitor(left);
                visitor(index_one);
                visitor(index_two);
            }
            Access::Array {
                left,
                index_one,
                index_two,
                using_accessor: _,
            } => {
                visitor(left);
                visitor(index_one);
                if let Some(index_two) = index_two {
                    visitor(index_two);
                }
            }
        }
    }
    fn visit_child_exprs_mut<E: FnMut(&mut Expr)>(&mut self, mut visitor: E) {
        match self {
            Access::Global { .. } | Access::Identity { .. } | Access::Other { .. } => {}
            Access::Dot { left, .. } => visitor(left),
            Access::Map { left, key: right }
            | Access::List { left, index: right }
            | Access::Struct { left, key: right } => {
                visitor(left);
                visitor(right);
            }
            Access::Grid {
                left,
                index_one,
                index_two,
            } => {
                visitor(left);
                visitor(index_one);
                visitor(index_two);
            }
            Access::Array {
                left,
                index_one,
                index_two,
                using_accessor: _,
            } => {
                visitor(left);
                visitor(index_one);
                if let Some(index_two) = index_two {
                    visitor(index_two);
                }
            }
        }
    }

    fn visit_child_stmts<S: FnMut(&Stmt)>(&self, mut _visitor: S) {}
    fn visit_child_stmts_mut<S: FnMut(&mut Stmt)>(&mut self, _visitor: S) {}
}
