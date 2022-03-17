use std::fmt::Display;

use crate::parse::{
    Access, Assignment, AssignmentOp, Ast, Expr, ExprType, Grouping, Identifier, Literal, ParseVisitor, Postfix, Stmt,
    StmtType, Unary, UnaryOp,
};
use colored::Colorize;
use hashbrown::HashMap;
#[derive(Debug, Default)]
pub struct TypeWriter {
    pub scope: HashMap<String, Marker>,
    pub constraints: Vec<Constraint>,
    pub substitutions: HashMap<Marker, Symbol>,
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
        // important to understand this rev is not needed, it just makes things operate the way I do it on
        // paper, which makes it easier to debug
        self.constraints.reverse();
        while let Some(Constraint(marker, symbol)) = self.constraints.pop() {
            self.substitutions.insert(marker, symbol.clone());
            for Constraint(test_marker, test_symbol) in self.constraints.iter_mut() {
                let previous = format!("{} => {}", test_marker, test_symbol).bright_black();
                if Self::update_symbol(test_symbol, marker, &symbol) {
                    self.substitutions.insert(*test_marker, test_symbol.clone());
                    println!(
                        "[{} = {}]: {previous} => {} => {}",
                        marker.to_string().bright_cyan(),
                        format!("{test_marker}").bright_blue(),
                        test_marker.to_string().bright_cyan(),
                        format!("{test_symbol}").bright_blue()
                    )
                }
            }
        }

        // With the results, update our ast
        for stmt in ast.stmts_mut() {
            self.finalize_stmt(stmt);
        }
    }

    fn update_symbol(symbol: &mut Symbol, marker: Marker, new_target: &Symbol) -> bool {
        match symbol {
            Symbol::Variable(inner_marker) => {
                if *inner_marker == marker {
                    *symbol = new_target.clone();
                    true
                } else {
                    false
                }
            }
            Symbol::Application(application) => match application {
                Application::Array(inner_symbol) => Self::update_symbol(inner_symbol, marker, new_target),
                Application::Object(fields) => {
                    let mut any = false;
                    for (_, field_symbol) in fields {
                        if Self::update_symbol(field_symbol, marker, new_target) {
                            any = true;
                        }
                    }
                    any
                }
            },
            Symbol::Deref(deref) => match deref {
                Deref::Array(inner_marker) => {
                    if *inner_marker == marker {
                        match new_target {
                            Symbol::Variable(new_marker) => *inner_marker = *new_marker,
                            Symbol::Application(Application::Array(inner_symbol)) => {
                                *symbol = inner_symbol.as_ref().clone()
                            }
                            _ => panic!("cannot access {inner_marker} with a {new_target}"),
                        }
                        true
                    } else {
                        false
                    }
                }
                Deref::Object(inner_marker, key) => {
                    if *inner_marker == marker {
                        match new_target {
                            Symbol::Variable(new_marker) => *inner_marker = *new_marker,
                            Symbol::Application(Application::Object(fields)) => {
                                *symbol = fields.get(key).expect("couldn't find a field on a struct").clone()
                            }
                            _ => panic!("cannot access {inner_marker} with a {new_target}"),
                        }
                        true
                    } else {
                        false
                    }
                }
            },
            Symbol::Constant(_) => false,
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
                    if !self.scope.contains_key(&iden.lexeme) {
                        self.scope.insert(iden.lexeme.clone(), left.marker);
                        self.register_substitution(left, Symbol::Variable(right.marker));
                    } else {
                        // validate that the new type is equal to the last type? shadowing is a
                        // problem
                    }
                }
            }
            StmtType::Expr(_) => {
                // todo: named functions. this feels wrong?
            }

            // todo: constrain types required for these things
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
                        self.register_substitution(
                            right,
                            Symbol::Deref(Deref::Object(
                                left.marker,
                                right.inner().as_identifier().unwrap().lexeme.clone(),
                            )),
                        );
                        self.register_substitution(expr, Symbol::Variable(right.marker));
                    }
                    Access::Array {
                        left,
                        index_one,
                        index_two,
                        ..
                    } => {
                        // our indexes must be real
                        self.register_substitution(index_one, Symbol::Constant(Type::Real));
                        if let Some(index_two) = index_two {
                            self.register_substitution(index_two, Symbol::Constant(Type::Real));
                        }

                        // meanwhile, our type is a deref of the left's type
                        self.register_substitution(expr, Symbol::Deref(Deref::Array(left.marker)));
                    }
                    Access::Struct { left, key } => {}
                    _ => todo!(),
                }
            }
            ExprType::Call(_) => {
                // access the call's type from scope, validate arg types, then use its return value.
            }
            ExprType::Grouping(Grouping { inner, .. }) => {
                self.register_substitution(expr, Symbol::Variable(inner.marker));
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
                let symbol = match literal {
                    Literal::True | Literal::False => Symbol::Constant(Type::Bool),
                    Literal::Undefined => Symbol::Constant(Type::Undefined),
                    Literal::Noone => Symbol::Constant(Type::Noone),
                    Literal::String(_) => Symbol::Constant(Type::String),
                    Literal::Real(_) | Literal::Hex(_) => Symbol::Constant(Type::Real),
                    Literal::Array(exprs) => {
                        // Infer the type based on the first member
                        if let Some(marker) = exprs.first().map(|expr| expr.marker) {
                            Symbol::Application(Application::Array(Box::new(Symbol::Variable(marker))))
                        } else {
                            Symbol::Application(Application::Array(Box::new(Symbol::Constant(Type::Unknown)))) // todo will this resolve?
                        }
                    }
                    Literal::Struct(declarations) => {
                        // We can construct a type for this since we'll know the structure of the fields,
                        // even if we don't know the type of the fields themselves
                        let mut fields = HashMap::default();
                        for declaration in declarations {
                            fields.insert(declaration.0.lexeme.clone(), Symbol::Variable(declaration.1.marker));
                        }
                        Symbol::Application(Application::Object(fields))
                    }
                    Literal::Misc(_) => Symbol::Constant(Type::Unknown),
                };
                self.register_substitution(expr, symbol);
            }
        }
    }

    fn alias_stmt(&mut self, stmt: &mut Stmt) {
        stmt.visit_child_stmts_mut(|stmt| self.alias_stmt(stmt));
        stmt.visit_child_exprs_mut(|expr| self.alias_expr(expr));
    }

    fn alias_expr(&mut self, expr: &mut Expr) {
        expr.marker.0 = self.alias_iter;
        println!(
            "{}: {}",
            expr.marker.to_string().bright_cyan(),
            expr.to_string().bright_white().bold(),
        );
        self.alias_iter += 1;
        expr.visit_child_stmts_mut(|stmt| self.alias_stmt(stmt));
        expr.visit_child_exprs_mut(|expr| self.alias_expr(expr));
    }

    fn finalize_stmt(&mut self, stmt: &mut Stmt) {
        stmt.visit_child_stmts_mut(|stmt| self.finalize_stmt(stmt));
        stmt.visit_child_exprs_mut(|expr| self.finalize_expr(expr));
    }

    fn finalize_expr(&mut self, expr: &mut Expr) {
        expr.tpe = self.resolve_type(expr.marker);
        expr.visit_child_stmts_mut(|stmt| self.finalize_stmt(stmt));
        expr.visit_child_exprs_mut(|expr| self.finalize_expr(expr));
    }

    fn register_substitution(&mut self, expr: &Expr, symbol: Symbol) {
        let substitution = Constraint(expr.marker, symbol);
        println!("{substitution}");
        self.constraints.push(substitution);
    }

    pub fn resolve_type(&self, mut marker: Marker) -> Type {
        let mut tpe = Type::Unknown;
        while let Some(symbol) = self.substitutions.get(&marker) {
            match symbol {
                Symbol::Variable(new_marker) => marker = *new_marker,
                symbol => {
                    tpe = Type::from(symbol.clone());
                    break;
                }
            }
        }
        tpe
    }
}

