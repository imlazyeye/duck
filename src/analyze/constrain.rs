use super::*;
use crate::parse::*;

#[derive(Debug)]
pub(super) struct Constraints<'s> {
    pub collection: Vec<Constraint>,
    typewriter: &'s mut Typewriter,
    errors: Vec<TypeError>,
}

// Constraining
impl<'s> Constraints<'s> {
    fn constrain_stmt(&mut self, stmt: &Stmt) -> Result<(), TypeError> {
        match stmt.inner() {
            StmtType::Assignment(Assignment { left, right, op }) => {
                self.constrain_expr(right)?;
                let right_marker = self.typewriter.marker_for(right)?;
                if let AssignmentOp::Identity(_) = op {
                    match left.inner() {
                        ExprType::Identifier(iden) => {
                            if self.typewriter.locals.contains(&iden.lexeme) {
                                self.typewriter.write_local(&iden.lexeme, left, right_marker)?;
                            } else {
                                self.typewriter.write_self(&iden.lexeme, left, right_marker)?;
                            }
                        }
                        ExprType::Access(Access::Current { right: iden }) => {
                            self.typewriter.write_self(&iden.lexeme, left, right_marker)?;
                        }
                        ExprType::Access(Access::Dot {
                            left: struct_expr,
                            right: iden,
                        }) => {
                            let mut record = Record::writer();
                            record
                                .write_field(
                                    &iden.lexeme,
                                    left.id(),
                                    left.location(),
                                    self.typewriter.marker_for(right)?,
                                )?
                                .apply(self.typewriter)?;
                            self.expr_eq_app(struct_expr, App::Record(record))?;
                        }
                        _ => {
                            self.constrain_expr(left)?;
                        }
                    }
                }
                return Ok(());
            }
            StmtType::LocalVariableSeries(LocalVariableSeries { declarations }) => {
                for initializer in declarations.iter() {
                    let value = match initializer {
                        OptionalInitilization::Uninitialized(_) => Marker::NULL,
                        OptionalInitilization::Initialized(_) => {
                            self.typewriter.marker_for(initializer.assignment_value().unwrap())?
                        }
                    };
                    self.typewriter
                        .write_local(initializer.name(), initializer.name_expr(), value)?;
                }
            }

            StmtType::Return(Return { value }) => {
                if let Some(value) = value {
                    let marker = self.typewriter.marker_for(value)?;
                    self.marker_eq_term(Marker::RETURN, Term::Marker(marker));
                } else {
                    self.marker_eq_term(Marker::RETURN, Term::Type(Type::Undefined));
                }
            }

            // todo: constrain types required for these things
            _ => {}
        }

        stmt.visit_child_stmts(|stmt| {
            if let Err(err) = self.constrain_stmt(stmt) {
                self.errors.push(err);
            }
        });
        stmt.visit_child_exprs(|expr| {
            if let Err(err) = self.constrain_expr(expr) {
                self.errors.push(err);
            }
        });
        Ok(())
    }

