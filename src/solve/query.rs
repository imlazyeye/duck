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
                    .write_constant(&e.name.lexeme, &Ty::Adt(adt))?
                    .commit(self)?;
            }
            StmtKind::Macro(mac) => {
                self.adt_mut(&Var::GlobalAdt)
                    .write_constant(&mac.name.lexeme, &Ty::Any)?
                    .commit(self)?;
            }
            StmtKind::Assignment(Assignment { left, right, op }) => match op {
                AssignmentOp::Identity(_) => {
                    left.unify(&right.query(self)?, self)?;
                }
                AssignmentOp::NullCoalecenceEqual(_) => todo!(),
                _ => {
                    left.unify(&Ty::Real, self)?;
                    right.unify(&Ty::Real, self)?;
                }
            },
            StmtKind::LocalVariableSeries(LocalVariableSeries { declarations }) => {
                for initializer in declarations.iter() {
                    // To enable shadowing, we first remove any old field for this name
                    self.local_mut().fields.remove(initializer.name());

                    if let Some(value) = initializer.assignment_value() {
                        let ty = value.query(self)?;
                        self.local_mut().write(initializer.name(), &ty)?.commit(self)?;
                    } else {
                        self.local_mut().write_unitialized(initializer.name())?;
                    };
                }
            }
            StmtKind::GlobalvarDeclaration(Globalvar { name }) => {
                self.adt_mut(&Var::GlobalAdt).write_unitialized(&name.lexeme)?
            }
            StmtKind::Return(Return { value }) => {
                if let Some(value) = value {
                    value.unify(&Ty::Var(Var::Return), self)?;
                } else {
                    self.unify_var_ty(&Var::Return, &Ty::Undefined)?;
                }
            }
            StmtKind::WithLoop(WithLoop { body, identity }) => {
                // let mut ty = identity.query(self)?.normalized(self);
                // if let Ty::Adt(adt) = &mut ty {
                //     if adt.state == AdtState::Inferred {
                //         adt.write(&right.lexeme, &Ty::Var(self.var()))?.commit(sess)?;
                //     } else {
                //         adt.read(&right.lexeme, &Ty::Var(self.var()))?.commit(sess)?;
                //     };
                // } else {
                //     let adt = Adt::new(AdtState::Inferred, vec![(right.clone(), Ty::Var(self.var()))]);
                //     left.unify(&Ty::Adt(adt), sess)?;
                // };
                self.visit_stmt(body)?;
            }
            StmtKind::RepeatLoop(RepeatLoop { tick_counts, body }) => {
                self.visit_stmt(body)?;
                tick_counts.unify(&Ty::Real, self)?;
            }
            StmtKind::ForLoop(ForLoop {
                condition,
                initializer,
                iterator,
                body,
            }) => {
                self.visit_stmt(initializer)?;
                self.visit_stmt(iterator)?;
                self.visit_stmt(body)?;
                condition.unify(&Ty::Bool, self)?;
            }
            StmtKind::DoUntil(DoUntil { condition, body }) | StmtKind::WhileLoop(WhileLoop { condition, body }) => {
                self.visit_stmt(body)?;
                condition.unify(&Ty::Bool, self)?;
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
                condition.unify(&Ty::Bool, self)?;
            }
            StmtKind::Switch(Switch {
                matching_value,
                cases,
                default_case,
            }) => {
                let identity = matching_value.query(self)?;
                for case in cases {
                    for stmt in case.iter_body_statements() {
                        self.visit_stmt(stmt)?;
                    }
                    case.identity().unify(&identity, self)?;
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

// Queries
pub trait QueryItem {
    fn query(&self, session: &mut Session) -> Result<Ty, TypeError>;
    fn var(&self) -> Var;
    fn unify(&self, ty: &Ty, session: &mut Session) -> Result<(), TypeError> {
        let me = self.query(session)?;
        session.unify_ty_ty(&me, ty)
    }
}
impl QueryItem for Expr {
    fn query<'s>(&self, sess: &'s mut Session) -> Result<Ty, TypeError> {
        if let Some(cache) = sess.subs.get(&self.var()) {
            return Ok(cache.clone());
        }

        println!("{}", Printer::query(self));

        let ty = match self.kind() {
            ExprKind::Function(func) => {
                let ty = if let Some(constructor) = func.constructor.as_ref() {
                    sess.process_constructor(func, constructor)?
                } else {
                    sess.process_function(func)?
                };
                if let Some(name) = &func.name {
                    sess.identity_mut().write(&name.lexeme, &ty)?.commit(sess)?;
                };
                Ok(ty)
            }
            ExprKind::Logical(log) => {
                log.left.unify(&Ty::Bool, sess)?;
                log.right.unify(&Ty::Bool, sess)?;
                Ok(Ty::Bool)
            }
            ExprKind::Equality(eq) => {
                let left = eq.left.query(sess)?;
                eq.right.unify(&left, sess)?;
                Ok(Ty::Bool)
            }
            ExprKind::Evaluation(eval) => {
                eval.left.unify(&Ty::Real, sess)?;
                eval.right.unify(&Ty::Real, sess)?;
                Ok(Ty::Real)
            }
            ExprKind::NullCoalecence(null) => {
                let ty = null.right.query(sess)?;
                null.left.unify(&Ty::Option(Box::new(ty.clone())), sess)?;
                Ok(ty)
            }
            ExprKind::Ternary(ternary) => {
                ternary.condition.unify(&Ty::Bool, sess)?;
                let false_value = ternary.false_value.query(sess)?;
                ternary.true_value.unify(&false_value, sess)?;
                Ok(false_value)
            }
            ExprKind::Unary(unary) => match unary.op {
                UnaryOp::Increment(_)
                | UnaryOp::Decrement(_)
                | UnaryOp::Positive(_)
                | UnaryOp::Negative(_)
                | UnaryOp::BitwiseNot(_) => {
                    unary.right.unify(&Ty::Real, sess)?;
                    Ok(Ty::Real)
                }
                UnaryOp::Not(_) => {
                    unary.right.unify(&Ty::Bool, sess)?;
                    Ok(Ty::Bool)
                }
            },
            ExprKind::Postfix(postfix) => {
                postfix.left.unify(&Ty::Real, sess)?;
                Ok(Ty::Real)
            }
            ExprKind::Access(access) => match access {
                Access::Global { right } => handle_adt(self, sess, &Var::GlobalAdt, right),
                Access::Identity { right } => {
                    let id = *sess.identity_var();
                    handle_adt(self, sess, &id, right)
                }
                Access::Dot { left, right } => {
                    // If we can find an adt on the left, we will read/write to it. Otherwise, we'll infer a new one.
                    left.query(sess)?; // todo: this should be an &mut...
                    let ty = sess.get_normalized_mut(left.var());
                    if let Some(Ty::Adt(adt)) = ty {
                        if adt.state == AdtState::Inferred {
                            adt.write(&right.lexeme, &Ty::Var(self.var()))?.commit(sess)?;
                        } else {
                            adt.read(&right.lexeme, &Ty::Var(self.var()))?.commit(sess)?;
                        };
                    } else {
                        let adt = Adt::new(AdtState::Inferred, vec![(right.clone(), Ty::Var(self.var()))]);
                        left.unify(&Ty::Adt(adt), sess)?;
                    };
                    Ok(Ty::Var(self.var()))
                }
                Access::Array {
                    left,
                    index_one,
                    index_two,
                    ..
                } => {
                    index_one.unify(&Ty::Real, sess)?;
                    if let Some(index_two) = index_two {
                        index_two.unify(&Ty::Real, sess)?;
                    }
                    left.unify(&Ty::Array(Box::new(Ty::Var(self.var()))), sess)?;
                    Ok(sess
                        .subs
                        .get(&self.var())
                        .cloned()
                        .unwrap_or_else(|| Ty::Var(self.var())))
                }
                _ => unimplemented!(),
            },
            ExprKind::Call(call) => {
                let parameters = call
                    .arguments
                    .iter()
                    .map(|expr| expr.query(sess))
                    .collect::<Result<Vec<Ty>, TypeError>>()?;
                call.left.unify(
                    &Ty::Func(super::Func::Call(super::Call {
                        parameters,
                        return_type: Box::new(Ty::Var(self.var())),
                    })),
                    sess,
                )?;
                Ok(Ty::Var(self.var()))
            }
            ExprKind::Grouping(grouping) => grouping.inner.query(sess),
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
                            let ty = expr.query(sess)?;
                            for expr in exprs.iter().skip(1) {
                                expr.unify(&ty, sess)?;
                            }
                            ty.clone()
                        } else {
                            var!()
                        };
                        Ty::Array(Box::new(ty))
                    }
                    Literal::Struct(declarations) => {
                        sess.enter_new_identity(vec![]);
                        for declaration in declarations {
                            let ty = declaration.1.query(sess)?;
                            sess.identity_mut().write(&declaration.0.lexeme, &ty)?.commit(sess)?;
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
            ExprKind::Identifier(iden) => {
                let id = if sess.local().contains(&iden.lexeme) {
                    *sess.local_var()
                } else if sess.adt(&Var::GlobalAdt).contains(&iden.lexeme) {
                    Var::GlobalAdt
                } else {
                    *sess.identity_var()
                };
                handle_adt(self, sess, &id, iden)
            }
        }?;

        if Ty::Var(self.var()) != ty {
            sess.sub(self.var(), ty.clone());
            Ok(ty)
        } else {
            Ok(ty)
        }
    }

    fn var(&self) -> Var {
        Var::Expr(self.id())
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
                value.query(self)?.clone()
            } else {
                if found_minimum.is_some() {
                    return duck_error!("default arguments can not be followed by standard arguments");
                }
                Ty::Var(param.name_expr().var())
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
        let inheritance_call = if let Constructor::WithInheritance(call) = constructor {
            todo!()
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
            return_type: Box::new(Ty::Identity),
        });

        if let Some(name) = &function.name {
            self.identity_mut()
                .write(&name.lexeme, &Ty::Func(func.clone()))?
                .commit(self)?;
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

fn handle_adt(expr: &Expr, session: &mut Session, id: &Var, iden: &Identifier) -> Result<Ty, TypeError> {
    let var = expr.var();
    let ty = if let Some(field) = session.adt(id).ty(&iden.lexeme) {
        field.clone()
    } else {
        session.adt_mut(id).read(&iden.lexeme, &Ty::Var(var))?.commit(session)?;
        Ty::Var(var)
    };
    Ok(ty)
}
