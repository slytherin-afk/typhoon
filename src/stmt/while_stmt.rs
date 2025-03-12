use crate::expression::Expression;

use super::Stmt;

pub struct WhileStmt<'a> {
    pub condition: Expression<'a>,
    pub body: Stmt<'a>,
}
