use crate::parse::{
    Access, Assignment, AssignmentOp, Ast, Expr, ExprType, Grouping, Identifier, Literal, ParseVisitor, Postfix, Stmt,
    StmtType, Unary, UnaryOp,
};
use hashbrown::HashMap;
#[derive(Debug, Default)]
pub struct TypeWriter {
    pub scope: HashMap<String, Marker>,
    pub constraints: Vec<Constraint>,
    pub substitutions: HashMap<Marker, Type>,
    pub alias_iter: u64,
}
impl TypeWriter {
    pub fn write_types(&mut self, ast: &mut Ast) {
        // If we're testing, we'll alias all the markers, just to make things easier to read
        if cfg!(test) {
            for stmt in ast.stmts_mut() {
                self.alias_stmt(stmt);
            }
        }

        // Constrain everything
        for stmt in ast.stmts_mut() {
            self.constrain_stmt(stmt);
        }

        // Sub everything
        while let Some(Constraint::Eq(marker, tpe)) = self.constraints.pop() {
            self.substitute(marker, &tpe);
        }
    }

    fn substitute(&mut self, marker: Marker, new_tpe: &Type) {
        // We accept this new constraint to be a new rule, and thus apply it to all our substitutions
        for (_, target) in self.substitutions.iter_mut() {
            Self::update_type(target, marker, new_tpe);
        }

        // We additionally must apply this new constraint to all future constraints
        for Constraint::Eq(_, con_tpe) in self.constraints.iter_mut() {
            Self::update_type(con_tpe, marker, new_tpe);
        }

        // Now having applied this substituion, we add it to the collection for future constraints
        self.substitutions.insert(marker, new_tpe.clone());
    }

    fn update_type(tpe: &mut Type, marker: Marker, new_tpe: &Type) {
        match tpe {
            Type::Marker(_) => {
                if let Type::Marker(inner_marker) = tpe {
                    if *inner_marker == marker {
                        *tpe = new_tpe.clone();
                    }
                }
            }
            Type::Array(inner_tpe) => {
                Self::update_type(inner_tpe.as_mut(), marker, new_tpe);
            }
            Type::Struct(fields) => {
                for (_, field_tpe) in fields {
                    Self::update_type(field_tpe, marker, new_tpe);
                }
            }
            _ => {}
        }
    }

    fn constrain_stmt(&mut self, stmt: &mut Stmt) {
        stmt.visit_child_stmts_mut(|stmt| self.constrain_stmt(stmt));
        stmt.visit_child_exprs_mut(|expr| self.constrain_expr(expr));

        match stmt.inner_mut() {
            StmtType::Assignment(Assignment {
                left,
                op: AssignmentOp::Identity(_),
                right,
            }) => {
                if let ExprType::Identifier(iden) = left.inner_mut() {
                    // Is this assignment new to the scope?
                    if !self.scope.contains_key(&iden.lexeme) {
                        // Register it to the scope
                        self.scope.insert(iden.lexeme.clone(), left.marker);

                        // Constrain it to the rhs
                        self.constraints
                            .push(Constraint::Eq(left.marker, Type::Marker(right.marker)));
                    } else {
                        // validate that the new type is equal to the last type? shadowing is a
                        // problem
                    }
                }
            }
            StmtType::Expr(_) => {
                // todo: named functions. this feels wrong?
            }

            // todo: validate types required for these things
            _ => {}
        }
    }

