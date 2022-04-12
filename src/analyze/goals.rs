use super::*;
use crate::parse::*;

impl Solver {
    pub fn process_statements(&mut self, stmts: &[Stmt]) -> Result<(), Vec<TypeError>> {
        let mut errors = vec![];
        for stmt in stmts.iter() {
            if let Err(e) = self.visit_stmt(stmt, &mut errors, &mut None) {
                errors.push(e);
            }
        }
        if errors.is_empty() { Ok(()) } else { Err(errors) }
    }
}

// Goal construction
impl Solver {
    fn visit_stmt(
        &mut self,
        stmt: &Stmt,
        errors: &mut Vec<TypeError>,
        struct_literal_override: &mut Option<Var>,
    ) -> Result<(), TypeError> {
        match stmt.inner() {
            StmtType::Assignment(Assignment { left, right, op }) => {
                self.visit_expr(right, errors, struct_literal_override)?;
                let right_ty = self.canonize(right)?;
                let origin = self.local_var();
                if let AssignmentOp::Identity(_) = op {
                    match left.inner() {
                        ExprType::Identifier(iden) => {
                            let scope = if self.local_scope().contains(&iden.lexeme) {
                                self.local_scope_mut()
                            } else if self.global_scope().contains(&iden.lexeme) {
                                self.global_scope_mut()
                            } else {
                                self.self_scope_mut()
                            };
                            scope
                                .apply_field(&iden.lexeme, Field::write(right_ty, stmt.location(), origin))?
                                .commit(self)?;
                        }
                        ExprType::Access(Access::Current { right: iden }) => {
                            self.self_scope_mut()
                                .apply_field(&iden.lexeme, Field::write(right_ty, stmt.location(), origin))?
                                .commit(self)?;
                        }
                        ExprType::Access(Access::Global { right: iden }) => {
                            self.global_scope_mut()
                                .apply_field(&iden.lexeme, Field::write(right_ty, stmt.location(), origin))?
                                .commit(self)?;
                        }
                        ExprType::Access(Access::Dot {
                            left: struct_expr,
                            right: iden,
                        }) => {
                            let mut record = Record::inferred();
                            let right_ty = self.canonize(right)?;
                            record
                                .apply_field(&iden.lexeme, Field::write(right_ty, stmt.location(), self.local_var()))?
                                .commit(self)?;
                            self.expr_eq_ty(struct_expr, Ty::Record(record))?;
                        }
                        _ => {
                            self.visit_expr(left, errors, struct_literal_override)?;
                        }
                    }
                }
                return Ok(());
            }
            StmtType::LocalVariableSeries(LocalVariableSeries { declarations }) => {
                for initializer in declarations.iter() {
                    let ty = match initializer {
                        OptionalInitilization::Uninitialized(_) => Ty::Null,
                        OptionalInitilization::Initialized(_) => {
                            self.canonize(initializer.assignment_value().unwrap())?
                        }
                    };
                    let origin = self.local_var();
                    self.local_scope_mut()
                        .apply_field(initializer.name(), Field::write(ty, stmt.location(), origin))?
                        .commit(self)?;
                }
            }
            StmtType::GlobalvarDeclaration(Globalvar { name }) => {
                let origin = self.local_var();
                self.global_scope_mut()
                    .apply_field(&name.lexeme, Field::write(Ty::Null, stmt.location(), origin))?
                    .commit(self)?;
            }
            StmtType::Return(Return { value }) => {
                if let Some(value) = value {
                    let mut ty = self.canonize(value)?;
                    self.unify_tys(&mut ty, &mut Ty::Var(Var::Return))?;
                } else {
                    self.unify_tys(&mut Ty::Undefined, &mut Ty::Var(Var::Return))?;
                }
            }
            StmtType::WithLoop(WithLoop { .. }) => {
                // TODO: Instance ID / Object ID!
            }
            StmtType::RepeatLoop(RepeatLoop { tick_counts, .. }) => {
                self.expr_eq_ty(tick_counts, Ty::Real)?;
            }
            StmtType::ForLoop(ForLoop { condition, .. })
            | StmtType::DoUntil(DoUntil { condition, .. })
            | StmtType::WhileLoop(WhileLoop { condition, .. }) => {
                self.expr_eq_ty(condition, Ty::Bool)?;
            }
            StmtType::If(If { condition, .. }) => {
                self.expr_eq_ty(condition, Ty::Bool)?;
            }
            StmtType::Switch(Switch {
                matching_value, cases, ..
            }) => {
                let mut identity = self.canonize(matching_value)?;
                for case in cases {
                    let mut case_identity = self.canonize(case.identity())?;
                    self.unify_tys(&mut identity, &mut case_identity)?;
                }
            }
            _ => {}
        }

        stmt.visit_child_stmts(|stmt| {
            if let Err(err) = self.visit_stmt(stmt, errors, struct_literal_override) {
                errors.push(err);
            }
        });
        stmt.visit_child_exprs(|expr| {
            if let Err(err) = self.visit_expr(expr, errors, struct_literal_override) {
                errors.push(err);
            }
        });
        Ok(())
    }

