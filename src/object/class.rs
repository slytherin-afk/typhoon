use std::{any::Any, cell::RefCell, collections::HashMap, rc::Rc};

use super::{
    callable_instance::CallableInstance, class_instance::ClassInstance, Callable, Instance, Object,
};

use crate::{errors::RuntimeError, interpreter::Interpreter, token::Token};

pub struct ClassInternal {
    pub name: String,
    super_class: Option<Rc<dyn CallableInstance>>,
    methods: HashMap<String, Object>,
    statics: RefCell<HashMap<String, Object>>,
}

#[derive(Clone)]
pub struct Class {
    pub internal: Rc<ClassInternal>,
}

impl Class {
    pub fn new(
        name: &str,
        super_class: Option<Rc<dyn CallableInstance>>,
        statics: HashMap<String, Object>,
        methods: HashMap<String, Object>,
    ) -> Self {
        Self {
            internal: Rc::new(ClassInternal {
                name: String::from(name),
                super_class,
                methods,
                statics: RefCell::new(statics),
            }),
        }
    }

    pub fn find_method(&self, name: &str) -> Option<Object> {
        if let Some(method) = self.internal.methods.get(name) {
            return Some(method.clone());
        }

        if let Some(super_class) = &self.internal.super_class {
            if let Some(class) = super_class.as_any().downcast_ref::<Class>() {
                return class.find_method(name);
            }
        }

        None
    }
}

impl Callable for Class {
    fn arity(&self) -> usize {
        if let Some(Object::Callable(callable)) = self.find_method("init") {
            callable.arity()
        } else {
            0
        }
    }

    fn call(
        &self,
        interpreter: &mut Interpreter,
        arguments: Vec<Object>,
    ) -> Result<Object, RuntimeError> {
        let class_instance = ClassInstance::new(self.clone());
        let instance = Object::Instance(Rc::new(class_instance));

        if let Some(Object::Callable(callable)) = self.find_method("init") {
            let bound_callable = callable.bind(instance.clone());

            if let Object::Callable(bound_callable) = bound_callable {
                bound_callable.call(interpreter, arguments)?;
            }
        }

        Ok(instance)
    }

    fn to_string(&self) -> String {
        format!("[Class: ({})]", self.internal.name)
    }

    fn bind(&self, _: Object) -> Object {
        unreachable!()
    }
}

impl Instance for Class {
    fn get(&self, _: Object, name: &Token) -> Result<Object, RuntimeError> {
        if let Some(field) = self.internal.statics.borrow().get(&name.lexeme) {
            return Ok(field.clone());
        }

        Err(RuntimeError {
            token: name.clone(),
            message: format!("Undefined property '{}'", name.lexeme),
        })
    }

    fn set(&self, name: &Token, value: Object) -> Result<(), RuntimeError> {
        self.internal
            .statics
            .borrow_mut()
            .insert(String::clone(&name.lexeme), value);

        Ok(())
    }

    fn to_string(&self) -> String {
        format!("[Class Instance: ({})]", self.internal.name)
    }
}

impl CallableInstance for Class {
    fn as_any(&self) -> &dyn Any {
        self
    }
}
