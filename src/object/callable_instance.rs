use super::{callable::Callable, instance::Instance};

pub trait CallableInstance: Callable + Instance {}
