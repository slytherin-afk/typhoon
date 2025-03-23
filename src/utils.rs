use crate::object::Object;

pub fn bool_to_number(boolean: bool) -> f64 {
    if boolean {
        1.0
    } else {
        0.0
    }
}

pub fn is_truthy(literal: &Object) -> bool {
    match literal {
        Object::Undefined => false,
        Object::Number(number) => *number != 0.0,
        Object::String(string) => !string.is_empty(),
        Object::Boolean(boolean) => *boolean,
        _ => true,
    }
}
