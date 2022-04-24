use crate::{duck_error, parse::*, solve::*};

impl Solver {
    pub fn process_statements(&mut self, stmts: &[Stmt]) -> Result<(), Vec<TypeError>> {
        let mut errors = vec![];
        for stmt in stmts.iter() {
            if let Err(e) = self.visit_stmt(stmt) {
                errors.push(e);
            }
        }
        if errors.is_empty() { Ok(()) } else { Err(errors) }
    }

    fn visit_stmt(&mut self, stmt: &Stmt) -> Result<(), TypeError> {
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
                let id = self.new_adt(AdtState::Concrete, fields);
                self.write_adt(AdtId::GLOBAL, &e.name, Ty::Adt(id))?;
                self.constants.insert(e.name.lexeme.clone());
            }
            StmtKind::Macro(mac) => {
                self.write_adt(AdtId::GLOBAL, &mac.name, Ty::Any)?;
                self.constants.insert(mac.name.lexeme.clone());
            }
            StmtKind::Assignment(Assignment { left, right, op }) => {
                let mut right_ty = right.query(self)?;
                if let AssignmentOp::Identity(_) = op {
                    if let Ok((adt_id, iden)) = self.expr_to_adt_access(left) {
                        self.write_adt(adt_id, iden, right_ty)?;
                    } else {
                        left.unify(&mut right_ty, self)?;
                    }
                }
            }
            StmtKind::LocalVariableSeries(LocalVariableSeries { declarations }) => {
                for initializer in declarations.iter() {
                    let ty = match initializer {
                        OptionalInitilization::Uninitialized(_) => Ty::Uninitialized,
                        OptionalInitilization::Initialized(_) => initializer.assignment_value().unwrap().query(self)?,
                    };
                    self.write_adt(self.local_id(), initializer.name_identifier(), ty)?;
                }
            }
            StmtKind::GlobalvarDeclaration(Globalvar { name }) => {
                self.write_adt(AdtId::GLOBAL, name, Ty::Uninitialized)?;
            }
            StmtKind::Return(Return { value }) => {
                let return_var = self.return_var();
                if let Some(value) = value {
                    value.unify(&mut Ty::Var(return_var), self)?;
                } else {
                    // todo impl query to var
                    self.unify_tys(&mut Ty::Undefined, &mut Ty::Var(return_var))?;
                }
            }
            StmtKind::WithLoop(WithLoop { body, .. }) => {
                self.visit_stmt(body)?;
                // TODO: Instance ID / Object ID!
            }
            StmtKind::RepeatLoop(RepeatLoop { tick_counts, body }) => {
                self.visit_stmt(body)?;
                tick_counts.unify(&mut Ty::Real, self)?;
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
                condition.unify(&mut Ty::Bool, self)?;
            }
            StmtKind::DoUntil(DoUntil { condition, body }) | StmtKind::WhileLoop(WhileLoop { condition, body }) => {
                self.visit_stmt(body)?;
                condition.unify(&mut Ty::Bool, self)?;
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
                condition.unify(&mut Ty::Bool, self)?;
            }
            StmtKind::Switch(Switch {
                matching_value,
                cases,
                default_case,
            }) => {
                let mut identity = matching_value.query(self)?;
                for case in cases {
                    for stmt in case.iter_body_statements() {
                        self.visit_stmt(stmt)?;
                    }
                    case.identity().unify(&mut identity, self)?;
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

        let ty = match self.kind() {
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
                eval.left.unify(&mut Ty::Real, solver)?;
                eval.right.unify(&mut Ty::Real, solver)?;
                Ok(Ty::Real)
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
                let mut ty = Ty::Var(self.var());
                solver.normalize(&mut ty);
                Ok(ty)
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
                        let id = solver.new_adt(AdtState::Extendable, vec![]);
                        solver.enter_self_scope(id);
                        for declaration in declarations {
                            let ty = declaration.1.query(solver)?.clone();
                            solver.write_adt(id, &declaration.0, ty)?;
                        }
                        solver.depart_self_scope();
                        Ty::Adt(id)
                    }
                };

                // Since this expr is a literal we can just skip unification and directly sub it
                // (just to reduce operations, no functional difference)
                Ok(ty)
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
        }?;

        if Ty::Var(self.var()) != ty {
            Ok(solver.sub(self.var(), ty))
        } else {
            Ok(ty)
        }
    }

    fn var(&self) -> Var {
        Var::Expr(self.id())
    }
}

impl Solver {
    fn process_function(&mut self, expr: &Expr, function: &crate::parse::Function) -> Result<Ty, TypeError> {
        let mut parameters = vec![];
        let mut local_fields = vec![];
        let mut found_minimum = None;
        for (i, param) in function.parameters.iter().enumerate() {
            let ty = if let Some(value) = param.assignment_value() {
                found_minimum = Some(i);
                value.query(self)?
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
        let local_scope = self.new_adt(AdtState::Extendable, local_fields);
        self.enter_local_scope(local_scope);
        let binding = if let Some(constructor) = function.constructor.as_ref() {
            let self_scope = self.new_adt(AdtState::Extendable, vec![]);
            self.enter_self_scope(self_scope);
            Binding::Constructor {
                local_scope,
                self_scope,
                inheritance: match constructor {
                    Constructor::WithInheritance(call) => Some(
                        call.kind()
                            .as_call()
                            .and_then(|call| call.left.kind().as_identifier())
                            .cloned()
                            .unwrap(),
                    ),
                    _ => None,
                },
            }
        } else {
            Binding::Method {
                local_scope,
                self_scope: self.self_id(),
            }
        };

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
            minimum_arguments,
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
        field.clone()
    } else {
        solver.read_adt(id, iden, Ty::Var(var))?;
        Ty::Var(var)
    };
    Ok(ty)
}
