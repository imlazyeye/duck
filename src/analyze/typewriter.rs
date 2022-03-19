use super::{Application, Constraint, Deref, Marker, Symbol, Type};
use crate::parse::{
    Access, Assignment, AssignmentOp, Ast, Call, Equality, Evaluation, Expr, ExprType, Function, Grouping, Literal,
    LocalVariableSeries, Logical, NullCoalecence, OptionalInitilization, ParseVisitor, Postfix, Stmt, StmtType,
    Ternary, Unary, UnaryOp,
};
use colored::Colorize;
use hashbrown::HashMap;

#[derive(Debug, Default)]
pub struct TypeWriter {
    alias_iter: u64,
}
impl TypeWriter {
    pub fn write_types(&mut self, ast: &mut Ast) -> Page {
        // If we're testing, we'll alias all the markers, just to make things easier to read
        if cfg!(test) {
            for stmt in ast.stmts_mut() {
                self.alias_stmt(stmt);
            }
        }

        let mut page = Page::default();
        Self::apply_stmts_to_page(ast.stmts_mut(), &mut page);

        // With the results, update our ast
        // eventually we won't do this?
        for stmt in ast.stmts_mut() {
            Self::finalize_stmt(stmt, &page);
        }
        page
    }

    fn apply_stmts_to_page(stmts: &mut Vec<Stmt>, page: &mut Page) {
        // Constrain everything
        for stmt in stmts.iter_mut() {
            Self::constrain_stmt(stmt, page);
        }

        // Sub everything
        page.constraints.reverse();
        page.substitutions = page
            .constraints
            .iter()
            .cloned()
            .map(|Constraint { marker, symbol }| (marker, symbol))
            .collect::<HashMap<Marker, Symbol>>();
        while let Some(Constraint { marker, symbol }) = page.constraints.pop() {
            for test_constraint in page.constraints.iter_mut() {
                let previous = test_constraint.symbol.clone();
                if Self::update_symbol(&mut test_constraint.symbol, marker, &symbol) {
                    println!("{} => {}", previous, test_constraint.symbol);
                }
            }
            page.substitutions.insert(marker, symbol);
        }
    }

    fn update_symbol(symbol: &mut Symbol, marker: Marker, new_target: &Symbol) -> bool {
        match symbol {
            Symbol::Variable(inner_marker) => {
                if *inner_marker == marker {
                    *symbol = new_target.clone();
                    true
                } else {
                    false
                }
            }
            Symbol::Application(application) => match application {
                Application::Array { member_type: inner } => Self::update_symbol(inner, marker, new_target),
                Application::Object { fields } => {
                    let mut any = false;
                    for (_, field_symbol) in fields {
                        if Self::update_symbol(field_symbol, marker, new_target) {
                            any = true;
                        }
                    }
                    any
                }
            },
            Symbol::Deref(deref) => match deref {
                Deref::Array(inner_marker) => {
                    if *inner_marker == marker {
                        match new_target {
                            Symbol::Variable(new_marker) => *inner_marker = *new_marker,
                            Symbol::Application(Application::Array { member_type: inner }) => {
                                *symbol = inner.as_ref().clone()
                            }
                            _ => panic!("cannot access {inner_marker} with a {new_target}"),
                        }
                        true
                    } else {
                        false
                    }
                }
                Deref::Object(inner_marker, key) => {
                    if *inner_marker == marker {
                        match new_target {
                            Symbol::Variable(new_marker) => *inner_marker = *new_marker,
                            Symbol::Application(Application::Object { fields }) => {
                                *symbol = fields.get(key).expect("couldn't find a field on a struct").clone()
                            }
                            _ => panic!("cannot access {inner_marker} with a {new_target}"),
                        }
                        true
                    } else {
                        false
                    }
                }
            },
            Symbol::Union(unions) => {
                let mut any = false;
                for union in unions {
                    if Self::update_symbol(union, marker, new_target) {
                        any = true;
                    }
                }
                any
            }
            Symbol::Constant(_) => false,
        }
    }

