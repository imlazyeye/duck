use super::*;
use crate::parse::*;
use hashbrown::HashMap;

#[derive(Debug)]
pub(super) struct Constraints<'s> {
    pub collection: Vec<Constraint>,
    scope: &'s mut Scope,
    typewriter: &'s mut Typewriter,
    context: Context,
}

// Constraining
impl<'s> Constraints<'s> {
    fn constrain_stmt(&mut self, stmt: &Stmt) {
        self.context = match stmt.inner() {
            StmtType::Assignment(Assignment { op, .. }) => match op {
                AssignmentOp::Identity(_) => Context::Declare,
                _ => Context::Write,
            },
            _ => Context::Read,
        };

        match stmt.inner() {
            StmtType::Assignment(Assignment { left, right, .. }) => {
                self.expr_eq_expr(left, right);
            }
            StmtType::LocalVariableSeries(LocalVariableSeries { declarations }) => {
                for initializer in declarations.iter() {
                    let expr = initializer.name_expr();
                    let iden = initializer.name_identifier();
                    if !self.scope.has_local_field(&iden.lexeme) {
                        self.scope.declare_local(iden.lexeme.clone(), expr);
                    }
                    match initializer {
                        OptionalInitilization::Uninitialized(_) => self.expr_eq_type(expr, Type::Undefined),
                        OptionalInitilization::Initialized(_) => {
                            let assign = initializer.assignment_value().unwrap();
                            self.expr_eq_expr(expr, assign);
                        }
                    }
                }
            }

            StmtType::Return(Return { value }) => {
                if let Some(value) = value {
                    let marker = self.scope.ensure_alias(value);
                    self.marker_eq_term(Marker::RETURN, Term::Marker(marker));
                } else {
                    self.marker_eq_term(Marker::RETURN, Term::Type(Type::Undefined));
                }
            }

            // todo: constrain types required for these things
            _ => {}
        }

        stmt.visit_child_stmts(|stmt| self.constrain_stmt(stmt));
        stmt.visit_child_exprs(|expr| self.constrain_expr(expr));
    }