    fn constrain_expr(&mut self, expr: &Expr) -> Result<(), TypeError> {
        expr.visit_child_stmts(|stmt| {
            if let Err(err) = self.constrain_stmt(stmt) {
                self.errors.push(err);
            }
        });
        expr.visit_child_exprs(|expr| {
            if let Err(err) = self.constrain_expr(expr) {
                self.errors.push(err);
            }
        });
        match expr.inner() {
            ExprType::FunctionDeclaration(_) => {}
            ExprType::Logical(Logical { left, right, .. }) => {
                self.expr_eq_type(left, Type::Bool)?;
                self.expr_eq_type(right, Type::Bool)?;
                self.expr_eq_type(expr, Type::Bool)?;
            }
            ExprType::Equality(Equality { left, right, .. }) => {
                self.expr_eq_expr(right, left)?;
                self.expr_eq_type(expr, Type::Bool)?;
            }
            ExprType::Evaluation(Evaluation { left, right, .. }) => {
                self.expr_eq_expr(right, left)?;
                self.expr_eq_expr(expr, left)?;
            }
            ExprType::NullCoalecence(NullCoalecence { .. }) => {
                todo!();
            }
            ExprType::Ternary(Ternary {
                condition,
                true_value,
                false_value,
            }) => {
                self.expr_eq_type(condition, Type::Bool)?;
                self.expr_eq_expr(false_value, true_value)?;
                self.expr_eq_expr(expr, true_value)?;
            }
            ExprType::Unary(Unary { op, right }) => match op {
                UnaryOp::Increment(_)
                | UnaryOp::Decrement(_)
                | UnaryOp::Positive(_)
                | UnaryOp::Negative(_)
                | UnaryOp::BitwiseNot(_) => {
                    self.expr_eq_type(right, Type::Real)?;
                    self.expr_eq_type(expr, Type::Real)?;
                }
                UnaryOp::Not(_) => {
                    self.expr_eq_type(right, Type::Bool)?;
                    self.expr_eq_type(expr, Type::Bool)?;
                }
            },
            ExprType::Postfix(Postfix { left, .. }) => {
                self.expr_eq_type(left, Type::Real)?;
                self.expr_eq_type(expr, Type::Real)?;
            }
            ExprType::Access(access) => {
                match access {
                    Access::Global { .. } => {}
                    Access::Current { .. } => {}
                    Access::Other { .. } => {}
                    Access::Dot { left, right } => {
                        let mut record = Record::reader();
                        record
                            .write_field(
                                &right.lexeme,
                                left.id(),
                                left.location(),
                                self.typewriter.marker_for(expr)?,
                            )?
                            .apply(self.typewriter)?;
                        self.expr_eq_app(left, App::Record(record))?;
                    }
                    Access::Array {
                        left,
                        index_one,
                        index_two,
                        ..
                    } => {
                        let expr_marker = self.typewriter.marker_for(expr)?;

                        // our indexes must be real
                        self.expr_eq_type(index_one, Type::Real)?;
                        if let Some(index_two) = index_two {
                            self.expr_eq_type(index_two, Type::Real)?;
                        }

                        // the left must be an array of the member
                        self.expr_eq_app(left, App::Array(Box::new(Term::Marker(expr_marker))))?;
                    }
                    Access::Struct { .. } => {}
                    Access::Map { .. } => {}
                    Access::Grid { .. } => {}
                    Access::List { .. } => {}
                }
            }
            ExprType::Call(Call { .. }) => {
                // let this_expr_marker = self.scope.ensure_alias(expr);

                // // Create a function based on what we know
                // let arguments: Vec<Term> = arguments
                //     .iter()
                //     .map(|arg| Term::Marker(self.scope.ensure_alias(arg)))
                //     .collect();

                // Make sure the left can implement this call
                todo!()
                // self.expr_impl(
                //     left,
                //     Trait::Callable {
                //         arguments,
                //         expected_return: Box::new(Term::Marker(this_expr_marker)),
                //         uses_new: *uses_new,
                //     },
                // );
            }
            ExprType::Grouping(Grouping { inner, .. }) => {
                self.expr_eq_expr(expr, inner)?;
            }

            ExprType::Identifier(..) => {}
            ExprType::Literal(literal) => {
                let tpe = match literal {
                    Literal::True | Literal::False => Type::Bool,
                    Literal::Undefined => Type::Undefined,
                    Literal::Noone => Type::Noone,
                    Literal::String(_) => Type::Str,
                    Literal::Real(_) | Literal::Hex(_) => Type::Real,
                    Literal::Misc(_) => Type::Any, // todo
                    Literal::Array(exprs) => {
                        // Infer the type based on the first member
                        let app = if let Some(marker) = exprs.first().map(|expr| self.typewriter.marker_for(expr)) {
                            let marker = marker?;
                            App::Array(Box::new(Term::Marker(marker)))
                        } else {
                            App::Array(Box::new(Term::Type(Type::Any)))
                        };
                        self.expr_eq_app(expr, app)?;
                        return Ok(());
                    }
                    Literal::Struct(declarations) => {
                        let mut record = Record::default();
                        for declaration in declarations {
                            record
                                .write_field(
                                    &declaration.0.lexeme,
                                    declaration.1.id(),
                                    declaration.1.location(),
                                    self.typewriter.marker_for(&declaration.1)?,
                                )?
                                .apply(self.typewriter)?;
                        }
                        self.expr_eq_app(expr, App::Record(record))?;
                        return Ok(());
                    }
                };
                self.expr_eq_type(expr, tpe)?;
            }
        }

        Ok(())
    }
}

// Utilities
impl<'s> Constraints<'s> {
    pub fn build(typewriter: &'s mut Typewriter, stmts: &[Stmt]) -> Result<Vec<Constraint>, Vec<TypeError>> {
        let mut constraints = Self {
            collection: vec![],
            typewriter,
            errors: vec![],
        };
        for stmt in stmts.iter() {
            if let Err(e) = constraints.constrain_stmt(stmt) {
                constraints.errors.push(e);
            }
        }
        constraints.collection.dedup();
        for con in constraints.collection.iter() {
            println!("{}", Printer::constraint(con, constraints.typewriter));
        }
        constraints.collection.reverse();
        if constraints.errors.is_empty() {
            Ok(constraints.collection)
        } else {
            Err(constraints.errors)
        }
    }