    fn visit_expr(
        &mut self,
        expr: &Expr,
        errors: &mut Vec<TypeError>,
        struct_literal_override: &mut Option<Var>,
    ) -> Result<(), TypeError> {
        match expr.inner() {
            ExprType::Function(function) => {
                let expr_ty = self.canonize(expr)?;
                if let Some(struct_literal_override) = struct_literal_override {
                    self.enter_self_scope(*struct_literal_override)
                }
                if let Some(name) = &function.name {
                    let origin = self.local_var();
                    self.self_scope_mut()
                        .apply_field(&name.lexeme, Field::write(expr_ty, expr.location(), origin))?
                        .commit(self)?;
                }
                self.process_function(expr, function)?;
                if struct_literal_override.is_some() {
                    self.depart_self_scope();
                }
                return Ok(());
            }
            ExprType::Enum(e) => {
                let mut fields = vec![];
                for init in e.members.iter() {
                    if let Some(value) = init.assignment_value() {
                        self.visit_expr(value, errors, struct_literal_override)?;
                        self.expr_eq_ty(value, Ty::Real)?;
                    }
                    fields.push((
                        init.name().to_string(),
                        Field::write(Ty::Real, init.name_expr().location(), self.local_var()),
                    ))
                }
                let expr_ty = self.canonize(expr)?;
                let origin = self.local_var();
                self.global_scope_mut()
                    .apply_field(&e.name.lexeme, Field::constant(expr_ty, expr.location(), origin))?
                    .commit(self)?;
                let record = Ty::Record(Record::concrete(fields, self)?);
                return self.expr_eq_ty(expr, record);
            }
            ExprType::Macro(Macro { name, .. }) => {
                let origin = self.local_var();
                return self
                    .global_scope_mut()
                    .apply_field(&name.lexeme, Field::constant(Ty::Any, expr.location(), origin))?
                    .commit(self);
            }
            ExprType::Literal(Literal::Struct(declarations)) => {
                let mut record = Record::extendable();
                for declaration in declarations {
                    let ty = self.canonize(&declaration.1)?;
                    record
                        .apply_field(
                            &declaration.0.lexeme,
                            Field::write(ty, expr.location(), self.local_var()),
                        )?
                        .commit(self)?;
                }
                self.expr_eq_ty(expr, Ty::Record(record))?;
                let expr_var = self.var_for_expr(expr);
                *struct_literal_override = Some(expr_var);
                for declaration in declarations {
                    self.visit_expr(&declaration.1, errors, struct_literal_override)?;
                }
                *struct_literal_override = None;
                return Ok(());
            }
            _ => (),
        }
        expr.visit_child_stmts(|stmt| {
            if let Err(err) = self.visit_stmt(stmt, errors, struct_literal_override) {
                errors.push(err);
            }
        });
        expr.visit_child_exprs(|expr| {
            if let Err(err) = self.visit_expr(expr, errors, struct_literal_override) {
                errors.push(err);
            }
        });
        match expr.inner() {
            ExprType::Enum(_) | ExprType::Macro(_) | ExprType::Function(_) => unreachable!(),
            ExprType::Logical(Logical { left, right, .. }) => {
                self.expr_eq_ty(left, Ty::Bool)?;
                self.expr_eq_ty(right, Ty::Bool)?;
                self.expr_eq_ty(expr, Ty::Bool)?;
            }
            ExprType::Equality(Equality { left, right, .. }) => {
                self.expr_eq_expr(right, left)?;
                self.expr_eq_ty(expr, Ty::Bool)?;
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
                self.expr_eq_ty(condition, Ty::Bool)?;
                self.expr_eq_expr(false_value, true_value)?;
                self.expr_eq_expr(expr, true_value)?;
            }
            ExprType::Unary(Unary { op, right }) => match op {
                UnaryOp::Increment(_)
                | UnaryOp::Decrement(_)
                | UnaryOp::Positive(_)
                | UnaryOp::Negative(_)
                | UnaryOp::BitwiseNot(_) => {
                    self.expr_eq_ty(right, Ty::Real)?;
                    self.expr_eq_ty(expr, Ty::Real)?;
                }
                UnaryOp::Not(_) => {
                    self.expr_eq_ty(right, Ty::Bool)?;
                    self.expr_eq_ty(expr, Ty::Bool)?;
                }
            },
            ExprType::Postfix(Postfix { left, .. }) => {
                self.expr_eq_ty(left, Ty::Real)?;
                self.expr_eq_ty(expr, Ty::Real)?;
            }
            ExprType::Access(access) => {
                match access {
                    Access::Global { .. } => {}
                    Access::Current { right } => {
                        let ty = self.canonize(expr)?;
                        let origin = self.local_var();
                        self.self_scope_mut()
                            .apply_field(&right.lexeme, Field::read(ty, expr.location(), origin))?
                            .commit(self)?;
                    }
                    Access::Other { .. } => {}
                    Access::Dot { left, right } => {
                        let mut record = Record::inferred();
                        let ty = self.canonize(expr)?;
                        let origin = self.local_var();
                        record
                            .apply_field(&right.lexeme, Field::read(ty, expr.location(), origin))?
                            .commit(self)?;
                        self.expr_eq_ty(left, Ty::Record(record))?;
                    }
                    Access::Array {
                        left,
                        index_one,
                        index_two,
                        ..
                    } => {
                        let ty = self.canonize(expr)?;

                        // our indexes must be real
                        self.expr_eq_ty(index_one, Ty::Real)?;
                        if let Some(index_two) = index_two {
                            self.expr_eq_ty(index_two, Ty::Real)?;
                        }

                        // the left must be an array of the member
                        self.expr_eq_ty(left, Ty::Array(Box::new(ty)))?;
                    }
                    Access::Struct { .. } => {}
                    Access::Map { .. } => {}
                    Access::Grid { .. } => {}
                    Access::List { .. } => {}
                }
            }
            ExprType::Call(crate::parse::Call { left, arguments, .. }) => {
                let expr_ty = self.canonize(expr)?;
                let mut parameters = vec![];
                for arg in arguments.iter() {
                    parameters.push(self.canonize(arg)?);
                }
                self.expr_eq_ty(
                    left,
                    Ty::Func(super::Func::Call(super::Call {
                        parameters,
                        return_type: Box::new(expr_ty),
                    })),
                )?;
            }
            ExprType::Grouping(Grouping { inner, .. }) => {
                self.expr_eq_expr(expr, inner)?;
            }

            ExprType::Identifier(iden) => {
                let ty = self.canonize(expr)?;
                let origin = self.local_var();
                let scope = if self.local_scope().contains(&iden.lexeme) {
                    self.local_scope_mut()
                } else {
                    self.self_scope_mut()
                };
                scope
                    .apply_field(&iden.lexeme, Field::read(ty, expr.location(), origin))?
                    .commit(self)?;
            }
            ExprType::Literal(literal) => {
                let ty = match literal {
                    Literal::True | Literal::False => Ty::Bool,
                    Literal::Undefined => Ty::Undefined,
                    Literal::Noone => Ty::Noone,
                    Literal::String(_) => Ty::Str,
                    Literal::Real(_) | Literal::Hex(_) => Ty::Real,
                    Literal::Misc(_) => Ty::Any, // todo
                    Literal::Array(exprs) => {
                        // Infer the type based on the first member
                        let app = if let Some(ty) = exprs.first().map(|expr| self.canonize(expr)) {
                            let ty = ty?;
                            Ty::Array(Box::new(ty))
                        } else {
                            Ty::Array(Box::new(Ty::Any))
                        };
                        self.expr_eq_ty(expr, app)?;
                        return Ok(());
                    }
                    Literal::Struct(_) => unreachable!(),
                };
                self.expr_eq_ty(expr, ty)?;
            }
        }

        Ok(())
    }

