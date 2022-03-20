use super::{Application, Constraint, Deref, Marker, Symbol, Type};
use crate::{
    parse::{
        Access, Assignment, AssignmentOp, Ast, Block, Call, Equality, Evaluation, Expr, ExprId, ExprType, Function,
        Grouping, Identifier, Literal, LocalVariableSeries, Logical, NullCoalecence, OptionalInitilization,
        ParseVisitor, Postfix, Return, Stmt, StmtType, Ternary, Unary, UnaryOp,
    },
    FileId,
};
use codespan_reporting::diagnostic::{Diagnostic, Label};
use hashbrown::HashMap;

#[derive(Debug, Default)]
pub struct TypeWriter;
impl TypeWriter {
    pub fn write_types(&mut self, ast: &mut Ast) -> Page {
        let mut page = Page::default();
        page.apply_stmts(ast.stmts_mut());
        page
    }
}

#[derive(Debug, Default)]
pub struct Page {
    pub fields: HashMap<String, Marker>,
    pub markers: HashMap<ExprId, Marker>,
    pub marker_iter: u64,
    pub constraints: Vec<Constraint>,
    pub substitutions: HashMap<Marker, Symbol>,
    pub file_id: FileId,
}

// Unification
impl Page {
    pub fn apply_stmts(&mut self, stmts: &mut Vec<Stmt>) {
        // Constrain everything
        for stmt in stmts.iter_mut() {
            self.constrain_stmt(stmt);
        }

        // Sub everything
        for con in self.constraints.iter() {
            println!("{con}");
        }
        self.constraints.reverse();
        let mut constraints = self.constraints.clone();

        while let Some(mut pattern) = constraints.pop() {
            self.add_substitution(pattern.marker, pattern.symbol.clone());
            if let Some(sub) = self.find_substitute_recurse(&pattern.symbol) {
                pattern.symbol = sub;
            }

            for target in constraints.iter_mut() {
                if let Some(sub) = self.find_substitute_recurse(&target.symbol) {
                    target.symbol = sub;
                } else if target.marker == pattern.marker {
                    // We can do a distributive transfer (if a == b and a == c, b == c)
                    match &target.symbol {
                        Symbol::Constant(_) => {}
                        Symbol::Variable(marker) => {
                            self.add_substitution(*marker, pattern.symbol.clone());
                        }
                        Symbol::Application(_) => {}
                        Symbol::Deref(deref) => match deref {
                            Deref::Array(dereffed_marker) => {
                                let new_symbol = Symbol::Application(Application::Array {
                                    member_type: Box::new(pattern.symbol.clone()),
                                });
                                self.add_substitution(*dereffed_marker, new_symbol);
                            }
                            Deref::Object(_, _) => todo!(),
                        },
                        Symbol::Union(_) => {}
                    }
                }
            }
        }
        for (marker, symbol) in self.substitutions.iter() {
            println!("{} => {}", marker, symbol);
        }
    }

    fn add_substitution(&mut self, marker: Marker, symbol: Symbol) {
        let new_sub = Constraint { marker, symbol };
        let mut updates = vec![];
        for (marker, symbol) in self.substitutions.iter() {
            if let Some(new_symbol) = self.find_substitute_recurse(symbol) {
                updates.push((*marker, new_symbol));
            }
        }
        for (marker, new_symbol) in updates {
            *self.substitutions.get_mut(&marker).unwrap() = new_symbol;
        }
        let Constraint { marker, symbol } = new_sub;
        self.substitutions.insert(marker, symbol);
    }
}

// Scope getters
impl Page {
    pub fn has_field(&self, name: &str) -> bool {
        self.fields.contains_key(name)
    }

    pub fn field_marker(&self, identifier: &Identifier) -> Result<Marker, Diagnostic<FileId>> {
        match self.fields.get(&identifier.lexeme).copied() {
            Some(marker) => Ok(marker),
            None => Err(Diagnostic::error()
                .with_message(format!("Unrecognized variable: {}", identifier.lexeme))
                .with_labels(vec![
                    Label::primary(self.file_id, identifier.span).with_message("not found in current scope"),
                ])),
        }
    }

    pub fn field_type(&self, identifier: &Identifier) -> Result<Type, Diagnostic<FileId>> {
        self.field_marker(identifier).map(|marker| self.seek_type_for(marker))
    }

    pub fn return_type(&self) -> Type {
        let tpe = self.seek_type_for(Marker::RETURN_VALUE);
        if let Type::Generic {
            marker: Marker::RETURN_VALUE,
        } = tpe
        {
            Type::Undefined
        } else {
            tpe
        }
    }

    fn seek_type_for(&self, marker: Marker) -> Type {
        let symbol = Symbol::Variable(marker);
        self.find_substitute_recurse(&symbol).unwrap_or(symbol).into()
    }

    fn find_substitute_recurse(&self, symbol: &Symbol) -> Option<Symbol> {
        if let Some(mut inner) = self.find_substitute(symbol) {
            while let Some(new_symbol) = self.find_substitute(&inner) {
                inner = new_symbol;
            }
            Some(inner)
        } else {
            None
        }
    }

