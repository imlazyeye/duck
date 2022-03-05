use crate::parse::{Expression, ExpressionBox, IntoExpressionBox, ParseVisitor, StatementBox};

/// Representation of a access in gml, such as an array lookup, or dot-notation.
#[derive(Debug, PartialEq, Clone)]
pub enum Access {
    /// Accessing the global scope via `global.`.
    Global {
        /// The value being extracted from the global scope.
        right: ExpressionBox,
    },
    /// Accessing the current scope via `self`. (This would be called `Self`, but its reserved by
    /// rust.)
    Current {
        /// The value being extracted from the local scope.
        right: ExpressionBox,
    },
    /// Accessing the scope above the current one via `other`.
    Other {
        /// The value being extracted from the other scope.
        right: ExpressionBox,
    },
    /// Dot access with any struct or object.
    Dot {
        /// The value being accessed.
        left: ExpressionBox,
        /// The value being extracted from the leftside value.
        right: ExpressionBox,
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
        left: ExpressionBox,
        /// The first index supplied to the access.
        index_one: ExpressionBox,
        /// The second index, if using 2d accessing.
        index_two: Option<ExpressionBox>,
        /// Whether or not the `@` was provided.
        using_accessor: bool,
    },
    /// Ds Map access.
    Map {
        /// The map being accessed.
        left: ExpressionBox,
        /// The key used to access the map.
        key: ExpressionBox,
    },
    /// Ds Grid access.
    Grid {
        /// The grid being accessed.
        left: ExpressionBox,
        /// The `x` value being passed in.
        index_one: ExpressionBox,
        /// The `y` value being passed in.
        index_two: ExpressionBox,
    },
    /// Ds List access.
    List {
        /// The list being accessed.
        left: ExpressionBox,
        /// The index being accessed out of the list.
        index: ExpressionBox,
    },
    /// Struct access. This is not dot-notation, this is specifically when the user uses `foo[$
    /// "bar"]`.
    Struct {
        /// The struct being accessed.
        left: ExpressionBox,
        /// The key being used to access the struct.
        key: ExpressionBox,
    },
}
impl From<Access> for Expression {
    fn from(access: Access) -> Self {
        Self::Access(access)
    }
}
impl IntoExpressionBox for Access {}
impl ParseVisitor for Access {
    fn visit_child_expressions<E: FnMut(&ExpressionBox)>(&self, mut expression_visitor: E) {
        match self {
            Access::Global { right } | Access::Current { right } | Access::Other { right } => expression_visitor(right),
            Access::Dot { left, right }
            | Access::Map { left, key: right }
            | Access::List { left, index: right }
            | Access::Struct { left, key: right } => {
                expression_visitor(left);
                expression_visitor(right);
            }
            Access::Grid {
                left,
                index_one,
                index_two,
            } => {
                expression_visitor(left);
                expression_visitor(index_one);
                expression_visitor(index_two);
            }
            Access::Array {
                left,
                index_one,
                index_two,
                using_accessor: _,
            } => {
                expression_visitor(left);
                expression_visitor(index_one);
                if let Some(index_two) = index_two {
                    expression_visitor(index_two);
                }
            }
        }
    }
    fn visit_child_statements<S: FnMut(&StatementBox)>(&self, _statement_visitor: S) {}
}
