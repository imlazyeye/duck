use crate::{duck_error, parse::*, solve::*, var};

impl<'s> Session<'s> {
    pub fn process_statements(&mut self, stmts: &[Stmt]) -> Result<(), TypeError> {
        for stmt in stmts.iter() {
            // todo: there's no reason (I don't think) that we have to only accept ONE error, we could keep
            // going, but its easier for types right now if everyting just returns one type error
            self.visit_stmt(stmt)?;
        }
        Ok(())
    }

    fn visit_stmt(&mut self, stmt: &Stmt) -> Result<(), TypeError> {
        println!("{}", Printer::stmt(stmt));
        match stmt.kind() {
            StmtKind::Enum(e) => {
                let mut fields = vec![];
                for init in e.members.iter() {
                    if !init
                        .assignment_value()
                        .map_or(true, |v| v.kind().is_numerical_constant())
                    {
                        return duck_error!("enum members must be numerical constants");
                    }
                    fields.push((init.name_identifier().clone(), Ty::Real))
                }
                let adt = Adt::new(AdtState::Concrete, fields);
                self.adt_mut(&Var::GlobalAdt)
                    .write_constant(&e.name.lexeme, Ty::Adt(adt))?
                    .commit(self.subs)?;
            }
            StmtKind::Macro(mac) => {
                self.adt_mut(&Var::GlobalAdt)
                    .write_constant(&mac.name.lexeme, Ty::Any)?
                    .commit(self.subs)?;
            }
            StmtKind::Assignment(Assignment { left, right, op }) => match op {
                AssignmentOp::Identity(_) => {
                    left.unify_expr(right, self)?;
                }
                AssignmentOp::NullCoalecenceEqual(_) => unimplemented!(),
                _ => {
                    left.unify_ty(Ty::Real, self)?;
                    right.unify_ty(Ty::Real, self)?;
                }
            },
            StmtKind::LocalVariables(LocalVariables { declarations }) => {
                for initializer in declarations.iter() {
                    // To enable shadowing, we first remove any old field for this name
                    self.local_mut().fields.remove(initializer.name());

                    if let Some(value) = initializer.assignment_value() {
                        let ty = Ty::Var(value.query(self)?);
                        self.local_mut().write(initializer.name(), ty)?.commit(self.subs)?;
                    } else {
                        self.local_mut().write_unitialized(initializer.name())?;
                    };
                }
            }
            StmtKind::Globalvar(Globalvar { name }) => self.adt_mut(&Var::GlobalAdt).write_unitialized(&name.lexeme)?,
            StmtKind::Return(Return { value }) => {
                if let Some(value) = value {
                    Unification::var_var(Var::Return, value.query(self)?, self)?;
                } else {
                    self.subs.register(Var::Return, Ty::Undefined)?;
                }
            }
            StmtKind::With(With {
                body,
                identity: _identity,
            }) => {
                // let mut ty = identity.query(self)?.normalized(self);
                // if let Ty::Adt(adt) = &mut ty {
                //     if adt.state == AdtState::Inferred {
                //         adt.write(&right.lexeme, &Ty::Var(my_var))?.commit(sess)?;
                //     } else {
                //         adt.read(&right.lexeme, &Ty::Var(my_var))?.commit(sess)?;
                //     };
                // } else {
                //     let adt = Adt::new(AdtState::Inferred, vec![(right.clone(), Ty::Var(my_var))]);
                //     left.unify(Ty::Adt(adt), sess)?;
                // };
                self.visit_stmt(body)?;
            }
            StmtKind::Repeat(Repeat { tick_counts, body }) => {
                self.visit_stmt(body)?;
                tick_counts.unify_ty(Ty::Real, self)?;
            }
            StmtKind::For(For {
                condition,
                initializer,
                iterator,
                body,
            }) => {
                self.visit_stmt(initializer)?;
                self.visit_stmt(iterator)?;
                self.visit_stmt(body)?;
                condition.unify_ty(Ty::Bool, self)?;
            }
            StmtKind::DoUntil(DoUntil { condition, body }) | StmtKind::While(While { condition, body }) => {
                self.visit_stmt(body)?;
                condition.unify_ty(Ty::Bool, self)?;
            }
            StmtKind::If(If {
                condition,
                body,
                else_stmt,
                ..
            }) => {
                self.visit_stmt(body)?;
                if let Some(else_stmt) = else_stmt {
                    self.visit_stmt(else_stmt)?;
                }
                condition.unify_ty(Ty::Bool, self)?;
            }
            StmtKind::Switch(Switch {
                matching_value,
                cases,
                default_case,
            }) => {
                for case in cases {
                    for stmt in case.iter_body_statements() {
                        self.visit_stmt(stmt)?;
                    }
                    case.identity().unify_expr(matching_value, self)?;
                }
                if let Some(default_case) = default_case {
                    for stmt in default_case.iter() {
                        self.visit_stmt(stmt)?;
                    }
                }
            }
            StmtKind::Expr(expr) => {
                expr.query(self)?;
            }
            StmtKind::TryCatch(try_catch) => {
                self.visit_stmt(&try_catch.try_body)?;
                self.visit_stmt(&try_catch.catch_body)?;
                if let Some(finally_body) = &try_catch.finally_body {
                    self.visit_stmt(finally_body)?;
                }
            }
            StmtKind::Block(block) => {
                for stmt in block.body.iter() {
                    self.visit_stmt(stmt)?;
                }
            }
            _ => {}
        }
        Ok(())
    }
}