    fn find_substitute(&self, symbol: &Symbol) -> Option<Symbol> {
        match symbol {
            Symbol::Variable(marker) => {
                if let Some(sub) = self.substitutions.get(marker) {
                    return Some(sub.clone());
                }
            }
            Symbol::Application(Application::Array { member_type }) => {
                if let Some(member_sub) = self.find_substitute(member_type) {
                    return Some(Symbol::Application(Application::Array {
                        member_type: Box::new(member_sub),
                    }));
                }
            }
            Symbol::Application(Application::Object { fields }) => {
                let mut new_fields = fields.clone();
                let mut any_changed = false;
                for (_, field) in new_fields.iter_mut() {
                    if let Some(new_symbol) = self.find_substitute(field) {
                        any_changed = true;
                        *field = new_symbol;
                    }
                }
                if any_changed {
                    return Some(Symbol::Application(Application::Object { fields: new_fields }));
                }
            }
            Symbol::Deref(Deref::Array(inner_marker)) => {
                let member_type = self.substitutions.get(inner_marker).and_then(|sub| {
                    if let Symbol::Application(Application::Array { member_type }) = sub {
                        Some(member_type)
                    } else {
                        None
                    }
                });
                if let Some(member_type) = member_type {
                    return Some(member_type.as_ref().clone());
                }
            }
            Symbol::Deref(Deref::Object(inner_marker, field_name)) => {
                if let Some(Symbol::Application(Application::Object { fields })) = self.substitutions.get(inner_marker)
                {
                    return Some(
                        fields
                            .get(field_name)
                            .expect("struct did not have required field")
                            .clone(),
                    );
                }
            }
            Symbol::Constant(_) => {}
            Symbol::Union(_) => todo!(),
        }
        None
    }
}

// Scope creation
impl Page {
    fn new_marker(&mut self, expr: &Expr) -> Marker {
        let marker = Marker(self.marker_iter);
        self.alias_expr_to_marker(expr, marker);
        self.marker_iter += 1;
        marker
    }

    fn alias_expr_to_marker(&mut self, expr: &Expr, marker: Marker) {
        self.markers.insert(expr.id, marker);
    }

    fn new_field(&mut self, name: impl Into<String>, expr: &Expr) {
        let marker = self.new_marker(expr);
        self.fields.insert(name.into(), marker);
        println!("{marker}: {expr}");
    }

    fn get_expr_marker(&mut self, expr: &Expr) -> Marker {
        match self.markers.get(&expr.id).copied() {
            Some(marker) => marker,
            None => {
                let marker = self.new_marker(expr);
                self.markers.insert(expr.id, marker);
                println!("{marker}: {expr}");
                marker
            }
        }
    }
}

