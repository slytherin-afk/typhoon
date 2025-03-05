use super::Expression;

pub struct Comma<'a> {
    pub left: Expression<'a>,
    pub right: Expression<'a>,
}