impl Expr {
    fn unify_expr(&self, expr: &Expr, session: &mut Session) -> Result<(), TypeError> {
        Unification::var_var(self.query(session)?, expr.query(session)?, session)
    }

    fn unify_ty(&self, mut ty: Ty, session: &mut Session) -> Result<(), TypeError> {
        Unification::var_ty(self.query(session)?, &mut ty, session)
    }

    pub(super) fn query<'s>(&self, sess: &'s mut Session) -> Result<Var, TypeError> {
        let my_var = Var::Expr(self.id());
        if !sess.subs.contains(&my_var) {
            println!("{}", Printer::query(self));

            let ty = match self.kind() {
                ExprKind::Function(func) => {
                    let ty = if let Some(constructor) = func.constructor.as_ref() {
                        sess.process_constructor(func, constructor)?
                    } else {
                        sess.process_function(func)?
                    };
                    if let Some(name) = &func.name {
                        sess.identity_mut().write(&name.lexeme, ty.clone())?.commit(sess.subs)?;
                    };
                    Ok(ty)
                }
                ExprKind::Logical(log) => {
                    log.left.unify_ty(Ty::Bool, sess)?;
                    log.right.unify_ty(Ty::Bool, sess)?;
                    Ok(Ty::Bool)
                }
                ExprKind::Equality(eq) => {
                    eq.right.unify_expr(&eq.left, sess)?;
                    Ok(Ty::Bool)
                }
                ExprKind::Evaluation(eval) => {
                    if let EvaluationOp::Plus(_) = eval.op {
                        let var = eval.right.query(sess)?;
                        eval.left.unify_ty(Ty::Var(var), sess)?;
                        Ok(Ty::Var(var))
                    } else {
                        eval.right.unify_ty(Ty::Real, sess)?;
                        eval.left.unify_ty(Ty::Real, sess)?;
                        Ok(Ty::Real)
                    }
                }
                ExprKind::NullCoalecence(null) => {
                    let ty = Ty::Var(null.right.query(sess)?);
                    null.left.unify_ty(Ty::Option(Box::new(ty.clone())), sess)?;
                    Ok(ty)
                }
                ExprKind::Ternary(ternary) => {
                    ternary.condition.unify_ty(Ty::Bool, sess)?;
                    ternary.true_value.unify_expr(&ternary.false_value, sess)?;
                    Ok(Ty::Var(ternary.false_value.query(sess)?))
                }
                ExprKind::Unary(unary) => match unary.op {
                    UnaryOp::Increment(_)
                    | UnaryOp::Decrement(_)
                    | UnaryOp::Positive(_)
                    | UnaryOp::Negative(_)
                    | UnaryOp::BitwiseNot(_) => {
                        unary.right.unify_ty(Ty::Real, sess)?;
                        Ok(Ty::Real)
                    }
                    UnaryOp::Not(_) => {
                        unary.right.unify_ty(Ty::Bool, sess)?;
                        Ok(Ty::Bool)
                    }
                },
                ExprKind::Postfix(postfix) => {
                    postfix.left.unify_ty(Ty::Real, sess)?;
                    Ok(Ty::Real)
                }
                ExprKind::Access(access) => match access {
                    Access::Global { right } => handle_adt(my_var, sess, &Var::GlobalAdt, right),
                    Access::Identity { right } => {
                        let id = *sess.identity_var();
                        handle_adt(my_var, sess, &id, right)
                    }
                    Access::Dot { left, right } => {
                        // If we can find an adt on the left, we will read/write to it. Otherwise, we'll infer a new
                        // one.
                        let adt_var = left.query(sess)?;
                        let adt_ty = sess.get_normalized_mut(adt_var); // TODO: prob bad?
                        if let Some(Ty::Adt(adt)) = adt_ty {
                            if adt.state == AdtState::Inferred {
                                adt.write(&right.lexeme, Ty::Var(my_var))?.commit(sess.subs)?;
                            } else {
                                adt.read(&right.lexeme, Ty::Var(my_var))?.commit(sess.subs)?;
                            };
                        } else {
                            let adt = Adt::new(AdtState::Inferred, vec![(right.clone(), Ty::Var(my_var))]);
                            left.unify_ty(Ty::Adt(adt), sess)?;
                        };
                        Ok(Ty::Var(my_var))
                    }
                    Access::Array {
                        left,
                        index_one,
                        index_two,
                        ..
                    } => {
                        index_one.unify_ty(Ty::Real, sess)?;
                        if let Some(index_two) = index_two {
                            index_two.unify_ty(Ty::Real, sess)?;
                        }
                        left.unify_ty(Ty::Array(Box::new(Ty::Var(my_var))), sess)?;
                        Ok(Ty::Var(my_var))
                    }
                    _ => unimplemented!(),
                },
                ExprKind::Call(call) => {
                    let parameters = call
                        .arguments
                        .iter()
                        .map(|expr| expr.query(sess).map(Ty::Var))
                        .collect::<Result<Vec<Ty>, TypeError>>()?;
                    call.left.unify_ty(
                        Ty::Func(super::Func::Call(super::Call {
                            parameters,
                            return_type: Box::new(Ty::Var(my_var)),
                        })),
                        sess,
                    )?;
                    Ok(Ty::Var(my_var))
                }
                ExprKind::Grouping(grouping) => Ok(Ty::Var(grouping.inner.query(sess)?)),
                ExprKind::Literal(literal) => {
                    let ty = match literal {
                        Literal::True | Literal::False => Ty::Bool,
                        Literal::Undefined => Ty::Undefined,
                        Literal::Noone => Ty::Noone,
                        Literal::String(_) => Ty::Str,
                        Literal::Real(_) | Literal::Hex(_) => Ty::Real,
                        Literal::Misc(_) => Ty::Any, // todo
                        Literal::Array(exprs) => {
                            let ty = if let Some(expr) = exprs.first() {
                                let ty = Ty::Var(expr.query(sess)?);
                                for expr in exprs.iter().skip(1) {
                                    expr.unify_ty(ty.clone(), sess)?;
                                }
                                ty
                            } else {
                                var!()
                            };
                            Ty::Array(Box::new(ty))
                        }
                        Literal::Struct(declarations) => {
                            sess.enter_new_identity(vec![]);
                            for declaration in declarations {
                                let ty = Ty::Var(declaration.1.query(sess)?);
                                sess.identity_mut()
                                    .write(&declaration.0.lexeme, ty)?
                                    .commit(sess.subs)?;
                            }
                            let ty = sess.identity().clone(); // todo
                            sess.pop_identity();
                            Ty::Adt(ty)
                        }
                    };

                    // Since this expr is a literal we can just skip unification and directly sub it
                    // (just to reduce operations, no functional difference)
                    Ok(ty)
                }
                ExprKind::Identifier(iden) => match iden.lexeme.as_str() {
                    "self" => Ok(Ty::Identity),
                    name => {
                        let id = if sess.local().contains(name) {
                            *sess.local_var()
                        } else if sess.adt(&Var::GlobalAdt).contains(name) {
                            Var::GlobalAdt
                        } else {
                            *sess.identity_var()
                        };
                        handle_adt(my_var, sess, &id, iden)
                    }
                },
            }?;

            sess.subs.register(my_var, ty)?;
        }
        Ok(my_var)
    }
}

