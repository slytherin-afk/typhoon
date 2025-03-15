pub mod interpreter;

use crate::{
    expression::{
        assignment::Assignment, binary::Binary, call::Call, comma::Comma, grouping::Grouping,
        lambda::Lambda, literal::Literal, logical::Logical, ternary::Ternary, unary::Unary,
        variable::Variable, Expression,
    },
    stmt::{
        block_stmt::BlockStmt, expression_stmt::ExpressionStmt, function_stmt::FunctionStmt,
        if_stmt::IfStmt, print_stmt::PrintStmt, return_stmt::ReturnStmt,
        variable_stmt::VariableStmt, while_stmt::WhileStmt, Stmt,
    },
};

trait ExpressionVisitor {
    type Item;

    fn visit_comma(&mut self, expr: Comma) -> Self::Item;
    fn visit_ternary(&mut self, expr: Ternary) -> Self::Item;
    fn visit_binary(&mut self, expr: Binary) -> Self::Item;
    fn visit_unary(&mut self, expr: Unary) -> Self::Item;
    fn visit_grouping(&mut self, expr: Grouping) -> Self::Item;
    fn visit_literal(&mut self, expr: Literal) -> Self::Item;
    fn visit_variable(&mut self, expr: Variable) -> Self::Item;
    fn visit_assignment(&mut self, expr: Assignment) -> Self::Item;
    fn visit_logical(&mut self, expr: Logical) -> Self::Item;
    fn visit_call(&mut self, expr: Call) -> Self::Item;
    fn visit_lambda(&mut self, expr: Lambda) -> Self::Item;
}

impl Expression {
    fn accept<V: ExpressionVisitor>(self, visitor: &mut V) -> V::Item {
        match self {
            Expression::Comma(comma) => visitor.visit_comma(*comma),
            Expression::Ternary(ternary) => visitor.visit_ternary(*ternary),
            Expression::Binary(binary) => visitor.visit_binary(*binary),
            Expression::Unary(unary) => visitor.visit_unary(*unary),
            Expression::Grouping(grouping) => visitor.visit_grouping(*grouping),
            Expression::Literal(literal) => visitor.visit_literal(*literal),
            Expression::Variable(variable) => visitor.visit_variable(*variable),
            Expression::Assignment(assignment) => visitor.visit_assignment(*assignment),
            Expression::Logical(logical) => visitor.visit_logical(*logical),
            Expression::Call(call) => visitor.visit_call(*call),
            Expression::Lambda(lambda) => visitor.visit_lambda(*lambda),
        }
    }
}

trait StmtVisitor {
    type Item;

    fn visit_expression_stmt(&mut self, stmt: ExpressionStmt) -> Self::Item;
    fn visit_print_stmt(&mut self, stmt: PrintStmt) -> Self::Item;
    fn visit_variable_stmt(&mut self, stmt: VariableStmt) -> Self::Item;
    fn visit_block_stmt(&mut self, stmt: BlockStmt) -> Self::Item;
    fn visit_if_stmt(&mut self, stmt: IfStmt) -> Self::Item;
    fn visit_while_stmt(&mut self, stmt: WhileStmt) -> Self::Item;
    fn visit_function_stmt(&mut self, stmt: FunctionStmt) -> Self::Item;
    fn visit_return_stmt(&mut self, stmt: ReturnStmt) -> Self::Item;
    fn visit_empty_stmt(&mut self) -> Self::Item;
    fn visit_continue_stmt(&mut self) -> Self::Item;
    fn visit_break_stmt(&mut self) -> Self::Item;
}

impl Stmt {
    fn accept<V: StmtVisitor>(self, visitor: &mut V) -> V::Item {
        match self {
            Stmt::ExpressionStmt(expression_stmt) => {
                visitor.visit_expression_stmt(*expression_stmt)
            }
            Stmt::PrintStmt(print_stmt) => visitor.visit_print_stmt(*print_stmt),
            Stmt::VariableStmt(variable_stmt) => visitor.visit_variable_stmt(*variable_stmt),
            Stmt::BlockStmt(block_stmt) => visitor.visit_block_stmt(*block_stmt),
            Stmt::IfStmt(if_stmt) => visitor.visit_if_stmt(*if_stmt),
            Stmt::WhileStmt(while_stmt) => visitor.visit_while_stmt(*while_stmt),
            Stmt::FunctionStmt(function_stmt) => visitor.visit_function_stmt(*function_stmt),
            Stmt::ReturnStmt(return_stmt) => visitor.visit_return_stmt(*return_stmt),
            Stmt::EmptyStmt => visitor.visit_empty_stmt(),
            Stmt::ContinueStmt => visitor.visit_continue_stmt(),
            Stmt::BreakStmt => visitor.visit_break_stmt(),
        }
    }
}