// Constraining
impl Page {
    fn constrain_stmt(&mut self, stmt: &mut Stmt) {
        stmt.visit_child_stmts_mut(|stmt| self.constrain_stmt(stmt));
        stmt.visit_child_exprs_mut(|expr| self.constrain_expr(expr));
        match stmt.inner_mut() {
            StmtType::Assignment(Assignment {
                left,
                op: AssignmentOp::Identity(_),
                right,
            }) => {
                if let ExprType::Identifier(iden) = left.inner_mut() {
                    if !self.has_field(&iden.lexeme) {
                        self.new_field(iden.lexeme.clone(), left);
                        self.constraint_expr_to_expr(left, right);
                    } else {
                        // validate that the new type is equal to the last type? shadowing is a
                        // problem
                    }
                }
            }
            StmtType::LocalVariableSeries(LocalVariableSeries { declarations }) => {
                for initializer in declarations.iter() {
                    if let OptionalInitilization::Uninitialized(expr) = initializer {
                        let iden = initializer.name_identifier();
                        if !self.has_field(&iden.lexeme) {
                            self.new_field(iden.lexeme.clone(), expr);
                            self.constrain_expr_to_type(expr, Type::Undefined);
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
                    let marker = self.get_expr_marker(value);
                    self.constrain_marker(Marker::RETURN_VALUE, Symbol::Variable(marker));
                } else {
                    self.constrain_marker(Marker::RETURN_VALUE, Symbol::Constant(Type::Undefined));
                }
            }

            // todo: constrain types required for these things
            _ => {}
        }
    }

    fn constrain_expr(&mut self, expr: &mut Expr) {
        if let ExprType::FunctionDeclaration(function) = expr.inner_mut() {
            match &function.constructor {
                Some(_) => todo!(),
                None => {
                    let mut body_page = Page::default();
                    for param in function.parameters.iter() {
                        body_page.new_field(param.name(), param.name_expr())
                    }
                    let body = match function.body.inner_mut() {
                        StmtType::Block(Block { body, .. }) => body,
                        _ => unreachable!(),
                    };
                    body_page.apply_stmts(body);
                    let mut parameter_types = Vec::new();
                    for param in function.parameters.iter() {
                        parameter_types.push(body_page.seek_type_for(self.get_expr_marker(param.name_expr())));
                    }
                    self.constrain_expr_to_type(
                        expr,
                        Type::Function {
                            parameters: parameter_types,
                            return_type: Box::new(body_page.return_type()),
                        },
                    )
                }
            }

            // We return, as *we* handeled visiting the children.
            return;
        }

        expr.visit_child_stmts_mut(|stmt| self.constrain_stmt(stmt));
        expr.visit_child_exprs_mut(|expr| self.constrain_expr(expr));
        match expr.inner() {
            ExprType::FunctionDeclaration(_) => {}
            ExprType::Logical(Logical { left, right, .. }) => {
                self.constrain_expr_to_type(right, Type::Bool);
                self.constrain_expr_to_type(expr, Type::Bool);
            }
            ExprType::Equality(Equality { left, right, .. }) => {
                self.constraint_expr_to_expr(right, left);
                self.constrain_expr_to_type(expr, Type::Bool);
            }
            ExprType::Evaluation(Evaluation { left, right, .. }) => {
                self.constraint_expr_to_expr(right, left);
                self.constraint_expr_to_expr(expr, left);
            }
            ExprType::NullCoalecence(NullCoalecence { left, right }) => {
                self.constraint_expr_to_expr(right, left);
                self.constraint_expr_to_expr(expr, left);
            }
            ExprType::Ternary(Ternary {
                condition,
                true_value,
                false_value,
            }) => {
                self.constrain_expr_to_type(condition, Type::Bool);
                self.constraint_expr_to_expr(false_value, true_value);
                self.constraint_expr_to_expr(expr, true_value);
            }
            ExprType::Unary(Unary { op, right }) => match op {
                UnaryOp::Increment(_)
                | UnaryOp::Decrement(_)
                | UnaryOp::Positive(_)
                | UnaryOp::Negative(_)
                | UnaryOp::BitwiseNot(_) => {
                    self.constrain_expr_to_type(right, Type::Real);
                    self.constrain_expr_to_type(expr, Type::Real);
                }
                UnaryOp::Not(_) => {
                    self.constrain_expr_to_type(right, Type::Bool);
                    self.constrain_expr_to_type(expr, Type::Bool);
                }
            },
            ExprType::Postfix(Postfix { left, .. }) => {
                self.constrain_expr_to_type(left, Type::Real);
                self.constrain_expr_to_type(expr, Type::Real);
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
                        let deref = Deref::Object(self.get_expr_marker(left), right.lexeme.clone());
                        self.constraint_expr_to_deref(expr, deref);
                    }
                    Access::Array {
                        left,
                        index_one,
                        index_two,
                        ..
                    } => {
                        // our indexes must be real
                        self.constrain_expr_to_type(index_one, Type::Real);
                        if let Some(index_two) = index_two {
                            self.constrain_expr_to_type(index_two, Type::Real);
                        }

                        // meanwhile, our type is a deref of the left's type
                        let deref = Deref::Array(self.get_expr_marker(left));
                        self.constraint_expr_to_deref(expr, deref);
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
                self.constraint_expr_to_expr(expr, inner);
            }

            ExprType::Identifier(iden) => {
                if let Ok(marker) = self.field_marker(&iden) {
                    self.alias_expr_to_marker(expr, marker);
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
                        let app = if let Some(marker) = exprs.first().map(|expr| self.get_expr_marker(expr)) {
                            Application::Array {
                                member_type: Box::new(Symbol::Variable(marker)),
                            }
                        } else {
                            Application::Array {
                                member_type: Box::new(Symbol::Constant(Type::Unknown)),
                            } // todo will this resolve?
                        };
                        self.constraint_expr_to_application(expr, app);
                        return;
                    }
                    Literal::Struct(declarations) => {
                        // We can construct a type for this since we'll know the structure of the fields,
                        // even if we don't know the type of the fields themselves
                        let mut fields = HashMap::default();
                        for declaration in declarations {
                            fields.insert(
                                declaration.0.lexeme.clone(),
                                Symbol::Variable(self.get_expr_marker(&declaration.1)),
                            );
                        }
                        self.constraint_expr_to_application(expr, Application::Object { fields });
                        return;
                    }
                };
                self.constrain_expr_to_type(expr, tpe);
            }
        }
    }

    fn constrain_expr_to_type(&mut self, target: &Expr, tpe: Type) {
        self.constrain(target, Symbol::Constant(tpe));
    }

    fn constraint_expr_to_expr(&mut self, target: &Expr, expr: &Expr) {
        let marker = self.get_expr_marker(expr);
        self.constrain(target, Symbol::Variable(marker));
    }

    fn constraint_expr_to_deref(&mut self, target: &Expr, deref: Deref) {
        self.constrain(target, Symbol::Deref(deref));
    }

    fn constraint_expr_to_application(&mut self, target: &Expr, application: Application) {
        self.constrain(target, Symbol::Application(application));
    }

    fn constrain(&mut self, expr: &Expr, symbol: Symbol) {
        let marker = self.get_expr_marker(expr);
        self.constrain_marker(marker, symbol);
    }

    fn constrain_marker(&mut self, marker: Marker, symbol: Symbol) {
        let constraint = Constraint { marker, symbol };
        self.constraints.push(constraint);
    }
}
