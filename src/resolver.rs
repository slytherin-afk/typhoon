use std::collections::HashMap;

use crate::{
    expression::{
        Assignment, Binary, Call, Comma, Expression, ExpressionVisitor, Get, Lambda, Logical, Set,
        Ternary, Unary,
    },
    stmt::{
        ClassStmt, FunctionStmt, IfStmt, ReturnStmt, Stmt, StmtVisitor, VariableDeclaration,
        WhileStmt,
    },
    Interpreter, Lib, Object, ResolvableFunction, Token,
};

#[derive(Clone)]
enum FunctionType {
    Function,
    Initializer,
    Method,
    None,
}

#[derive(Clone)]
enum ClassType {
    None,
    Class,
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

    fn resolve_expression(&mut self, expr: &Expression) {
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
                Lib::warn_two(&unused, "Unused variable");
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

impl<'a> ExpressionVisitor for Resolver<'a> {
    type Item = ();

    fn visit_comma(&mut self, expr: &Comma) -> Self::Item {
        self.resolve_expression(&expr.left);
        self.resolve_expression(&expr.right);
    }

    fn visit_lambda(&mut self, expr: &Lambda) -> Self::Item {
        self.resolve_function(expr, FunctionType::Function);
    }

    fn visit_assignment(&mut self, expr: &Assignment) -> Self::Item {
        self.resolve_expression(&expr.value);
        self.resolve_local(&expr.name);
    }

    fn visit_ternary(&mut self, expr: &Ternary) -> Self::Item {
        self.resolve_expression(&expr.condition);
        self.resolve_expression(&expr.truth);
        self.resolve_expression(&expr.falsy);
    }

    fn visit_logical(&mut self, expr: &Logical) -> Self::Item {
        self.resolve_expression(&expr.left);
        self.resolve_expression(&expr.right);
    }

    fn visit_set(&mut self, expr: &Set) -> Self::Item {
        self.resolve_expression(&expr.value);
        self.resolve_expression(&expr.object);
    }

    fn visit_binary(&mut self, expr: &Binary) -> Self::Item {
        self.resolve_expression(&expr.left);
        self.resolve_expression(&expr.right);
    }

    fn visit_unary(&mut self, expr: &Unary) -> Self::Item {
        self.resolve_expression(&expr.right);
    }

    fn visit_call(&mut self, expr: &Call) -> Self::Item {
        self.resolve_expression(&expr.callee);

        for arg in &expr.arguments {
            self.resolve_expression(&arg);
        }
    }

    fn visit_get(&mut self, expr: &Get) -> Self::Item {
        self.resolve_expression(&expr.object);
    }

    fn visit_grouping(&mut self, expr: &Expression) -> Self::Item {
        self.resolve_expression(expr);
    }

    fn visit_variable(&mut self, expr: &Token) -> Self::Item {
        if !self.scopes.is_empty() {
            if let Some(&false) = self.scopes.last().unwrap().get(&expr.lexeme) {
                Lib::error_two(expr, "Can't read local variable in its own initializer.");
            }
        }

        self.resolve_local(expr);
    }

    fn visit_this(&mut self, expr: &Token) -> Self::Item {
        if matches!(self.class_type, ClassType::None) {
            Lib::error_two(&expr, "Can't use 'this' outside a class method");
        }
        self.resolve_local(expr);
    }

    fn visit_literal(&mut self, _: &Object) -> Self::Item {}
}

impl<'a> StmtVisitor for Resolver<'a> {
    type Item = ();

    fn visit_empty_stmt(&mut self) -> Self::Item {}

    fn visit_expression_stmt(&mut self, stmt: &Expression) -> Self::Item {
        self.resolve_expression(stmt);
    }

    fn visit_print_stmt(&mut self, stmt: &Expression) -> Self::Item {
        self.resolve_expression(stmt);
    }

    fn visit_variable_stmt(&mut self, stmt: &Vec<VariableDeclaration>) -> Self::Item {
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

    fn visit_if_stmt(&mut self, stmt: &IfStmt) -> Self::Item {
        self.resolve_expression(&stmt.condition);
        self.resolve_stmt(&stmt.truth);

        if let Some(falsy) = &stmt.falsy {
            self.resolve_stmt(falsy);
        }
    }

    fn visit_while_stmt(&mut self, stmt: &WhileStmt) -> Self::Item {
        self.loop_depth += 1;
        self.resolve_expression(&stmt.condition);
        self.resolve_stmt(&stmt.body);
        self.loop_depth -= 1;
    }

    fn visit_break_stmt(&mut self, keyword: &Token) -> Self::Item {
        if self.loop_depth == 0 {
            Lib::error_two(keyword, "Can't use 'break' outside a loop");
        } else if self.function_depth >= self.loop_depth {
            Lib::error_two(keyword, "Jump target 'cannot' cross function boundary");
        }
    }

    fn visit_continue_stmt(&mut self, keyword: &Token) -> Self::Item {
        if self.loop_depth == 0 {
            Lib::error_two(keyword, "Can't use 'continue' outside a loop");
        } else if self.function_depth >= self.loop_depth {
            Lib::error_two(keyword, "Jump target cannot cross function boundary");
        }
    }

    fn visit_function_stmt(&mut self, stmt: &FunctionStmt) -> Self::Item {
        self.declare(&stmt.name);
        self.define(&stmt.name);
        self.resolve_function(stmt, FunctionType::Function);
    }

    fn visit_return_stmt(&mut self, stmt: &ReturnStmt) -> Self::Item {
        if self.function_depth == 0 {
            Lib::error_two(&stmt.keyword, "Can't use 'return' outside a function");
        }

        if let Some(value) = &stmt.value {
            if matches!(self.function_type, FunctionType::Initializer) {
                Lib::error_two(&stmt.keyword, "Can't return a value from initializer");
            }

            self.resolve_expression(value);
        }
    }

    fn visit_class_stmt(&mut self, stmt: &ClassStmt) -> Self::Item {
        let enclosing = self.class_type.clone();
        self.class_type = ClassType::Class;

        self.declare(&stmt.name);
        self.define(&stmt.name);
        self.begin_scope();
        self.scopes
            .last_mut()
            .unwrap()
            .insert(String::from("this"), true);

        for method in &stmt.methods {
            let mut declaration = FunctionType::Method;

            if let Stmt::FunctionStmt(function_stmt) = method {
                if function_stmt.name.lexeme.eq("init") {
                    print!("fsd");
                    declaration = FunctionType::Initializer;
                }

                self.resolve_function(&**function_stmt, declaration);
            }
        }

        self.end_scope();

        self.class_type = enclosing;
    }
}