    fn constrain_expr(&mut self, expr: &mut Expr) {
        expr.visit_child_stmts_mut(|stmt| self.constrain_stmt(stmt));
        expr.visit_child_exprs_mut(|expr| self.constrain_expr(expr));

        match expr.inner() {
            ExprType::FunctionDeclaration(_) => {
                // will have to inference the type of the arguments, and create a function type out
                // of that
            }
            ExprType::Logical(_) => {
                // validate lhs and rhs are bool, result is bool
            }
            ExprType::Equality(_) => {
                // validate lhs and rhs are equal, result is bool
            }
            ExprType::Evaluation(_) => {
                // validate lhs and rhs are equal. lhs decides the resulting type
            }
            ExprType::NullCoalecence(_) => {
                // validate lhs and rhs are compatible. their combo is the result
            }
            ExprType::Ternary(_) => {
                // validate condition is bool and lhs and rhs are compatible, result is their combo
            }
            ExprType::Unary(Unary { op, .. }) => match op {
                UnaryOp::Increment(_) | UnaryOp::Decrement(_) | UnaryOp::Positive(_) | UnaryOp::Negative(_) => {
                    // validate expr is real, result is real
                }
                UnaryOp::Not(_) => {
                    // validate expr is bool, result is bool
                }
                UnaryOp::BitwiseNot(_) => {
                    // validate expr is real, result is real
                }
            },
            ExprType::Postfix(_) => {
                // validate expr is real, result is real
            }
            ExprType::Access(access) => {
                match access {
                    Access::Global { right } => {
                        // read the global scope for the type?
                    }
                    Access::Current { right } => {
                        // read the current scope for the type?
                    }
                    Access::Other { right } => {
                        // read the above scope for the type?
                    }
                    Access::Dot { left, right } => {
                        // identify the left's scope and read the type of the right?
                    }
                    Access::Array {
                        left,
                        index_one,
                        index_two,
                        using_accessor,
                    } => {
                        // validate indexes are real, left is array. result is inner
                    }
                    Access::Map { left, key } => {
                        // validate key is string, left is a ds_map. result is inner
                    }
                    Access::Grid {
                        left,
                        index_one,
                        index_two,
                    } => {
                        // validate the indexes are real, left is grid. result is inner
                    }
                    Access::List { left, index } => {
                        // validate the index is real, left is list
                    }
                    Access::Struct { left, key } => {
                        // validate the left is a struct, right is key
                    }
                }
            }
            ExprType::Call(_) => {
                // access the call's type from scope, validate arg types, then use its return value.
            }
            ExprType::Grouping(Grouping { inner, .. }) => {
                self.constraints
                    .push(Constraint::Eq(expr.marker, Type::Marker(inner.marker)));
            }

            ExprType::Identifier(iden) => {
                // if this identifier is already in scope, then we need to remap this to the previous declaration
                if let Some(marker) = self.scope.get(&iden.lexeme) {
                    expr.marker = *marker;
                } else {
                    // if its not in scope, then we can't constrain it to anything
                }
            }
            ExprType::Literal(literal) => {
                let tpe = match literal {
                    Literal::True | Literal::False => Type::Bool,
                    Literal::Undefined => Type::Undefined,
                    Literal::Noone => Type::Noone,
                    Literal::String(_) => Type::String,
                    Literal::Real(_) | Literal::Hex(_) => Type::Real,
                    Literal::Array(exprs) => {
                        // Infer the type based on the first member
                        if let Some(marker) = exprs.first().map(|expr| expr.marker) {
                            Type::Array(Box::new(Type::Marker(marker)))
                        } else {
                            Type::Array(Box::new(Type::Unknown))
                        }
                    }
                    Literal::Struct(declarations) => {
                        // We can construct a type for this since we'll know the structure of the fields,
                        // even if we don't know the type of the fields themselves
                        let mut fields = HashMap::default();
                        for declaration in declarations {
                            fields.insert(declaration.0.lexeme.clone(), Type::Marker(declaration.1.marker));
                        }
                        Type::Struct(fields)
                    }
                    Literal::Misc(_) => Type::Unknown,
                };
                println!("marking {} as {:?}", expr.marker.0, tpe);
                self.constraints.push(Constraint::Eq(expr.marker, tpe));
            }
        }
    }

    fn alias_stmt(&mut self, stmt: &mut Stmt) {
        stmt.visit_child_stmts_mut(|stmt| self.alias_stmt(stmt));
        stmt.visit_child_exprs_mut(|expr| self.alias_expr(expr));
    }

    fn alias_expr(&mut self, expr: &mut Expr) {
        expr.marker.0 = self.alias_iter;
        self.alias_iter += 1;
        expr.visit_child_stmts_mut(|stmt| self.alias_stmt(stmt));
        expr.visit_child_exprs_mut(|expr| self.alias_expr(expr));
    }
}

#[derive(Debug)]
pub enum Constraint {
    Eq(Marker, Type),
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy, Default)]
pub struct Marker(pub u64);
impl Marker {
    pub fn new() -> Self {
        Self(rand::random())
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Type {
    /// We do not know the type of this symbol
    Unknown,
    /// This type defers to the type that corresponds to the inner Marker.
    Marker(Marker),
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
    Struct(HashMap<String, Type>),
}
impl std::fmt::Display for Type {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Type::Unknown => f.pad("<?>"),
            Type::Marker(n) => f.pad(&format!("<Marker: {}>", n.0)),
            Type::Undefined => f.pad("Undefined"),
            Type::Noone => f.pad("Noone"),
            Type::Bool => f.pad("Bool"),
            Type::Real => f.pad("Real"),
            Type::String => f.pad("String"),
            Type::Array(inner) => f.pad(&format!("{}[]", *inner)),
            Type::Struct(fields) => {
                f.pad("{")?;
                for (name, value) in fields.iter() {
                    f.pad(&format!(" {}: {},", name, value))?;
                }
                f.pad(" }")
            }
        }
    }
}
