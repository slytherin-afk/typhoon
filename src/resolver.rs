use std::collections::HashMap;

use crate::{
    expr::{self, Expr, ExprVisitor},
    object::{Object, ResolvableFunction},
    stmt::{self, Stmt, StmtVisitor},
    token::Token,
    Interpreter, Lib,
};

#[derive(Clone)]
enum FunctionType {
    Function,
    Initializer,
    Static,
    Method,
    None,
}

#[derive(Clone)]
enum ClassType {
    None,
    Class,
    SubClass,
}

pub struct Resolver<'a> {
    interpreter: &'a mut Interpreter,
    scopes: Vec<HashMap<String, bool>>,
    unused_variables: Vec<HashMap<String, Token>>,
    function_type: FunctionType,
    class_type: ClassType,
    loop_depth: usize,
    function_depth: usize,
}

impl<'a> Resolver<'a> {
    pub fn new(interpreter: &'a mut Interpreter) -> Self {
        Self {
            interpreter,
            scopes: vec![],
            unused_variables: vec![],
            function_type: FunctionType::None,
            class_type: ClassType::None,
            loop_depth: 0,
            function_depth: 0,
        }
    }

    fn resolve_expression(&mut self, expr: &Expr) {
        expr.accept(self)
    }

    fn resolve_stmt(&mut self, stmt: &Stmt) {
        stmt.accept(self)
    }

    pub fn resolve_stmts(&mut self, stmts: &Vec<Stmt>) {
        for stmt in stmts {
            stmt.accept(self)
        }
    }

    fn resolve_function<T: ResolvableFunction>(
        &mut self,
        function: &T,
        function_type: FunctionType,
    ) {
        let enclosing = self.function_type.clone();
        self.function_type = function_type;
        self.function_depth += 1;
        self.begin_scope();

        for param in function.params() {
            self.declare(param);
            self.define(param);
        }

        self.resolve_stmts(function.body());
        self.end_scope();
        self.function_depth -= 1;
        self.function_type = enclosing;
    }

    fn resolve_local(&mut self, name: &Token) {
        for i in (0..self.scopes.len()).rev() {
            if self.scopes[i].contains_key(&name.lexeme) {
                self.unused_variables[i].remove(&name.lexeme);
                self.interpreter.resolve(
                    &name.identifier_hash.as_ref().unwrap(),
                    self.scopes.len() - 1 - i,
                );
            }
        }
    }

    fn begin_scope(&mut self) {
        self.unused_variables.push(HashMap::new());
        self.scopes.push(HashMap::new());
    }

    fn end_scope(&mut self) {
        if let Some(unused_vars) = self.unused_variables.pop() {
            for unused in unused_vars.into_values() {
                Lib::warn_token(&unused, "Unused variable");
            }
        }

        self.scopes.pop();
    }

    fn declare(&mut self, name: &Token) {
        if self.scopes.is_empty() {
            return;
        }

        self.unused_variables
            .last_mut()
            .unwrap()
            .insert(String::from(&name.lexeme), name.clone());

        self.scopes
            .last_mut()
            .unwrap()
            .insert(String::from(&name.lexeme), false);
    }

    fn define(&mut self, name: &Token) {
        if self.scopes.is_empty() {
            return;
        }

        self.scopes
            .last_mut()
            .unwrap()
            .insert(String::from(&name.lexeme), true);
    }
}

impl<'a> ExprVisitor for Resolver<'a> {
    type Item = ();

    fn visit_comma(&mut self, expr: &expr::Comma) -> Self::Item {
        self.resolve_expression(&expr.left);
        self.resolve_expression(&expr.right);
    }

    fn visit_lambda(&mut self, expr: &expr::Lambda) -> Self::Item {
        self.resolve_function(expr, FunctionType::Function);
    }

    fn visit_assignment(&mut self, expr: &expr::Assignment) -> Self::Item {
        self.resolve_expression(&expr.value);
        self.resolve_local(&expr.name);
    }

    fn visit_set(&mut self, expr: &expr::Set) -> Self::Item {
        self.resolve_expression(&expr.value);
        self.resolve_expression(&expr.object);
    }

    fn visit_ternary(&mut self, expr: &expr::Ternary) -> Self::Item {
        self.resolve_expression(&expr.condition);
        self.resolve_expression(&expr.truth);
        self.resolve_expression(&expr.falsy);
    }

    fn visit_logical(&mut self, expr: &expr::Logical) -> Self::Item {
        self.resolve_expression(&expr.left);
        self.resolve_expression(&expr.right);
    }

    fn visit_binary(&mut self, expr: &expr::Binary) -> Self::Item {
        self.resolve_expression(&expr.left);
        self.resolve_expression(&expr.right);
    }

    fn visit_unary(&mut self, expr: &expr::Unary) -> Self::Item {
        self.resolve_expression(&expr.right);
    }

    fn visit_call(&mut self, expr: &expr::Call) -> Self::Item {
        self.resolve_expression(&expr.callee);

        for arg in &expr.arguments {
            self.resolve_expression(&arg);
        }
    }

    fn visit_get(&mut self, expr: &expr::Get) -> Self::Item {
        self.resolve_expression(&expr.object);
    }

    fn visit_grouping(&mut self, expr: &Expr) -> Self::Item {
        self.resolve_expression(expr);
    }

    fn visit_variable(&mut self, expr: &Token) -> Self::Item {
        if !self.scopes.is_empty() {
            if let Some(&false) = self.scopes.last().unwrap().get(&expr.lexeme) {
                Lib::error_token(expr, "Can't read local variable in its own initializer.");
            }
        }

        self.resolve_local(expr);
    }

