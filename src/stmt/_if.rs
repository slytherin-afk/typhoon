use crate::expr::Expr;

use super::Stmt;

#[derive(Clone)]
pub struct If {
    pub condition: Expr,
    pub truth: Stmt,
    pub falsy: Option<Stmt>,
}
