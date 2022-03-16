use crate::parse::{
    Access, Assignment, AssignmentOp, Ast, Expr, ExprType, Grouping, Identifier, Literal, ParseVisitor, Postfix, Stmt,
    StmtType, Unary, UnaryOp,
};
use hashbrown::HashMap;
#[derive(Debug, Default)]
pub struct TypeWriter {
    pub scope: HashMap<String, Marker>,
    pub constraints: Vec<Constraint>,
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
        let mut subs = self.constraints.clone();
        while let Some(Constraint::Eq(marker, target)) = subs.pop() {
            self.substitute(marker, &target);
        }
    }

    fn substitute(&mut self, marker: Marker, target: &Symbol) {
        // Apply this target to all future constraints
        for Constraint::Eq(_, con_tpe) in self.constraints.iter_mut() {
            Self::update_type(con_tpe, marker, target);
        }
    }

    fn update_type(symbol: &mut Symbol, marker: Marker, new_target: &Symbol) {
        match symbol {
            Symbol::Variable(inner_marker) => {
                if *inner_marker == marker {
                    *symbol = new_target.clone();
                }
            }
            Symbol::Constant(inner_tpe) => match inner_tpe {
                Type::Array(inner_symbol) => Self::update_type(inner_symbol.as_mut(), marker, new_target),
                Type::Struct(fields) => {
                    for (_, field_symbol) in fields {
                        Self::update_type(field_symbol, marker, new_target);
                    }
                }
                _ => {}
            },
            Symbol::Deref(inner_marker) => {
                if *inner_marker == marker {
                    match new_target {
                        Symbol::Constant(tpe) => match tpe {
                            Type::Array(inner_symbol) => *symbol = inner_symbol.as_ref().clone(),
                            _ => panic!("cannot deref {:?}", tpe),
                        },
                        Symbol::Variable(new_marker) => *inner_marker = *new_marker,
                        Symbol::Deref(_) => unreachable!("i think?"),
                    }
                }
            }
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
                            .push(Constraint::Eq(left.marker, Symbol::Variable(right.marker)));
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
                        // our indexes must be real
                        self.constraints
                            .push(Constraint::Eq(index_one.marker, Symbol::Constant(Type::Real)));
                        if let Some(index_two) = index_two {
                            self.constraints
                                .push(Constraint::Eq(index_two.marker, Symbol::Constant(Type::Real)));
                        }

                        // and our lhs must be an array
                        // self.constraints
                        //     .push(Constraint::Eq(left.marker, Type::Array(Box::new(Type::Unknown))));

                        // meanwhile, our type is a deref of the left's type
                        self.constraints
                            .push(Constraint::Eq(expr.marker, Symbol::Deref(left.marker)));
                    }
                    Access::Struct { left, key } => {
                        // validate the left is a struct, right is key
                    }
                    _ => todo!(),
                }
            }
            ExprType::Call(_) => {
                // access the call's type from scope, validate arg types, then use its return value.
            }
            ExprType::Grouping(Grouping { inner, .. }) => {
                self.constraints
                    .push(Constraint::Eq(expr.marker, Symbol::Variable(inner.marker)));
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
                            Type::Array(Box::new(Symbol::Variable(marker)))
                        } else {
                            Type::Array(Box::new(Symbol::Constant(Type::Unknown))) // TODO: this will never be resolved
                        }
                    }
                    Literal::Struct(declarations) => {
                        // We can construct a type for this since we'll know the structure of the fields,
                        // even if we don't know the type of the fields themselves
                        let mut fields = HashMap::default();
                        for declaration in declarations {
                            fields.insert(declaration.0.lexeme.clone(), Symbol::Variable(declaration.1.marker));
                        }
                        Type::Struct(fields)
                    }
                    Literal::Misc(_) => Type::Unknown,
                };
                self.constraints
                    .push(Constraint::Eq(expr.marker, Symbol::Constant(tpe)));
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

#[derive(Debug, Clone)]
pub enum Constraint {
    Eq(Marker, Symbol),
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Symbol {
    Constant(Type),
    Variable(Marker),
    Deref(Marker),
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
    Array(Box<Symbol>),
    /// A struct with the given fields
    Struct(HashMap<String, Symbol>),
}
impl std::fmt::Display for Type {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Type::Unknown => f.pad("<?>"),
            Type::Undefined => f.pad("Undefined"),
            Type::Noone => f.pad("Noone"),
            Type::Bool => f.pad("Bool"),
            Type::Real => f.pad("Real"),
            Type::String => f.pad("String"),
            Type::Array(inner) => f.pad(&format!("{:?}[]", *inner)),
            Type::Struct(fields) => {
                f.pad("{")?;
                for (name, symbol) in fields.iter() {
                    f.pad(&format!(" {name}: {symbol:?},"))?;
                }
                f.pad(" }")
            }
        }
    }
}
