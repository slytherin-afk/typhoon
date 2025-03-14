use crate::expression::Expression;

use super::Stmt;

#[derive(Clone)]
pub struct WhileStmt {
    pub condition: Expression,
    pub body: Stmt,
}
