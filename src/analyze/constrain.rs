use std::sync::Arc;

use super::*;
use crate::parse::*;
use hashbrown::HashMap;
use parking_lot::Mutex;

#[derive(Debug)]
pub(super) struct Constraints<'s> {
    pub collection: Vec<Constraint>,
    scope: &'s mut Scope,
    typewriter: &'s mut Typewriter,
}

// Constraining
impl<'s> Constraints<'s> {
    fn constrain_stmt(&mut self, stmt: &Stmt) {
        stmt.visit_child_stmts(|stmt| self.constrain_stmt(stmt));
        stmt.visit_child_exprs(|expr| self.constrain_expr(expr));
        match stmt.inner() {
            StmtType::Assignment(Assignment { left, op, right }) => {
                match left.inner() {
                    ExprType::Identifier(iden) => {
                        if let AssignmentOp::Identity(_) = op {
                            // Check if a local variable for this already exists
                            if self.scope.has_local_field(&iden.lexeme) {
                                // do nothing for now
                            } else if !self.scope.has_namespace_field(&iden.lexeme) {
                                self.scope.declare_to_namespace(iden.lexeme.clone(), left);
                            }
                            self.expr_eq_expr(left, right);
                        }
                    }
                    ExprType::Access(Access::Current { right: self_right }) => {
                        if !self.scope.has_namespace_field(&self_right.lexeme) {
                            self.scope.declare_to_namespace(self_right.lexeme.clone(), left);
                        }
                        self.expr_eq_expr(left, right);
                    }
                    ExprType::Access(Access::Dot {
                        left: obj,
                        right: field,
                    }) => {
                        self.expr_impl(
                            obj,
                            Trait::Contains(HashMap::from([(field.lexeme.clone(), Term::Marker(Marker::new()))])), // this feels wrong
                        );
                    }
                    _ => {}
                }
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
            StmtType::Expr(_) => {
                // todo: named functions. this feels wrong?
            }

            StmtType::Return(Return { value }) => {
                if let Some(value) = value {
                    let marker = self.scope.ensure_alias(value);
                    self.marker_eq_symbol(Marker::RETURN, Term::Marker(marker));
                } else {
                    self.marker_eq_symbol(Marker::RETURN, Term::Type(Type::Undefined));
                }
            }

            // todo: constrain types required for these things
            _ => {}
        }
    }

    fn constrain_expr(&mut self, expr: &Expr) {
        if let ExprType::FunctionDeclaration(function) = expr.inner() {
            if let Some(iden) = &function.name {
                if !self.scope.has_field(&iden.lexeme) {
                    self.scope.declare_to_namespace(iden.lexeme.clone(), expr);
                } else {
                    // validate that the new type is equal to the last type? shadowing is a
                    // problem
                }
            }

            let mut func_scope = Scope::new();

            for param in function.parameters.iter() {
                func_scope.declare_local(param.name().into(), param.name_expr());
            }

            let mut body = match function.body.inner() {
                StmtType::Block(Block { body, .. }) => body.clone(),
                _ => unreachable!(),
            };

            if let Some(Constructor::WithInheritance(parent)) = &function.constructor {
                self.constrain_expr(parent);
                body.insert(0, parent.clone().into_stmt_lazy());
            };

            let mut func_writer = self.typewriter.clone();
            println!("\n--- Processing function... ---\n");
            func_writer.write(&mut func_scope, &body);
            println!("\n--- Ending process... ---\n");

            let mut parameters = Vec::new();
            for param in function.parameters.iter() {
                let param_marker = func_scope.ensure_alias(param.name_expr());
                let param_term = func_writer.find_term(param_marker);
                parameters.push((param.name().into(), param_term));
            }

            // If the function has any fields in its namespace, then it is generic over some self
            let namespace_fields = func_scope.namespace_fields();
            let self_parameter = if !namespace_fields.is_empty() {
                let fields = func_scope
                    .namespace_fields()
                    .into_iter()
                    .map(|name| {
                        (
                            name.clone(),
                            func_scope
                                .lookup_term(&Identifier::lazy(name), &func_writer)
                                .expect("not possible"),
                        )
                    })
                    .collect();
                Some(Box::new(Term::Trait(Trait::Contains(fields))))
            } else {
                None
            };

            self.expr_eq_app(
                expr,
                App::Function {
                    self_parameter,
                    parameters,
                    return_type: Box::new(func_writer.return_term()),
                    body: function.body_stmts(),
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
                    Access::Current { .. } => {
                        // read the current scope for the type?
                    }
                    Access::Other { .. } => {
                        // read the above scope for the type?
                    }
                    Access::Dot { left, right } => {
                        let this_expr_marker = self.scope.ensure_alias(expr);
                        let left_marker = self.scope.ensure_alias(left);
                        self.expr_impl(
                            left,
                            Trait::Contains(HashMap::from([(right.lexeme.clone(), Term::Marker(this_expr_marker))])),
                        );

                        // constrain the result of this expression to the field
                        self.expr_eq_deref(
                            expr,
                            Deref::Field {
                                field_name: right.lexeme.clone(),
                                target: Box::new(Term::Marker(left_marker)),
                            },
                        );
                    }
                    Access::Array {
                        left,
                        index_one,
                        index_two,
                        ..
                    } => {
                        let this_expr_marker = self.scope.ensure_alias(expr);
                        let left_marker = self.scope.ensure_alias(left);

                        // our indexes must be real
                        self.expr_eq_type(index_one, Type::Real);
                        if let Some(index_two) = index_two {
                            self.expr_eq_type(index_two, Type::Real);
                        }

                        // the left must be an array of the member
                        self.expr_eq_app(left, App::Array(Box::new(Term::Marker(this_expr_marker))));

                        // constrain the result of this expression to the member
                        self.expr_eq_deref(
                            expr,
                            Deref::MemberType {
                                target: Box::new(Term::Marker(left_marker)),
                            },
                        );
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
                // let trt = Trait::ReturnType(Box::new(Term::Marker(self.scope.ensure_alias(expr))));
                // self.expr_impl(left, trt);
                let left_marker = self.scope.ensure_alias(left);
                let deref = Deref::Call {
                    target: Box::new(Term::Marker(left_marker)),
                    arguments: arguments
                        .iter()
                        .map(|arg| Term::Marker(self.scope.ensure_alias(arg)))
                        .collect(),
                    uses_new: *uses_new,
                };
                self.expr_eq_deref(expr, deref);
            }
            ExprType::Grouping(Grouping { inner, .. }) => {
                self.expr_eq_expr(expr, inner);
            }

            ExprType::Identifier(iden) => {
                if let Ok(marker) = self.scope.lookup_marker(iden) {
                    self.scope.alias_expr_to(expr, marker);
                }
            }
            ExprType::Literal(literal) => {
                let tpe = match literal {
                    Literal::True | Literal::False => Type::Bool,
                    Literal::Undefined => Type::Undefined,
                    Literal::Noone => Type::Noone,
                    Literal::String(_) => Type::String,
                    Literal::Real(_) | Literal::Hex(_) => Type::Real,
                    Literal::Misc(_) => Type::Unknown,
                    Literal::Array(exprs) => {
                        // Infer the type based on the first member
                        let app = if let Some(marker) = exprs.first().map(|expr| self.scope.ensure_alias(expr)) {
                            App::Array(Box::new(Term::Marker(marker)))
                        } else {
                            App::Array(Box::new(Term::Type(Type::Unknown))) // todo will this resolve?
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
                                Term::Marker(self.scope.ensure_alias(&declaration.1)),
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
    pub fn build(scope: &'s mut Scope, typewriter: &'s mut Typewriter, stmts: &[Stmt]) -> Vec<Constraint> {
        let mut constraints = Self {
            collection: vec![],
            typewriter,
            scope,
        };
        for stmt in stmts.iter() {
            constraints.constrain_stmt(stmt);
        }
        constraints.collection.dedup();
        // for (marker, name) in constraints.scope.expr_strings.iter() {
        //     Printer::give_expr_alias(*marker, name.clone());
        // }
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

    pub fn expr_eq_deref(&mut self, target: &Expr, deref: Deref) {
        self.expr_eq_term(target, Term::Deref(deref))
    }

    pub fn expr_impl(&mut self, target: &Expr, trt: Trait) {
        let marker = self.scope.ensure_alias(target);
        self.marker_impl(marker, trt);
    }

    pub fn expr_eq_term(&mut self, expr: &Expr, term: Term) {
        let marker = self.scope.ensure_alias(expr);
        self.marker_eq_symbol(marker, term);
    }

    pub fn marker_impl(&mut self, marker: Marker, trt: Trait) {
        self.collection.push(Constraint::Trait(marker, trt));
    }

    pub fn marker_eq_symbol(&mut self, marker: Marker, term: Term) {
        self.collection.push(Constraint::Eq(marker, term));
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Constraint {
    Eq(Marker, Term),
    Trait(Marker, Trait),
}