    fn visit_this(&mut self, expr: &Token) -> Self::Item {
        if matches!(self.class_type, ClassType::None) {
            Lib::error_token(&expr, "Can't use 'this' outside a class method");
        }

        if matches!(self.function_type, FunctionType::Static) {
            Lib::error_token(&expr, "Can't use 'this' inside a static method");
        }

        self.resolve_local(expr);
    }

    fn visit_super(&mut self, expr: &expr::Super) -> Self::Item {
        if matches!(self.class_type, ClassType::None) {
            Lib::error_token(&expr.keyword, "Can't use 'super' outside a class method");
        }

        if matches!(self.class_type, ClassType::Class) {
            Lib::error_token(
                &expr.keyword,
                "Can't use 'super' inside a class with no super class",
            );
        }

        if matches!(self.function_type, FunctionType::Static) {
            Lib::error_token(&expr.keyword, "Can't use 'super' inside a static method");
        }

        self.resolve_local(&expr.keyword);
    }

    fn visit_literal(&mut self, _: &Object) -> Self::Item {}
}

impl<'a> StmtVisitor for Resolver<'a> {
    type Item = ();

    fn visit_empty_stmt(&mut self) -> Self::Item {}

    fn visit_expression_stmt(&mut self, stmt: &Expr) -> Self::Item {
        self.resolve_expression(stmt);
    }

    fn visit_print_stmt(&mut self, stmt: &Expr) -> Self::Item {
        self.resolve_expression(stmt);
    }

    fn visit_variable_stmt(&mut self, stmt: &Vec<stmt::VariableDeclaration>) -> Self::Item {
        for variable in stmt {
            self.declare(&variable.name);

            if let Some(initializer) = &variable.initializer {
                self.resolve_expression(initializer);
            }

            self.define(&variable.name);
        }
    }

    fn visit_block_stmt(&mut self, stmt: &Vec<Stmt>) -> Self::Item {
        self.begin_scope();
        self.resolve_stmts(stmt);
        self.end_scope();
    }

    fn visit_if_stmt(&mut self, stmt: &stmt::If) -> Self::Item {
        self.resolve_expression(&stmt.condition);
        self.resolve_stmt(&stmt.truth);

        if let Some(falsy) = &stmt.falsy {
            self.resolve_stmt(falsy);
        }
    }

    fn visit_while_stmt(&mut self, stmt: &stmt::While) -> Self::Item {
        self.loop_depth += 1;
        self.resolve_expression(&stmt.condition);
        self.resolve_stmt(&stmt.body);
        self.loop_depth -= 1;
    }

    fn visit_break_stmt(&mut self, keyword: &Token) -> Self::Item {
        if self.loop_depth == 0 {
            Lib::error_token(keyword, "Can't use 'break' outside a loop");
        } else if self.function_depth >= self.loop_depth {
            Lib::error_token(keyword, "Jump target 'cannot' cross function boundary");
        }
    }

    fn visit_continue_stmt(&mut self, keyword: &Token) -> Self::Item {
        if self.loop_depth == 0 {
            Lib::error_token(keyword, "Can't use 'continue' outside a loop");
        } else if self.function_depth >= self.loop_depth {
            Lib::error_token(keyword, "Jump target cannot cross function boundary");
        }
    }

    fn visit_function_stmt(&mut self, stmt: &stmt::Function) -> Self::Item {
        self.declare(&stmt.name);
        self.define(&stmt.name);
        self.resolve_function(stmt, FunctionType::Function);
    }

    fn visit_return_stmt(&mut self, stmt: &stmt::Return) -> Self::Item {
        if self.function_depth == 0 {
            Lib::error_token(&stmt.keyword, "Can't use 'return' outside a function");
        }

        if let Some(value) = &stmt.value {
            if matches!(self.function_type, FunctionType::Initializer) {
                Lib::error_token(&stmt.keyword, "Can't return a value from initializer");
            }

            self.resolve_expression(value);
        }
    }

    fn visit_class_stmt(&mut self, stmt: &stmt::Class) -> Self::Item {
        let enclosing = self.class_type.clone();
        self.class_type = ClassType::Class;

        self.declare(&stmt.name);
        self.define(&stmt.name);

        if let Some(super_class) = &stmt.super_class {
            self.class_type = ClassType::SubClass;
            self.resolve_expression(super_class);

            self.begin_scope();

            self.scopes
                .last_mut()
                .unwrap()
                .insert(String::from("super"), true);

            if let Expr::Variable(super_class) = super_class {
                if super_class.lexeme == stmt.name.lexeme {
                    Lib::error_token(&super_class, "A class can't inherit from itself");
                }
            }
        }

        self.begin_scope();

        for method in &stmt.statics {
            let declaration = FunctionType::Static;

            if let Stmt::Function(function_stmt) = method {
                self.resolve_function(&**function_stmt, declaration);
            }
        }

        self.scopes
            .last_mut()
            .unwrap()
            .insert(String::from("this"), true);

        for method in &stmt.methods {
            let mut declaration = FunctionType::Method;

            if let Stmt::Function(function_stmt) = method {
                if function_stmt.name.lexeme.eq("init") {
                    declaration = FunctionType::Initializer;
                }

                self.resolve_function(&**function_stmt, declaration);
            }
        }

        self.end_scope();

        if let Some(_) = &stmt.super_class {
            self.end_scope();
        }

        self.class_type = enclosing;
    }
}
