use crate::expression::Expression;

pub struct PrintStmt<'a> {
    pub expression: Expression<'a>,
}
