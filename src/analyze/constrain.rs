use super::{App, Deref, Impl, Marker, Page, Printer, Scope, Term, Type};
use crate::parse::{
    Access, Assignment, AssignmentOp, Block, Call, Equality, Evaluation, Expr, ExprType, Grouping, Literal,
    LocalVariableSeries, Logical, NullCoalecence, OptionalInitilization, ParseVisitor, Postfix, Return, Stmt, StmtType,
    Ternary, Unary, UnaryOp,
};
use colored::Colorize;
use hashbrown::HashMap;

#[derive(Debug)]
pub(super) struct Constraints<'s> {
    pub collection: Vec<Constraint>,
    scope: &'s mut Scope,
    printer: &'s mut Printer,
    in_write: bool,
}

// Constraining
impl<'s> Constraints<'s> {
    fn constrain_stmt(&mut self, stmt: &Stmt) {
        if let StmtType::Assignment(Assignment { left, op, right }) = stmt.inner() {
            self.in_write = true;
            stmt.visit_child_stmts(|stmt| self.constrain_stmt(stmt));
            stmt.visit_child_exprs(|expr| self.constrain_expr(expr));
            match left.inner() {
                ExprType::Identifier(iden) => {
                    if let AssignmentOp::Identity(_) = op {
                        if !self.scope.has_field(&iden.lexeme) {
                            self.scope.new_field(iden.lexeme.clone(), left);
                            self.expr_eq_expr(left, right);
                        } else {
                            // validate that the new type is equal to the last type? shadowing is a
                            // problem
                        }
                    }
                }
                ExprType::Access(Access::Dot {
                    left: obj,
                    right: field,
                }) => {
                    let right_marker = self.scope.get_expr_marker(right);
                    self.expr_eq_impl(
                        obj,
                        Impl::Fields(HashMap::from([(field.lexeme.clone(), Term::Marker(right_marker))])),
                    );
                }
                _ => {}
            }
            self.in_write = false;
            return;
        }

        stmt.visit_child_stmts(|stmt| self.constrain_stmt(stmt));
        stmt.visit_child_exprs(|expr| self.constrain_expr(expr));
        match stmt.inner() {
            StmtType::Assignment(_) => {}
            StmtType::LocalVariableSeries(LocalVariableSeries { declarations }) => {
                for initializer in declarations.iter() {
                    if let OptionalInitilization::Uninitialized(expr) = initializer {
                        let iden = initializer.name_identifier();
                        if !self.scope.has_field(&iden.lexeme) {
                            self.scope.new_field(iden.lexeme.clone(), expr);
                            self.expr_eq_type(expr, Type::Undefined);
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

            StmtType::Return(Return { value }) => {
                if let Some(value) = value {
                    let marker = self.scope.get_expr_marker(value);
                    self.marker_eq_symbol(Marker::RETURN_VALUE, Term::Marker(marker));
                } else {
                    self.marker_eq_symbol(Marker::RETURN_VALUE, Term::Type(Type::Undefined));
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
                    self.scope.new_field(iden.lexeme.clone(), expr);
                } else {
                    // validate that the new type is equal to the last type? shadowing is a
                    // problem
                }
            }

            match &function.constructor {
                Some(_) => todo!(),
                None => {
                    let mut body_page = Page::default();
                    for param in function.parameters.iter() {
                        body_page.scope.new_field(param.name(), param.name_expr())
                    }
                    let (parameters, return_type) =
                        App::process_function(function.clone(), &mut body_page, self.printer);
                    self.expr_eq_app(expr, App::Function(parameters, return_type, function.clone()));
                }
            }

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
                        let this_expr_marker = self.scope.get_expr_marker(expr);
                        let left_marker = self.scope.get_expr_marker(left);
                        self.expr_eq_impl(
                            left,
                            Impl::Fields(HashMap::from([(right.lexeme.clone(), Term::Marker(this_expr_marker))])),
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
                        let this_expr_marker = self.scope.get_expr_marker(expr);

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
                                target: Box::new(Term::Marker(this_expr_marker)),
                            },
                        );
                    }
                    Access::Struct { .. } => {}
                    Access::Map { .. } => {}
                    Access::Grid { .. } => {}
                    Access::List { .. } => {}
                }
            }
            ExprType::Call(Call { left, arguments, .. }) => {
                // let imp = Rule::Function(
                //     arguments
                //         .iter()
                //         .map(|v| Term::Marker(self.scope.get_expr_marker(v)))
                //         .collect(),
                //     Box::new(Term::Marker(self.scope.get_expr_marker(expr))),
                // );
                // self.expr_eq_rule(left, imp);
                let left_marker = self.scope.get_expr_marker(left);
                let deref = Deref::Call {
                    target: Box::new(Term::Marker(left_marker)),
                    arguments: arguments
                        .iter()
                        .map(|arg| Term::Marker(self.scope.get_expr_marker(arg)))
                        .collect(),
                };
                self.expr_eq_deref(expr, deref);
            }
            ExprType::Grouping(Grouping { inner, .. }) => {
                self.expr_eq_expr(expr, inner);
            }

            ExprType::Identifier(iden) => {
                if let Ok(marker) = self.scope.field_marker(iden) {
                    self.scope.alias_expr_to_marker(expr, marker);
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
                        let app = if let Some(marker) = exprs.first().map(|expr| self.scope.get_expr_marker(expr)) {
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
                                Term::Marker(self.scope.get_expr_marker(&declaration.1)),
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
    pub fn new(scope: &'s mut Scope, stmts: &[Stmt], printer: &'s mut Printer) -> Self {
        let mut constraints = Self {
            collection: vec![],
            scope,
            printer,
            in_write: false,
        };
        for stmt in stmts.iter() {
            constraints.constrain_stmt(stmt);
        }
        constraints.collection.dedup();
        for (marker, name) in constraints.scope.expr_strings.iter() {
            println!(
                "{}  {} : {}",
                "ALIAS".bright_red(),
                constraints.printer.marker(marker),
                name
            );
            constraints.printer.give_expr_alias(*marker, name.clone());
        }
        for con in constraints.collection.iter() {
            println!("{}", constraints.printer.constraint(con));
        }
        constraints.collection.reverse();
        constraints
    }

    pub fn expr_eq_marker(&mut self, target: &Expr, marker: Marker) {
        self.expr_eq_term(target, Term::Marker(marker))
    }

    pub fn expr_eq_type(&mut self, target: &Expr, tpe: Type) {
        self.expr_eq_term(target, Term::Type(tpe))
    }

    pub fn expr_eq_expr(&mut self, target: &Expr, expr: &Expr) {
        let marker = self.scope.get_expr_marker(expr);
        self.expr_eq_term(target, Term::Marker(marker))
    }

    pub fn expr_eq_app(&mut self, target: &Expr, application: App) {
        self.expr_eq_term(target, Term::App(application))
    }

    pub fn expr_eq_deref(&mut self, target: &Expr, deref: Deref) {
        self.expr_eq_term(target, Term::Deref(deref))
    }

    pub fn expr_eq_impl(&mut self, target: &Expr, imp: Impl) {
        self.expr_eq_term(target, Term::Impl(imp))
    }

    pub fn expr_eq_term(&mut self, expr: &Expr, term: Term) {
        let marker = self.scope.get_expr_marker(expr);
        self.marker_eq_symbol(marker, term);
    }

    pub fn marker_eq_symbol(&mut self, marker: Marker, term: Term) {
        self.collection.push(Constraint { marker, term });
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Constraint {
    pub marker: Marker,
    pub term: Term,
}
