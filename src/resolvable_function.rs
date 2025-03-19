use crate::{scanner::token::Token, stmt::Stmt};

pub trait ResolvableFunction {
    fn params(&self) -> &Vec<Token>;
    fn body(&self) -> &Vec<Stmt>;
    fn name(&self) -> &str;
}
