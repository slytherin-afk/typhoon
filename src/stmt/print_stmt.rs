use crate::expression::Expression;

#[derive(Clone)]
pub struct PrintStmt {
    pub expression: Expression,
}
