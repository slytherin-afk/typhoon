use super::Expression;

#[derive(Clone)]
pub struct Comma {
    pub left: Expression,
    pub right: Expression,
}
