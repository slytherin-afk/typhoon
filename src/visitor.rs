pub mod interpreter;

use crate::{
    expression::{
        assignment::Assignment, binary::Binary, comma::Comma, grouping::Grouping, literal::Literal,
        logical::Logical, ternary::Ternary, unary::Unary, variable::Variable, Expression,
    },
    stmt::{
        block_stmt::BlockStmt, empty_stmt::EmptyStmt, exit_stmt::ExitStmt,
        expression_stmt::ExpressionStmt, if_stmt::IfStmt, print_stmt::PrintStmt,
        variable_stmt::VariableStmt, while_stmt::WhileStmt, Stmt,
    },
};

trait ExpressionVisitor {
    type Item;

    fn visit_comma(&mut self, expr: &mut Comma) -> Self::Item;
    fn visit_ternary(&mut self, expr: &mut Ternary) -> Self::Item;
    fn visit_binary(&mut self, expr: &mut Binary) -> Self::Item;
    fn visit_unary(&mut self, expr: &mut Unary) -> Self::Item;
    fn visit_grouping(&mut self, expr: &mut Grouping) -> Self::Item;
    fn visit_literal(&mut self, expr: &mut Literal) -> Self::Item;
    fn visit_variable(&mut self, expr: &mut Variable) -> Self::Item;
    fn visit_assignment(&mut self, expr: &mut Assignment) -> Self::Item;
    fn visit_logical(&mut self, expr: &mut Logical) -> Self::Item;
}

impl<'a> Expression<'a> {
    fn accept<V: ExpressionVisitor>(&mut self, visitor: &mut V) -> V::Item {
        match self {
            Expression::Comma(comma) => visitor.visit_comma(comma),
            Expression::Ternary(ternary) => visitor.visit_ternary(ternary),
            Expression::Binary(binary) => visitor.visit_binary(binary),
            Expression::Unary(unary) => visitor.visit_unary(unary),
            Expression::Grouping(grouping) => visitor.visit_grouping(grouping),
            Expression::Literal(literal) => visitor.visit_literal(literal),
            Expression::Variable(variable) => visitor.visit_variable(variable),
            Expression::Assignment(assignment) => visitor.visit_assignment(assignment),
            Expression::Logical(logical) => visitor.visit_logical(logical),
        }
    }
}

trait StmtVisitor {
    type Item;

    fn visit_expression_stmt(&mut self, stmt: &mut ExpressionStmt) -> Self::Item;
    fn visit_print_stmt(&mut self, stmt: &mut PrintStmt) -> Self::Item;
    fn visit_variable_stmt(&mut self, stmt: &mut VariableStmt) -> Self::Item;
    fn visit_block_stmt(&mut self, stmt: &mut BlockStmt) -> Self::Item;
    fn visit_exit_stmt(&mut self, stmt: &mut ExitStmt) -> Self::Item;
    fn visit_if_stmt(&mut self, stmt: &mut IfStmt) -> Self::Item;
    fn visit_empty_stmt(&mut self, stmt: &mut EmptyStmt) -> Self::Item;
    fn visit_while_stmt(&mut self, stmt: &mut WhileStmt) -> Self::Item;
}

impl<'a> Stmt<'a> {
    fn accept<V: StmtVisitor>(&mut self, visitor: &mut V) -> V::Item {
        match self {
            Stmt::ExpressionStmt(expression_stmt) => visitor.visit_expression_stmt(expression_stmt),
            Stmt::PrintStmt(print_stmt) => visitor.visit_print_stmt(print_stmt),
            Stmt::VariableStmt(variable_stmt) => visitor.visit_variable_stmt(variable_stmt),
            Stmt::BlockStmt(block_stmt) => visitor.visit_block_stmt(block_stmt),
            Stmt::ExitStmt(exit_stmt) => visitor.visit_exit_stmt(exit_stmt),
            Stmt::IfStmt(if_stmt) => visitor.visit_if_stmt(if_stmt),
            Stmt::WhileStmt(while_stmt) => visitor.visit_while_stmt(while_stmt),
            Stmt::EmptyStmt(empty_stmt) => visitor.visit_empty_stmt(empty_stmt),
        }
    }
}
