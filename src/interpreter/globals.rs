use std::time::{SystemTime, UNIX_EPOCH};

use crate::{
    errors::RuntimeError,
    object::{Callable, Object},
};

use super::Interpreter;

pub struct Clock;

impl Callable for Clock {
    fn arity(&self) -> usize {
        0
    }

    fn call(&self, _: &mut Interpreter, _: Vec<Object>) -> Result<Object, RuntimeError> {
        let now = SystemTime::now();
        let millis = now
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_millis() as f64;

        Ok(Object::Number(millis))
    }

    fn to_string(&self) -> String {
        String::from("Native Function: (clock)")
    }

    fn bind(&self, _: Object) -> Object {
        unreachable!()
    }
}