    pub fn expr_eq_type(&mut self, target: &Expr, tpe: Type) -> Result<(), TypeError> {
        self.expr_eq_term(target, Term::Type(tpe))
    }

    pub fn expr_eq_expr(&mut self, target: &Expr, expr: &Expr) -> Result<(), TypeError> {
        let marker = self.typewriter.marker_for(expr)?;
        self.expr_eq_term(target, Term::Marker(marker))
    }

    pub fn expr_eq_app(&mut self, target: &Expr, application: App) -> Result<(), TypeError> {
        self.expr_eq_term(target, Term::App(application))
    }

    pub fn expr_eq_term(&mut self, expr: &Expr, term: Term) -> Result<(), TypeError> {
        let marker = self.typewriter.marker_for(expr)?;
        self.marker_eq_term(marker, term);
        Ok(())
    }

    pub fn marker_eq_term(&mut self, marker: Marker, term: Term) {
        self.collection.push(Constraint::Eq(marker, term));
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Constraint {
    Eq(Marker, Term),
}

// if let ExprType::FunctionDeclaration(function) = expr.inner() {
//     // If this is a named function, declare it to the scope
//     if let Some(iden) = &function.name {
//         let marker = self.scope.ensure_alias(expr);
//         self.marker_impl(
//             self.scope.self_marker,
//             Trait::FieldOp(iden.lexeme.clone(),
// Box::new(FieldOp::Writable(Term::Marker(marker)))),         );
//     }

//     println!("\n--- Processing function... ---\n");

//     // Create a new scope for this function
//     let mut writer = self.typewriter.clone();
//     let mut scope = self.scope.new_child();

//     // Declare the parameters into the scope, collecting their markers
//     #[allow(clippy::needless_collect)]
//     let param_registrations: Vec<Marker> = function
//         .parameters
//         .iter()
//         .map(|param| scope.declare_local(param.name().into(), param.name_expr()))
//         .collect();

//     // Access the body
//     let mut body = function.body_stmts();

//     // If there is inheritance, inject it into the body as normal calls
//     let inheritance = if let Some(Constructor::WithInheritance(expr)) = &function.constructor {
//         let call = if let ExprType::Call(call) = expr.inner().clone() {
//             call
//         } else {
//             unreachable!()
//         };

//         // First ensure that this inheritance call is accessible in the function's scope
//         let identifier = call.left.inner().as_identifier().unwrap().clone();
//         scope.declare_local(identifier.lexeme.clone(), &call.left);

//         // Now inject the call
//         body.insert(
//             0,
//             StmtType::Expr(
//                 ExprType::Call(Call {
//                     left: call.left,
//                     arguments: call.arguments,
//                     uses_new: false, /* we cheat and call the constructor as a normal function so
// it'll
//                                       * modify the calling scope */
//                 })
//                 .into_expr(expr.id(), expr.span(), expr.file_id(), None),
//             )
//             .into_stmt(expr.span(), expr.file_id(), None),
//         );

//         // Finally, return the marker of this expression, which will later be used to
//         // provide the call with the inheritance function
//         Some(identifier)
//     } else {
//         None
//     };

//     // Typewrite the function
//     if let Err(e) = writer.write(&body, &mut scope) {
//         self.errors.extend(e);
//     }
//     println!("\n--- Ending process... ---\n");

//     // Re-collect the parameters
//     let parameters: Vec<Term> = param_registrations
//         .into_iter()
//         .map(|marker| writer.find_term(&marker).cloned().unwrap_or(Term::Marker(marker)))
//         .collect();

//     // Retrieve any fields this function's self must implement
//     let self_fields = writer.find_term(&scope.self_marker).unwrap().as_object().unwrap();
//     let self_fields = if self_fields.is_empty() {
//         None
//     } else {
//         Some(self_fields.clone())
//     };

//     let return_type = writer.take_return_term();
//     self.expr_eq_app(
//         expr,
//         App::Function(super::Function {
//             binding: Some(self.scope.self_marker),
//             inheritance,
//             parameters,
//             return_type: Box::new(return_type),
//         }),
//     );

//     // We return, as *we* handeled visiting the children.
//     return;
// }
