use std::collections::HashMap;

use crate::{
    expression::{
        Assignment, Binary, Call, Comma, Expression, ExpressionVisitor, Grouping, Lambda, Literal,
        Logical, Ternary, Unary, Variable,
    },
    stmt::{
        BlockStmt, ExpressionStmt, FunctionStmt, IfStmt, PrintStmt, ReturnStmt, Stmt, StmtVisitor,
        VariableStmt, WhileStmt,
    },
    Interpreter, Lib, ResolvableFunction, Token,
};

pub struct Resolver<'a> {
    interpreter: &'a mut Interpreter,
    scopes: Vec<HashMap<String, bool>>,
    unused_variables: Vec<HashMap<String, Token>>,
    loop_depth: usize,
    function_depth: usize,
}

impl<'a> Resolver<'a> {
    pub fn new(interpreter: &'a mut Interpreter) -> Self {
        Self {
            interpreter,
            scopes: vec![],
            unused_variables: vec![],
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

    fn resolve_function<T: ResolvableFunction>(&mut self, function: &T) {
        self.function_depth += 1;
        self.begin_scope();

        for param in function.params() {
            self.declare(param);
            self.define(param);
        }

        self.resolve_stmts(function.body());
        self.end_scope();
        self.function_depth -= 1;
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
        self.resolve_function(expr);
    }

    fn visit_assignment(&mut self, expr: &Assignment) -> Self::Item {
        self.resolve_expression(&expr.expression);
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

    fn visit_grouping(&mut self, expr: &Grouping) -> Self::Item {
        self.resolve_expression(&expr.expression);
    }

    fn visit_variable(&mut self, expr: &Variable) -> Self::Item {
        if !self.scopes.is_empty() {
            if let Some(&false) = self.scopes.last().unwrap().get(&expr.name.lexeme) {
                Lib::error_two(
                    &expr.name,
                    "Can't read local variable in its own initializer.",
                );
            }
        }

        self.resolve_local(&expr.name);
    }

    fn visit_literal(&mut self, _: &Literal) -> Self::Item {}
}

impl<'a> StmtVisitor for Resolver<'a> {
    type Item = ();

    fn visit_empty_stmt(&mut self) -> Self::Item {}

    fn visit_expression_stmt(&mut self, stmt: &ExpressionStmt) -> Self::Item {
        self.resolve_expression(&stmt.value);
    }

    fn visit_print_stmt(&mut self, stmt: &PrintStmt) -> Self::Item {
        self.resolve_expression(&stmt.value);
    }

    fn visit_variable_stmt(&mut self, stmt: &VariableStmt) -> Self::Item {
        for variable in &stmt.stmts {
            self.declare(&variable.name);

            if let Some(initializer) = &variable.initializer {
                self.resolve_expression(initializer);
            }

            self.define(&variable.name);
        }
    }

    fn visit_block_stmt(&mut self, stmt: &BlockStmt) -> Self::Item {
        self.begin_scope();
        self.resolve_stmts(&stmt.stmts);
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
            Lib::error_two(keyword, "Can't use break outside a loop");
        } else if self.function_depth >= self.loop_depth {
            Lib::error_two(keyword, "Jump target cannot cross function boundary");
        }
    }

    fn visit_continue_stmt(&mut self, keyword: &Token) -> Self::Item {
        if self.loop_depth == 0 {
            Lib::error_two(keyword, "Can't use continue outside a loop");
        } else if self.function_depth >= self.loop_depth {
            Lib::error_two(keyword, "Jump target cannot cross function boundary");
        }
    }

    fn visit_function_stmt(&mut self, stmt: &FunctionStmt) -> Self::Item {
        self.declare(&stmt.name);
        self.define(&stmt.name);
        self.resolve_function(stmt);
    }

    fn visit_return_stmt(&mut self, stmt: &ReturnStmt) -> Self::Item {
        if self.function_depth == 0 {
            Lib::error_two(&stmt.keyword, "Can't use return outside a function");
        }

        if let Some(value) = &stmt.value {
            self.resolve_expression(value);
        }
    }
}