impl<'s> Session<'s> {
    fn process_function_head(&mut self, function: &Function) -> Result<(Vec<Ty>, usize, Var), TypeError> {
        #[cfg(test)]
        println!("\n--- Entering function... ---\n");
        let mut parameters = vec![];
        let mut local_fields = vec![];
        let mut found_minimum = None;
        for (i, param) in function.parameters.iter().enumerate() {
            let ty = if let Some(value) = param.assignment_value() {
                found_minimum = Some(i);
                Ty::Var(value.query(self)?)
            } else {
                if found_minimum.is_some() {
                    return duck_error!("default arguments can not be followed by standard arguments");
                }
                Ty::Var(param.name_expr().query(self)?)
            };
            local_fields.push((param.name_identifier().clone(), ty.clone()));
            parameters.push(ty);
        }
        let minimum_arguments = found_minimum.unwrap_or(parameters.len());
        Ok((parameters, minimum_arguments, self.enter_new_local(local_fields)))
    }

    fn process_constructor(
        &mut self,
        function: &crate::parse::Function,
        constructor: &Constructor,
    ) -> Result<Ty, TypeError> {
        let inheritance_call = if let Constructor::WithInheritance(_call) = constructor {
            unimplemented!()
            // Some(call.query(self)?.normalized(self))
        } else {
            None
        };

        let (parameters, minimum_arguments, local_var) = self.process_function_head(function)?;
        self.push_local(local_var);
        self.enter_new_identity(vec![]);

        if let Some(inheritance_call) = inheritance_call {
            if let Ty::Adt(adt) = inheritance_call {
                self.identity_mut().fields.extend(adt.fields);
            } else {
                panic!("i don't know what to do here: {}", Printer::ty(&inheritance_call))
            }
        }
        self.process_statements(function.body_stmts())?;
        self.pop_local();
        self.subs.remove(&Var::Return);

        let mut func = super::Func::Def(super::Def {
            binding: Some(Binding {
                local_var,
                identity_var: *self.identity_var(),
            }),
            parameters,
            minimum_arguments,
            return_type: Box::new(Ty::Var(*self.identity_var())),
        });

        if let Some(name) = &function.name {
            self.identity_mut()
                .write(&name.lexeme, Ty::Func(func.clone()))?
                .commit(self.subs)?;
        }
        self.identity_mut().state = AdtState::Concrete;
        *func.return_type_mut() = Ty::Adt(self.identity().clone());
        self.pop_identity();
        println!("\n--- Departing function... ---\n");

        Ok(Ty::Func(func))
    }