    fn process_function(&mut self, expr: &Expr, function: &crate::parse::Function) -> Result<(), TypeError> {
        let expr_ty = self.canonize(expr)?;
        println!(
            "\n--- Entering function ({}: {})... ---\n",
            if let Some(name) = &function.name {
                &name.lexeme
            } else {
                "anon"
            },
            Printer::ty(&expr_ty)
        );
        let binding = if let Some(constructor) = function.constructor.as_ref() {
            Binding::Constructor {
                local_scope: self.enter_new_local_scope(),
                self_scope: self.enter_new_constructor_scope(),
                inheritance: match constructor {
                    Constructor::WithInheritance(call) => Some(
                        call.inner()
                            .as_call()
                            .and_then(|call| call.left.inner().as_identifier())
                            .cloned()
                            .unwrap(),
                    ),
                    _ => None,
                },
            }
        } else {
            Binding::Method {
                local_scope: self.enter_new_local_scope(),
                self_scope: self.current_self_var(),
            }
        };

        let mut parameters = vec![];
        for param in function.parameters.iter() {
            let ty = match param {
                OptionalInitilization::Uninitialized(_) => Ty::Var(self.var_for_expr(param.name_expr())),
                OptionalInitilization::Initialized(_) => self.canonize(param.assignment_value().unwrap())?,
            };
            let origin = self.local_var();

            self.local_scope_mut()
                .apply_field(
                    param.name(),
                    Field::write(ty.clone(), param.name_expr().location(), origin),
                )?
                .commit(self)?;
            parameters.push(ty);
        }
        if let Err(errs) = &mut self.process_statements(function.body_stmts()) {
            return Err(errs.pop().unwrap()); // todo
        }

        self.depart_local_scope();
        let return_type = if function.constructor.is_some() {
            let ret = self.resolve_var(&self.current_self_var())?;
            self.depart_self_scope();
            Box::new(ret)
        } else {
            Box::new(self.subs.remove(&Var::Return).unwrap_or(Ty::Undefined))
        };
        println!("\n--- Exiting function... ---\n");

        // Create the new definition
        let mut ty = Ty::Func(super::Func::Def(super::Def {
            binding: Some(binding),
            parameters,
            return_type,
        }));

        // Do we already have a call placed on us?
        let expr_var = self.var_for_expr(expr);
        if let Ok(mut expr_ty) = self.resolve_var(&expr_var) {
            println!("--- Resolving a previous definition... ---\n");
            self.unify_tys(&mut ty, &mut expr_ty)?;
        }

        self.new_substitution(expr_var, ty);

        Ok(())
    }
}

// Utilities
impl Solver {
    #[inline]
    fn expr_eq_expr(&mut self, target: &Expr, expr: &Expr) -> Result<(), TypeError> {
        let mut lhs = self.canonize(target)?;
        let mut rhs = self.canonize(expr)?;
        self.unify_tys(&mut lhs, &mut rhs)
    }

    #[inline]
    fn expr_eq_ty(&mut self, expr: &Expr, mut ty: Ty) -> Result<(), TypeError> {
        let mut expr_ty = self.canonize(expr)?;
        self.unify_tys(&mut expr_ty, &mut ty)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Goal {
    Eq(Var, Ty),
}
