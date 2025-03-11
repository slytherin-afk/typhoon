pub mod block_stmt;
pub mod exit_stmt;
pub mod expression_stmt;
pub mod if_stmt;
pub mod print_stmt;
pub mod variable_stmt;

use block_stmt::BlockStmt;
use exit_stmt::ExitStmt;
use expression_stmt::ExpressionStmt;
use if_stmt::IfStmt;
use print_stmt::PrintStmt;
use variable_stmt::VariableStmt;

pub enum Stmt<'a> {
    ExpressionStmt(Box<ExpressionStmt<'a>>),
    VariableStmt(Box<VariableStmt<'a>>),
    PrintStmt(Box<PrintStmt<'a>>),
    BlockStmt(Box<BlockStmt<'a>>),
    ExitStmt(Box<ExitStmt<'a>>),
    IfStmt(Box<IfStmt<'a>>),
}
