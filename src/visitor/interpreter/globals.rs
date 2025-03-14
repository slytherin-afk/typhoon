use super::{callable::Callable, Interpreter, RuntimeError};
use crate::object::Object;
use std::time::{SystemTime, UNIX_EPOCH};

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
}
