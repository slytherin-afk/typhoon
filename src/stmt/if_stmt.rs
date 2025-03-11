use crate::expression::Expression;

use super::Stmt;

pub struct IfStmt<'a> {
    pub condition: Expression<'a>,
    pub truth: Stmt<'a>,
    pub falsy: Option<Stmt<'a>>,
}
