use crate::expression::Expression;

use super::Stmt;

#[derive(Clone)]
pub struct IfStmt {
    pub condition: Expression,
    pub truth: Stmt,
    pub falsy: Option<Stmt>,
}
