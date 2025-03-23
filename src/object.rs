mod callable;
mod callable_instance;
mod class;
mod class_instance;
mod definition;
mod function;
mod instance;
mod resolvable_function;

use std::rc::Rc;

pub use callable::Callable;
pub use callable_instance::CallableInstance;
pub use class::Class;
pub use function::Function;
pub use instance::Instance;
pub use resolvable_function::ResolvableFunction;

#[derive(Clone)]
pub enum Object {
    Undefined,
    Boolean(bool),
    Number(f64),
    String(String),
    Callable(Rc<dyn Callable>),
    Instance(Rc<dyn Instance>),
    CallableInstance(Rc<dyn CallableInstance>),
}
