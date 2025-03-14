use crate::expression::Expression;

#[derive(Clone)]
pub struct ExitStmt {
    pub expression: Option<Expression>,
}