    fn constrain_expr(&mut self, expr: &Expr) {
        if let ExprType::FunctionDeclaration(function) = expr.inner() {
            // If this is a named function, declare it to the scope
            if let Some(iden) = &function.name {
                let marker = self.scope.ensure_alias(expr);
                self.marker_impl(
                    self.scope.self_marker,
                    Trait::FieldOp(iden.lexeme.clone(), Box::new(FieldOp::Writable(Term::Marker(marker)))),
                );
            }

            println!("\n--- Processing function... ---\n");

            // Grab the inheritance if its there
            if let Some(Constructor::WithInheritance(expr)) = &function.constructor {
                self.constrain_expr(expr);
            }

            // Create a new scope for this function
            let mut writer = self.typewriter.clone();
            let mut scope = if function.constructor.is_some() {
                Scope::new(&mut writer)
            } else {
                Scope::shallow_new(self.scope)
            };

            // Declare the parameters into the scope, collecting their markers
            #[allow(clippy::needless_collect)]
            let param_registrations: Vec<Marker> = function
                .parameters
                .iter()
                .map(|param| scope.declare_local(param.name().into(), param.name_expr()))
                .collect();

            // Access the body
            let body = function.body_stmts();

            // Typewrite the function
            writer.write(&body, &mut scope);
            println!("\n--- Ending process... ---\n");

            // Re-collect the parameters
            let parameters: Vec<Term> = param_registrations
                .into_iter()
                .map(|marker| writer.find_term(marker))
                .collect();

            // Retrieve any fields this function's self must implement
            let self_fields = writer.self_fields_mut(&scope);
            let self_fields = if self_fields.is_empty() {
                None
            } else {
                Some(self_fields.clone())
            };

            let return_type = writer.take_return_term();
            self.expr_eq_app(
                expr,
                App::Function {
                    self_fields,
                    parameters,
                    return_type: Box::new(return_type),
                },
            );

            // We return, as *we* handeled visiting the children.
            return;
        }

        expr.visit_child_stmts(|stmt| self.constrain_stmt(stmt));
        expr.visit_child_exprs(|expr| self.constrain_expr(expr));
        match expr.inner() {
            ExprType::FunctionDeclaration(_) => {}
            ExprType::Logical(Logical { left, right, .. }) => {
                self.expr_eq_type(left, Type::Bool);
                self.expr_eq_type(right, Type::Bool);
                self.expr_eq_type(expr, Type::Bool);
            }
            ExprType::Equality(Equality { left, right, .. }) => {
                self.expr_eq_expr(right, left);
                self.expr_eq_type(expr, Type::Bool);
            }
            ExprType::Evaluation(Evaluation { left, right, .. }) => {
                self.expr_eq_expr(right, left);
                self.expr_eq_expr(expr, left);
            }
            ExprType::NullCoalecence(NullCoalecence { left, right }) => {
                self.expr_eq_expr(right, left);
                self.expr_eq_expr(expr, left);
            }
            ExprType::Ternary(Ternary {
                condition,
                true_value,
                false_value,
            }) => {
                self.expr_eq_type(condition, Type::Bool);
                self.expr_eq_expr(false_value, true_value);
                self.expr_eq_expr(expr, true_value);
            }
            ExprType::Unary(Unary { op, right }) => match op {
                UnaryOp::Increment(_)
                | UnaryOp::Decrement(_)
                | UnaryOp::Positive(_)
                | UnaryOp::Negative(_)
                | UnaryOp::BitwiseNot(_) => {
                    self.expr_eq_type(right, Type::Real);
                    self.expr_eq_type(expr, Type::Real);
                }
                UnaryOp::Not(_) => {
                    self.expr_eq_type(right, Type::Bool);
                    self.expr_eq_type(expr, Type::Bool);
                }
            },
            ExprType::Postfix(Postfix { left, .. }) => {
                self.expr_eq_type(left, Type::Real);
                self.expr_eq_type(expr, Type::Real);
            }
            ExprType::Access(access) => {
                match access {
                    Access::Global { .. } => {
                        // read the global scope for the type?
                    }
                    Access::Current { right } => {
                        let marker = self.scope.ensure_alias(expr);
                        if self.context == Context::Declare {
                            self.marker_impl(
                                self.scope.self_marker,
                                Trait::FieldOp(right.lexeme.clone(), Box::new(FieldOp::Writable(Term::Marker(marker)))),
                            );
                        } else {
                            self.marker_impl(
                                self.scope.self_marker,
                                Trait::FieldOp(right.lexeme.clone(), Box::new(FieldOp::Readable(Term::Marker(marker)))),
                            );
                        }
                    }
                    Access::Other { .. } => {
                        // read the above scope for the type?
                    }
                    Access::Dot { left, right } => {
                        let this_expr_marker = self.scope.ensure_alias(expr);
                        self.expr_impl(
                            left,
                            Trait::FieldOp(
                                right.lexeme.clone(),
                                Box::new(if self.context != Context::Read {
                                    FieldOp::Writable(Term::Marker(this_expr_marker))
                                } else {
                                    FieldOp::Readable(Term::Marker(this_expr_marker))
                                }),
                            ),
                        );
                    }
                    Access::Array {
                        left,
                        index_one,
                        index_two,
                        ..
                    } => {
                        let this_expr_marker = self.scope.ensure_alias(expr);

                        // our indexes must be real
                        self.expr_eq_type(index_one, Type::Real);
                        if let Some(index_two) = index_two {
                            self.expr_eq_type(index_two, Type::Real);
                        }

                        // the left must be an array of the member
                        self.expr_eq_app(left, App::Array(Box::new(Term::Marker(this_expr_marker))));
                    }
                    Access::Struct { .. } => {}
                    Access::Map { .. } => {}
                    Access::Grid { .. } => {}
                    Access::List { .. } => {}
                }
            }
            ExprType::Call(Call {
                left,
                arguments,
                uses_new,
            }) => {
                let this_expr_marker = self.scope.ensure_alias(expr);

                // Create a function based on what we know
                let arguments: Vec<Term> = arguments
                    .iter()
                    .map(|arg| Term::Marker(self.scope.ensure_alias(arg)))
                    .collect();

                // Make sure the left can implement this call
                self.expr_impl(
                    left,
                    Trait::Callable {
                        arguments,
                        expected_return: Box::new(Term::Marker(this_expr_marker)),
                        uses_new: *uses_new,
                    },
                );
            }
            ExprType::Grouping(Grouping { inner, .. }) => {
                self.expr_eq_expr(expr, inner);
            }

            ExprType::Identifier(iden) => {
                if let Ok(marker) = self.scope.lookup_marker(iden) {
                    self.scope.alias_expr_to(expr, marker);
                } else if self.context != Context::Read {
                    // Check if a local variable for this already exists
                    if self.scope.has_local_field(&iden.lexeme) {
                        // do nothing for now
                    } else {
                        let marker = self.scope.ensure_alias(expr);
                        self.marker_impl(
                            self.scope.self_marker,
                            Trait::FieldOp(iden.lexeme.clone(), Box::new(FieldOp::Writable(Term::Marker(marker)))),
                        );
                    }
                } else {
                    let marker = self.scope.ensure_alias(expr);
                    self.marker_impl(
                        self.scope.self_marker,
                        Trait::FieldOp(iden.lexeme.clone(), Box::new(FieldOp::Readable(Term::Marker(marker)))),
                    );
                }
            }
            ExprType::Literal(literal) => {
                let tpe = match literal {
                    Literal::True | Literal::False => Type::Bool,
                    Literal::Undefined => Type::Undefined,
                    Literal::Noone => Type::Noone,
                    Literal::String(_) => Type::Str,
                    Literal::Real(_) | Literal::Hex(_) => Type::Real,
                    Literal::Misc(_) => Type::Any,
                    Literal::Array(exprs) => {
                        // Infer the type based on the first member
                        let app = if let Some(marker) = exprs.first().map(|expr| self.scope.ensure_alias(expr)) {
                            App::Array(Box::new(Term::Marker(marker)))
                        } else {
                            App::Array(Box::new(Term::Type(Type::Any))) // todo will this resolve?
                        };
                        self.expr_eq_app(expr, app);
                        return;
                    }
                    Literal::Struct(declarations) => {
                        // We can construct a type for this since we'll know the structure of the fields,
                        // even if we don't know the type of the fields themselves
                        let mut fields = HashMap::default();
                        for declaration in declarations {
                            fields.insert(
                                declaration.0.lexeme.clone(),
                                FieldOp::Readable(Term::Marker(self.scope.ensure_alias(&declaration.1))),
                            );
                        }
                        self.expr_eq_app(expr, App::Object(fields));
                        return;
                    }
                };
                self.expr_eq_type(expr, tpe);
            }
        }
    }
}

