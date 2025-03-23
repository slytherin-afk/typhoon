use crate::expr::Expr;

use super::Stmt;

#[derive(Clone)]
pub struct While {
    pub condition: Expr,
    pub body: Stmt,
}
