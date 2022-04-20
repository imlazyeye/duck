use crate::{parse::*, solve::*};

// Queries
pub trait QueryItem {
    fn query(&self, solver: &mut Solver) -> Result<Ty, TypeError>;
    fn var(&self) -> Var;
    fn unify(&self, ty: &mut Ty, solver: &mut Solver) -> Result<(), TypeError> {
        let me = &mut self.query(solver)?;
        solver.unify_tys(me, ty)
    }
}
impl QueryItem for Expr {
    fn query(&self, solver: &mut Solver) -> Result<Ty, TypeError> {
        if let Some(cache) = solver.subs.get(&self.var()) {
            return Ok(cache.clone());
        }

        println!("{}", Printer::query(self));

        match self.inner() {
            ExprKind::Enum(e) => {
                let mut fields = vec![];
                for init in e.members.iter() {
                    if let Some(value) = init.assignment_value() {
                        value.unify(&mut Ty::Real, solver)?;
                    }
                    fields.push((init.name_identifier().clone(), Ty::Real))
                }
                let adt = solver.new_adt(AdtState::Concrete, fields);
                solver.write_adt(AdtId::GLOBAL, &e.name, Ty::Adt(adt))?;
                Ok(Ty::Adt(adt))
            }
            ExprKind::Macro(mac) => {
                solver.write_adt(AdtId::GLOBAL, &mac.name, Ty::Any)?;
                Ok(Ty::Any)
            }
            ExprKind::Function(func) => {
                let ty = solver.process_function(self, func)?;
                if let Some(name) = &func.name {
                    solver.write_adt(solver.self_id(), name, ty.clone())?;
                };
                Ok(ty)
            }
            ExprKind::Logical(log) => {
                log.left.unify(&mut Ty::Bool, solver)?;
                log.right.unify(&mut Ty::Bool, solver)?;
                Ok(Ty::Bool)
            }
            ExprKind::Equality(eq) => {
                let mut left = eq.left.query(solver)?;
                eq.right.unify(&mut left, solver)?;
                Ok(Ty::Bool)
            }
            ExprKind::Evaluation(eval) => {
                let mut left = eval.left.query(solver)?;
                eval.right.unify(&mut left, solver)?;
                Ok(left)
            }
            ExprKind::NullCoalecence(_) => todo!(),
            ExprKind::Ternary(ternary) => {
                ternary.condition.unify(&mut Ty::Bool, solver)?;
                let mut false_value = ternary.false_value.query(solver)?;
                ternary.true_value.unify(&mut false_value, solver)?;
                Ok(false_value)
            }
            ExprKind::Unary(unary) => match unary.op {
                UnaryOp::Increment(_)
                | UnaryOp::Decrement(_)
                | UnaryOp::Positive(_)
                | UnaryOp::Negative(_)
                | UnaryOp::BitwiseNot(_) => {
                    unary.right.unify(&mut Ty::Real, solver)?;
                    Ok(Ty::Real)
                }
                UnaryOp::Not(_) => {
                    unary.right.unify(&mut Ty::Bool, solver)?;
                    Ok(Ty::Bool)
                }
            },
            ExprKind::Postfix(postfix) => {
                postfix.left.unify(&mut Ty::Real, solver)?;
                Ok(Ty::Real)
            }
            ExprKind::Access(access) => match access {
                Access::Global { right } => handle_adt(self, solver, AdtId::GLOBAL, right),
                Access::Identity { right } => handle_adt(self, solver, solver.self_id(), right),
                Access::Dot { left, right } => {
                    let id = if let Ty::Adt(id) = left.query(solver)? {
                        id
                    } else {
                        let id = solver.new_adt(AdtState::Inferred, vec![(right.clone(), Ty::Var(self.var()))]);
                        left.unify(&mut Ty::Adt(id), solver)?;
                        id
                    };
                    handle_adt(self, solver, id, right)
                }
                Access::Array {
                    left,
                    index_one,
                    index_two,
                    ..
                } => {
                    index_one.unify(&mut Ty::Real, solver)?;
                    if let Some(index_two) = index_two {
                        index_two.unify(&mut Ty::Real, solver)?;
                    }
                    left.unify(&mut Ty::Array(Box::new(Ty::Var(self.var()))), solver)?;
                    Ok(solver
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
                    .map(|expr| expr.query(solver))
                    .collect::<Result<Vec<Ty>, TypeError>>()?;
                call.left.unify(
                    &mut Ty::Func(super::Func::Call(super::Call {
                        parameters,
                        return_type: Box::new(Ty::Var(self.var())),
                    })),
                    solver,
                )?;
                Ok(Ty::Var(self.var())) // potentially not normalized
            }
            ExprKind::Grouping(grouping) => grouping.inner.query(solver),
            ExprKind::Literal(literal) => {
                let ty = match literal {
                    Literal::True | Literal::False => Ty::Bool,
                    Literal::Undefined => Ty::Undefined,
                    Literal::Noone => Ty::Noone,
                    Literal::String(_) => Ty::Str,
                    Literal::Real(_) | Literal::Hex(_) => Ty::Real,
                    Literal::Misc(_) => Ty::Any, // todo
                    Literal::Array(exprs) => Ty::Array(Box::new(
                        exprs
                            .first()
                            .and_then(|expr| expr.query(solver).ok())
                            .unwrap_or(Ty::Any),
                    )),
                    Literal::Struct(declarations) => {
                        let mut fields = vec![];
                        for declaration in declarations {
                            let ty = declaration.1.query(solver)?.clone();
                            fields.push((declaration.0.clone(), ty));
                        }
                        Ty::Adt(solver.new_adt(AdtState::Extendable, fields))
                    }
                };

                // Since this expr is a literal we can just skip unification and directly sub it
                // (just to reduce operations, no functional difference)
                Ok(solver.sub(self.var(), ty))
            }
            ExprKind::Identifier(iden) => {
                let id = if solver.get_adt(solver.local_id()).contains(&iden.lexeme) {
                    solver.local_id()
                } else if solver.get_adt(AdtId::GLOBAL).contains(&iden.lexeme) {
                    AdtId::GLOBAL
                } else {
                    solver.self_id()
                };
                handle_adt(self, solver, id, iden)
            }
        }
    }

    fn var(&self) -> Var {
        Var::Expr(self.id())
    }
}

impl Solver {
    fn process_function(&mut self, expr: &Expr, function: &crate::parse::Function) -> Result<Ty, TypeError> {
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
                self_scope: self.self_id(),
            }
        };

        let mut parameters = vec![];
        for param in function.parameters.iter() {
            let mut ty = param.name_expr().query(self)?;
            self.write_adt(self.local_id(), param.name_identifier(), ty.clone())?;
            if let Some(value) = param.assignment_value() {
                value.unify(&mut ty, self)?;
            }
            parameters.push(ty);
        }
        self.enter_new_return_body();
        if let Err(errs) = &mut self.process_statements(function.body_stmts()) {
            return Err(errs.pop().unwrap()); // todo
        }

        self.depart_local_scope();
        let return_type = if function.constructor.is_some() {
            let _ = self.retrieve_return_value();
            let ret = Ty::Adt(self.self_id());
            self.depart_self_scope();
            Box::new(ret)
        } else {
            let ty = self.retrieve_return_value();
            if ty == Ty::Uninitialized {
                Box::new(Ty::Undefined)
            } else {
                Box::new(ty)
            }
        };

        // Create the new definition
        let mut ty = Ty::Func(super::Func::Def(super::Def {
            binding: Some(binding),
            parameters,
            return_type,
        }));

        // Do we already have a call placed on us?
        if let Some(mut expr_ty) = self.subs.remove(&expr.var()) {
            #[cfg(test)]
            println!("--- Resolving a previous definition... ---\n");
            self.unify_tys(&mut ty, &mut expr_ty)?;
        }

        Ok(ty)
    }
}

fn handle_adt(expr: &Expr, solver: &mut Solver, id: AdtId, iden: &Identifier) -> Result<Ty, TypeError> {
    let var = expr.var();
    let ty = if let Some(field) = solver.get_adt(id).get(&iden.lexeme) {
        field.ty.clone()
    } else {
        solver.read_adt(id, iden, Ty::Var(var))?;
        Ty::Var(var)
    };
    Ok(ty)
}