    fn process_function(&mut self, function: &crate::parse::Function) -> Result<Ty, TypeError> {
        let (parameters, minimum_arguments, local_var) = self.process_function_head(function)?;
        self.push_local(local_var);
        let binding = if self.identity_var() == &Var::GlobalAdt {
            // We create a dummy adt for self, since global functions are not bound to anything
            self.enter_new_identity(vec![]);
            self.process_statements(function.body_stmts())?;
            self.pop_identity();
            None
        } else {
            self.process_statements(function.body_stmts())?;
            Some(Binding {
                local_var,
                identity_var: *self.identity_var(),
            })
        };
        self.pop_local();
        let return_type = Box::new(self.subs.remove(&Var::Return).unwrap_or(Ty::Undefined));
        println!("\n--- Departing function... ---\n");
        Ok(Ty::Func(super::Func::Def(super::Def {
            binding,
            parameters,
            minimum_arguments,
            return_type,
        })))
    }
}

fn handle_adt(expr_var: Var, session: &mut Session, id: &Var, iden: &Identifier) -> Result<Ty, TypeError> {
    let ty = if let Some(field) = session.adt(id).ty(&iden.lexeme) {
        field.clone()
    } else {
        session
            .adt_mut(id)
            .read(&iden.lexeme, Ty::Var(expr_var))?
            .commit(session.subs)?;
        Ty::Var(expr_var)
    };
    Ok(ty)
}