#[derive(Debug, Clone)]
pub struct Constraint(Marker, Symbol);
impl Display for Constraint {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.pad(&format!(
            "{} = {}",
            self.0.to_string().bright_cyan(),
            format!("{}", self.1).bright_blue()
        ))
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Symbol {
    Constant(Type),
    Variable(Marker),
    Application(Application),
    Deref(Deref),
}
impl Display for Symbol {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Symbol::Constant(tpe) => f.pad(&tpe.to_string()),
            Symbol::Variable(marker) => f.pad(&marker.to_string()),
            Symbol::Application(application) => f.pad(&application.to_string()),
            Symbol::Deref(deref) => f.pad(&deref.to_string()),
        }
    }
}
impl From<Symbol> for Type {
    fn from(symbol: Symbol) -> Self {
        match symbol {
            Symbol::Constant(tpe) => tpe,
            Symbol::Variable(_) => Type::Unknown,
            Symbol::Application(app) => match app {
                Application::Array(inner_symbol) => Type::Array(Box::new(Type::from(inner_symbol.as_ref().to_owned()))),
                Application::Object(fields) => {
                    let mut tpe_fields = HashMap::new();
                    for (name, symbol) in fields {
                        tpe_fields.insert(name, symbol.into());
                    }
                    Type::Struct(tpe_fields)
                }
            },
            Symbol::Deref(_) => Type::Unknown,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Application {
    Array(Box<Symbol>),
    Object(HashMap<String, Symbol>),
}
impl Display for Application {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Application::Array(symbol) => f.pad(&format!("[{symbol}]")),
            Application::Object(fields) => {
                f.pad("{")?;
                for (name, symbol) in fields.iter() {
                    f.pad(&format!(" {name}: {symbol},"))?;
                }
                f.pad(" }")
            }
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Deref {
    Array(Marker),
    Object(Marker, String),
}
impl Display for Deref {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Deref::Array(marker) => f.pad(&format!("*{marker}")),
            Deref::Object(marker, field) => f.pad(&format!("{marker}.{}", field)),
        }
    }
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy, Default)]
pub struct Marker(pub u64);
impl Marker {
    pub fn new() -> Self {
        Self(rand::random())
    }
}
impl Display for Marker {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.pad(&format!("t{}", self.0))
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
    Array(Box<Type>),
    /// A struct with the given fields
    Struct(HashMap<String, Type>),
}
impl Display for Type {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Type::Unknown => f.pad("<?>"),
            Type::Undefined => f.pad("Undefined"),
            Type::Noone => f.pad("Noone"),
            Type::Bool => f.pad("Bool"),
            Type::Real => f.pad("Real"),
            Type::String => f.pad("String"),
            Type::Array(inner) => f.pad(&format!("[{}]", *inner)),
            Type::Struct(fields) => {
                f.pad("{")?;
                for (name, symbol) in fields.iter() {
                    f.pad(&format!(" {name}: {symbol},"))?;
                }
                f.pad(" }")
            }
        }
    }
}
