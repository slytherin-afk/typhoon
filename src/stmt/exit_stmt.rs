use crate::expression::Expression;

pub struct ExitStmt<'a> {
    pub expression: Option<Expression<'a>>,
}
