pub mod ast_printer;
pub mod interpreter;

use crate::{
    expression::{
        assignment::Assignment, binary::Binary, comma::Comma, grouping::Grouping, literal::Literal,
        ternary::Ternary, unary::Unary, variable::Variable, Expression,
    },
    stmt::{
        block_stmt::BlockStmt, exit_stmt::ExitStmt, expression_stmt::ExpressionStmt,
        print_stmt::PrintStmt, variable_stmt::VariableStmt, Stmt,
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
}

impl<'a> Stmt<'a> {
    fn accept<V: StmtVisitor>(&mut self, visitor: &mut V) -> V::Item {
        match self {
            Stmt::ExpressionStmt(expression_stmt) => visitor.visit_expression_stmt(expression_stmt),
            Stmt::PrintStmt(print_stmt) => visitor.visit_print_stmt(print_stmt),
            Stmt::VariableStmt(variable_stmt) => visitor.visit_variable_stmt(variable_stmt),
            Stmt::BlockStmt(block_stmt) => visitor.visit_block_stmt(block_stmt),
            Stmt::ExitStmt(exit_stmt) => visitor.visit_exit_stmt(exit_stmt),
        }
    }
}