    fn constrain_stmt(stmt: &mut Stmt, page: &mut Page) {
        stmt.visit_child_stmts_mut(|stmt| Self::constrain_stmt(stmt, page));
        stmt.visit_child_exprs_mut(|expr| Self::constrain_expr(expr, page));
        match stmt.inner_mut() {
            StmtType::Assignment(Assignment {
                left,
                op: AssignmentOp::Identity(_),
                right,
            }) => {
                if let ExprType::Identifier(iden) = left.inner_mut() {
                    if !page.fields.contains_key(&iden.lexeme) {
                        page.fields.insert(iden.lexeme.clone(), left.marker);
                        page.constrain_to_expr(left, right);
                    } else {
                        // validate that the new type is equal to the last type? shadowing is a
                        // problem
                    }
                }
            }
            StmtType::LocalVariableSeries(LocalVariableSeries { declarations }) => {
                for initializer in declarations.iter() {
                    if let OptionalInitilization::Uninitialized(expr) = initializer {
                        if !page.fields.contains_key(initializer.name()) {
                            page.fields.insert(initializer.name().into(), expr.marker);
                            page.constrain_to_type(expr, Type::Undefined);
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

            // todo: constrain types required for these things
            _ => {}
        }
    }

    fn constrain_expr(expr: &mut Expr, page: &mut Page) {
        expr.visit_child_stmts_mut(|stmt| Self::constrain_stmt(stmt, page));
        expr.visit_child_exprs_mut(|expr| Self::constrain_expr(expr, page));
        match expr.inner() {
            ExprType::FunctionDeclaration(Function {
                name,
                parameters,
                constructor,
                body,
            }) => match constructor {
                Some(_) => todo!(),
                None => {
                    let mut parameter_types = HashMap::new();
                    for param in parameters {
                        let symbol = if let Some(value) = param.assignment_value() {
                            Symbol::Variable(value.marker)
                        } else {
                            Symbol::Constant(Type::Unknown)
                        };
                        parameter_types.insert(param.name().to_string(), symbol);
                    }
                }
            },
            ExprType::Logical(Logical { left, right, .. }) => {
                page.constrain_to_type(right, Type::Bool);
                page.constrain_to_type(expr, Type::Bool);
            }
            ExprType::Equality(Equality { left, right, .. }) => {
                page.constrain_to_expr(right, left);
                page.constrain_to_type(expr, Type::Bool);
            }
            ExprType::Evaluation(Evaluation { left, right, .. }) => {
                page.constrain_to_expr(right, left);
                page.constrain_to_expr(expr, left);
            }
            ExprType::NullCoalecence(NullCoalecence { left, right }) => {
                page.constrain_to_expr(right, left);
                page.constrain_to_expr(expr, left);
            }
            ExprType::Ternary(Ternary {
                condition,
                true_value,
                false_value,
            }) => {
                page.constrain_to_type(condition, Type::Bool);
                page.constrain_to_expr(false_value, true_value);
                page.constrain_to_expr(expr, true_value);
            }
            ExprType::Unary(Unary { op, right }) => match op {
                UnaryOp::Increment(_)
                | UnaryOp::Decrement(_)
                | UnaryOp::Positive(_)
                | UnaryOp::Negative(_)
                | UnaryOp::BitwiseNot(_) => {
                    page.constrain_to_type(right, Type::Real);
                    page.constrain_to_type(expr, Type::Real);
                }
                UnaryOp::Not(_) => {
                    page.constrain_to_type(right, Type::Bool);
                    page.constrain_to_type(expr, Type::Bool);
                }
            },
            ExprType::Postfix(Postfix { left, .. }) => {
                page.constrain_to_type(left, Type::Real);
                page.constrain_to_type(expr, Type::Real);
            }
            ExprType::Access(access) => {
                match access {
                    Access::Global { right } => {
                        // read the global scope for the type?
                    }
                    Access::Current { right } => {
                        // read the current scope for the type?
                    }
                    Access::Other { right } => {
                        // read the above scope for the type?
                    }
                    Access::Dot { left, right } => {
                        page.constrain_to_deref(expr, Deref::Object(left.marker, right.lexeme.clone()));
                    }
                    Access::Array {
                        left,
                        index_one,
                        index_two,
                        ..
                    } => {
                        // our indexes must be real
                        page.constrain_to_type(index_one, Type::Real);
                        if let Some(index_two) = index_two {
                            page.constrain_to_type(index_two, Type::Real);
                        }

                        // meanwhile, our type is a deref of the left's type
                        page.constrain_to_deref(expr, Deref::Array(left.marker));
                    }
                    Access::Struct { left, key } => {}
                    _ => todo!(),
                }
            }
            ExprType::Call(Call {
                left,
                arguments,
                uses_new,
            }) => {}
            ExprType::Grouping(Grouping { inner, .. }) => {
                page.constrain_to_expr(expr, inner);
            }

            ExprType::Identifier(iden) => {
                // if this identifier is already in scope, then we need to remap this to the previous declaration
                if let Some(marker) = page.fields.get(&iden.lexeme).copied() {
                    expr.marker = marker;
                } else {
                    // if its not in scope, then we can't constrain it to anything
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
                        let app = if let Some(marker) = exprs.first().map(|expr| expr.marker) {
                            Application::Array {
                                member_type: Box::new(Symbol::Variable(marker)),
                            }
                        } else {
                            Application::Array {
                                member_type: Box::new(Symbol::Constant(Type::Unknown)),
                            } // todo will this resolve?
                        };
                        page.constrain_to_application(expr, app);
                        return;
                    }
                    Literal::Struct(declarations) => {
                        // We can construct a type for this since we'll know the structure of the fields,
                        // even if we don't know the type of the fields themselves
                        let mut fields = HashMap::default();
                        for declaration in declarations {
                            fields.insert(declaration.0.lexeme.clone(), Symbol::Variable(declaration.1.marker));
                        }
                        page.constrain_to_application(expr, Application::Object { fields });
                        return;
                    }
                };
                page.constrain_to_type(expr, tpe);
            }
        }
    }

    fn alias_stmt(&mut self, stmt: &mut Stmt) {
        stmt.visit_child_stmts_mut(|stmt| self.alias_stmt(stmt));
        stmt.visit_child_exprs_mut(|expr| self.alias_expr(expr));
    }

    fn alias_expr(&mut self, expr: &mut Expr) {
        expr.marker.0 = self.alias_iter;
        println!(
            "{}: {}",
            expr.marker.to_string().bright_cyan(),
            expr.to_string().bright_white().bold(),
        );
        self.alias_iter += 1;
        expr.visit_child_stmts_mut(|stmt| self.alias_stmt(stmt));
        expr.visit_child_exprs_mut(|expr| self.alias_expr(expr));
    }

    fn finalize_stmt(stmt: &mut Stmt, page: &Page) {
        stmt.visit_child_stmts_mut(|stmt| Self::finalize_stmt(stmt, page));
        stmt.visit_child_exprs_mut(|expr| Self::finalize_expr(expr, page));
    }

    fn finalize_expr(expr: &mut Expr, page: &Page) {
        expr.tpe = page.seek_type_for(expr.marker);
        expr.visit_child_stmts_mut(|stmt| Self::finalize_stmt(stmt, page));
        expr.visit_child_exprs_mut(|expr| Self::finalize_expr(expr, page));
    }
}

#[derive(Debug, Default)]
pub struct Page {
    pub fields: HashMap<String, Marker>,
    pub constraints: Vec<Constraint>,
    pub substitutions: HashMap<Marker, Symbol>,
    pub yielded_type: Option<Type>,
}
impl Page {
    fn constrain_to_type(&mut self, target: &Expr, tpe: Type) {
        let constraint = Constraint {
            marker: target.marker,
            symbol: Symbol::Constant(tpe),
        };
        println!("{constraint}");
        self.constraints.push(constraint);
    }

    fn constrain_to_expr(&mut self, target: &Expr, expr: &Expr) {
        let constraint = Constraint {
            marker: target.marker,
            symbol: self.seek_symbol_for(expr.marker),
        };
        println!("{constraint}");
        self.constraints.push(constraint);
    }

    fn constrain_to_deref(&mut self, target: &Expr, deref: Deref) {
        let constraint = Constraint {
            marker: target.marker,
            symbol: Symbol::Deref(deref),
        };
        println!("{constraint}");
        self.constraints.push(constraint);
    }

    fn constrain_to_application(&mut self, target: &Expr, application: Application) {
        let constraint = Constraint {
            marker: target.marker,
            symbol: Symbol::Application(application),
        };
        println!("{constraint}");
        self.constraints.push(constraint);
    }

    pub fn seek_type_for(&self, marker: Marker) -> Type {
        self.seek_symbol_for(marker).into()
    }

    fn seek_symbol_for(&self, mut marker: Marker) -> Symbol {
        let mut symbol = Symbol::Variable(marker);
        while let Some(symbol) = self.substitutions.get(&marker) {
            match symbol {
                Symbol::Variable(new_marker) => marker = *new_marker,
                symbol => return symbol.clone(),
            }
        }
        symbol
    }
}
