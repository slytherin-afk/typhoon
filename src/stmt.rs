pub mod block_stmt;
pub mod expression_stmt;
pub mod function_stmt;
pub mod if_stmt;
pub mod print_stmt;
pub mod return_stmt;
pub mod variable_stmt;
pub mod while_stmt;

use block_stmt::BlockStmt;
use expression_stmt::ExpressionStmt;
use function_stmt::FunctionStmt;
use if_stmt::IfStmt;
use print_stmt::PrintStmt;
use return_stmt::ReturnStmt;
use variable_stmt::VariableStmt;
use while_stmt::WhileStmt;

use crate::scanner::token::Token;

#[derive(Clone)]
pub enum Stmt {
    ExpressionStmt(Box<ExpressionStmt>),
    VariableStmt(Box<VariableStmt>),
    PrintStmt(Box<PrintStmt>),
    BlockStmt(Box<BlockStmt>),
    IfStmt(Box<IfStmt>),
    WhileStmt(Box<WhileStmt>),
    FunctionStmt(Box<FunctionStmt>),
    ReturnStmt(Box<ReturnStmt>),
    ContinueStmt(Token),
    BreakStmt(Token),
    EmptyStmt,
}
