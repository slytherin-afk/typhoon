use std::any::Any;

use super::{callable::Callable, instance::Instance};

pub trait CallableInstance: Callable + Instance {
    fn as_any(&self) -> &dyn Any;
}