// Utilities
impl<'s> Constraints<'s> {
    pub fn build(typewriter: &'s mut Typewriter, scope: &'s mut Scope, stmts: &[Stmt]) -> Vec<Constraint> {
        let mut constraints = Self {
            collection: vec![],
            typewriter,
            scope,
            context: Context::Read,
        };
        for stmt in stmts.iter() {
            constraints.constrain_stmt(stmt);
        }
        constraints.collection.dedup();
        for con in constraints.collection.iter() {
            println!("{}", Printer::constraint(con));
        }
        constraints.collection.reverse();
        constraints.collection
    }

    pub fn expr_eq_type(&mut self, target: &Expr, tpe: Type) {
        self.expr_eq_term(target, Term::Type(tpe))
    }

    pub fn expr_eq_expr(&mut self, target: &Expr, expr: &Expr) {
        let marker = self.scope.ensure_alias(expr);
        self.expr_eq_term(target, Term::Marker(marker))
    }

    pub fn expr_eq_app(&mut self, target: &Expr, application: App) {
        self.expr_eq_term(target, Term::App(application))
    }

    pub fn expr_impl(&mut self, target: &Expr, trt: Trait) {
        let marker = self.scope.ensure_alias(target);
        self.marker_impl(marker, trt);
    }

    pub fn expr_eq_term(&mut self, expr: &Expr, term: Term) {
        let marker = self.scope.ensure_alias(expr);
        self.marker_eq_term(marker, term);
    }

    pub fn marker_impl(&mut self, marker: Marker, trt: Trait) {
        self.collection.push(Constraint::Eq(marker, Term::Trait(trt)));
    }

    pub fn marker_eq_term(&mut self, marker: Marker, term: Term) {
        self.collection.push(Constraint::Eq(marker, term));
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Constraint {
    Eq(Marker, Term),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Context {
    Read,
    Write,
    Declare,
}
