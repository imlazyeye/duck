use super::*;
use crate::parse::*;

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
}

// Goal construction
impl Solver {
    fn visit_stmt(&mut self, stmt: &Stmt) -> Result<(), TypeError> {
        match stmt.inner() {
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
            StmtKind::WithLoop(WithLoop { .. }) => {
                // TODO: Instance ID / Object ID!
            }
            StmtKind::RepeatLoop(RepeatLoop { tick_counts, .. }) => {
                tick_counts.unify(&mut Ty::Real, self)?;
            }
            StmtKind::ForLoop(ForLoop { condition, .. })
            | StmtKind::DoUntil(DoUntil { condition, .. })
            | StmtKind::WhileLoop(WhileLoop { condition, .. })
            | StmtKind::If(If { condition, .. }) => {
                condition.unify(&mut Ty::Bool, self)?;
            }
            StmtKind::Switch(Switch {
                matching_value, cases, ..
            }) => {
                let mut identity = matching_value.query(self)?;
                for case in cases {
                    case.identity().unify(&mut identity, self)?;
                }
            }
            StmtKind::Expr(expr) => {
                expr.query(self)?;
            }
            _ => {}
        }
        Ok(())
    }
}
