use crate::{errors::RuntimeError, token::Token};

use super::Object;

pub trait Instance {
    fn get(&self, this: Object, name: &Token) -> Result<Object, RuntimeError>;

    fn set(&self, name: &Token, value: Object) -> Result<(), RuntimeError>;

    fn to_string(&self) -> String;
}
