use super::{Application, Constraint, Deref, Marker, Symbol, Type};
use crate::parse::{
    Access, Assignment, AssignmentOp, Ast, Equality, Evaluation, Expr, ExprType, Grouping, Literal,
    LocalVariableSeries, Logical, NullCoalecence, OptionalInitilization, ParseVisitor, Postfix, Stmt, StmtType,
    Ternary, Unary, UnaryOp,
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
        while let Some(Constraint { marker, symbol }) = self.constraints.pop() {
            self.substitutions.insert(marker, symbol.clone());
            for Constraint {
                marker: test_marker,
                symbol: test_symbol,
            } in self.constraints.iter_mut()
            {
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
                        self.register_constraint(left, Symbol::Variable(right.marker));
                    } else {
                        // validate that the new type is equal to the last type? shadowing is a
                        // problem
                    }
                }
            }
            StmtType::LocalVariableSeries(LocalVariableSeries { declarations }) => {
                for initializer in declarations.iter() {
                    if let OptionalInitilization::Uninitialized(expr) = initializer {
                        if !self.scope.contains_key(initializer.name()) {
                            self.scope.insert(initializer.name().into(), expr.marker);
                        } else {
                            // validate that the new type is equal to the last type? shadowing is a
                            // problem
                        }
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
            ExprType::Logical(Logical { left, right, .. }) => {
                self.register_constraint(left, Symbol::Constant(Type::Bool));
                self.register_constraint(right, Symbol::Constant(Type::Bool));
                self.register_constraint(expr, Symbol::Constant(Type::Bool));
            }
            ExprType::Equality(Equality { left, right, .. }) => {
                self.register_constraint(left, Symbol::Variable(right.marker));
                self.register_constraint(right, Symbol::Variable(left.marker));
                self.register_constraint(expr, Symbol::Constant(Type::Bool));
            }
            ExprType::Evaluation(Evaluation { left, right, .. }) => {
                self.register_constraint(left, Symbol::Variable(right.marker));
                self.register_constraint(right, Symbol::Variable(left.marker));
                self.register_constraint(expr, Symbol::Variable(left.marker));
            }
            ExprType::NullCoalecence(NullCoalecence { left, right }) => {
                self.register_constraint(left, Symbol::Variable(right.marker));
                self.register_constraint(right, Symbol::Variable(left.marker));
                self.register_constraint(expr, Symbol::Variable(left.marker));
            }
            ExprType::Ternary(Ternary {
                condition,
                true_value,
                false_value,
            }) => {
                self.register_constraint(condition, Symbol::Constant(Type::Bool));
                self.register_constraint(true_value, Symbol::Variable(false_value.marker));
                self.register_constraint(false_value, Symbol::Variable(true_value.marker));
                self.register_constraint(expr, Symbol::Variable(true_value.marker));
            }
            ExprType::Unary(Unary { op, right }) => match op {
                UnaryOp::Increment(_)
                | UnaryOp::Decrement(_)
                | UnaryOp::Positive(_)
                | UnaryOp::Negative(_)
                | UnaryOp::BitwiseNot(_) => {
                    self.register_constraint(right, Symbol::Constant(Type::Real));
                    self.register_constraint(expr, Symbol::Constant(Type::Real));
                }
                UnaryOp::Not(_) => {
                    self.register_constraint(right, Symbol::Constant(Type::Bool));
                    self.register_constraint(expr, Symbol::Constant(Type::Bool));
                }
            },
            ExprType::Postfix(Postfix { left, .. }) => {
                self.register_constraint(left, Symbol::Constant(Type::Real));
                self.register_constraint(expr, Symbol::Constant(Type::Real));
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
                        self.register_constraint(
                            right,
                            Symbol::Deref(Deref::Object(
                                left.marker,
                                right.inner().as_identifier().unwrap().lexeme.clone(),
                            )),
                        );
                        self.register_constraint(expr, Symbol::Variable(right.marker));
                    }
                    Access::Array {
                        left,
                        index_one,
                        index_two,
                        ..
                    } => {
                        // our indexes must be real
                        self.register_constraint(index_one, Symbol::Constant(Type::Real));
                        if let Some(index_two) = index_two {
                            self.register_constraint(index_two, Symbol::Constant(Type::Real));
                        }

                        // meanwhile, our type is a deref of the left's type
                        self.register_constraint(expr, Symbol::Deref(Deref::Array(left.marker)));
                    }
                    Access::Struct { left, key } => {}
                    _ => todo!(),
                }
            }
            ExprType::Call(_) => {
                // access the call's type from scope, validate arg types, then use its return value.
            }
            ExprType::Grouping(Grouping { inner, .. }) => {
                self.register_constraint(expr, Symbol::Variable(inner.marker));
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
                self.register_constraint(expr, symbol);
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

    fn register_constraint(&mut self, expr: &Expr, symbol: Symbol) {
        let substitution = Constraint {
            marker: expr.marker,
            symbol,
        };
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
