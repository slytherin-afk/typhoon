use std::{cell::RefCell, collections::HashMap, fmt, rc::Rc};

use crate::{
    errors::{RuntimeError, RuntimeException},
    operations::bool_to_number,
    stmt::Stmt,
    Environment, Interpreter, Token,
};

#[derive(Clone)]
pub enum Object {
    Undefined,
    Number(f64),
    String(String),
    Boolean(bool),
    Callable(Rc<dyn Callable>),
    ClassInstance(Rc<RefCell<ClassInstance>>),
}

impl fmt::Display for Object {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Object::Undefined => write!(f, "{}", "undefined"),
            Object::Number(n) => write!(f, "{}", n),
            Object::String(s) => write!(f, "{}", s),
            Object::Boolean(b) => write!(f, "{}", b),
            Object::Callable(callee) => write!(f, "{}", callee.to_string()),
            Object::ClassInstance(class_instance) => {
                write!(f, "{}", class_instance.borrow().to_string())
            }
        }
    }
}

impl PartialEq for Object {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Object::Undefined, Object::Undefined) => true,
            (Object::Number(a), Object::Number(b)) => a == b,
            (Object::Number(a), Object::Boolean(b)) => *a == bool_to_number(*b),
            (Object::Boolean(a), Object::Number(b)) => bool_to_number(*a) == *b,
            (Object::String(a), Object::String(b)) => a == b,
            (Object::Boolean(a), Object::Boolean(b)) => a == b,
            (Object::Callable(a), Object::Callable(b)) => Rc::ptr_eq(a, b),
            (Object::ClassInstance(a), Object::ClassInstance(b)) => Rc::ptr_eq(a, b),
            _ => false,
        }
    }
}

pub trait Callable {
    fn arity(&self) -> usize;
    fn call(
        &self,
        interpreter: &mut Interpreter,
        arguments: Vec<Object>,
    ) -> Result<Object, RuntimeError>;
    fn to_string(&self) -> String {
        "[Native Function]".to_string()
    }
    fn bind(&self, _: Object) -> Object {
        unreachable!()
    }
}

pub trait ResolvableFunction: 'static {
    fn params(&self) -> &Vec<Token>;
    fn body(&self) -> &Vec<Stmt>;
    fn name(&self) -> &str;
}

pub struct Function<T: ResolvableFunction + Clone + 'static> {
    declaration: T,
    closure: Rc<RefCell<Environment>>,
    is_initializer: bool,
}

impl<T: ResolvableFunction + Clone> Function<T> {
    pub fn new(declaration: T, closure: Rc<RefCell<Environment>>, is_initializer: bool) -> Self {
        Self {
            declaration,
            closure,
            is_initializer,
        }
    }
}

impl<T: ResolvableFunction + Clone> Callable for Function<T> {
    fn arity(&self) -> usize {
        self.declaration.params().len()
    }

    fn call(
        &self,
        interpreter: &mut Interpreter,
        arguments: Vec<Object>,
    ) -> Result<Object, RuntimeError> {
        let mut env = Environment::new(Some(Rc::clone(&self.closure)));

        for (param, arg) in self.declaration.params().iter().zip(arguments) {
            env.define(&param.lexeme, arg);
        }

        if let Err(err) = interpreter.execute_block(&self.declaration.body(), env) {
            return match err {
                RuntimeException::RuntimeError(runtime_error) => Err(runtime_error),
                RuntimeException::ReturnException(object) => {
                    if self.is_initializer {
                        return self.closure.borrow().get_at(0, "this");
                    }

                    Ok(object)
                }
                _ => unreachable!(),
            };
        }

        Ok(Object::Undefined)
    }

    fn to_string(&self) -> String {
        format!("[Function: ({})]", self.declaration.name())
    }

    fn bind(&self, instance: Object) -> Object {
        let mut env = Environment::new(Some(Rc::clone(&self.closure)));

        env.define("this", instance);

        Object::Callable(Rc::new(Function::new(
            self.declaration.clone(),
            Rc::new(RefCell::new(env)),
            self.is_initializer,
        )))
    }
}

pub struct ClassInternal {
    name: String,
    methods: HashMap<String, Object>,
}

#[derive(Clone)]
pub struct Class {
    internal: Rc<ClassInternal>,
}

impl Class {
    pub fn new(name: &str, methods: HashMap<String, Object>) -> Self {
        Self {
            internal: Rc::new(ClassInternal {
                name: String::from(name),
                methods,
            }),
        }
    }

    fn find_method(&self, name: &str) -> Option<Object> {
        Some(self.internal.methods.get(name)?.clone())
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
        let instance = Object::ClassInstance(Rc::new(RefCell::new(class_instance)));

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
}

pub struct ClassInstance {
    class: Class,
    fields: HashMap<String, Object>,
}

impl ClassInstance {
    fn new(class: Class) -> Self {
        Self {
            class,
            fields: HashMap::new(),
        }
    }

    pub fn get(&self, this: Object, name: &Token) -> Result<Object, RuntimeError> {
        if let Some(field) = self.fields.get(&name.lexeme) {
            return Ok(field.clone());
        }

        if let Some(method) = self.class.find_method(&name.lexeme) {
            return match &method {
                Object::Callable(callable) => Ok(callable.bind(this)),
                _ => unreachable!(),
            };
        }

        Err(RuntimeError::new(
            name.clone(),
            format!("Undefined property '{}'", name.lexeme),
        ))
    }

    pub fn set(&mut self, name: &Token, value: Object) -> Result<(), RuntimeError> {
        self.fields.insert(String::clone(&name.lexeme), value);

        Ok(())
    }

    fn to_string(&self) -> String {
        format!("[Class Instance: ({})]", self.class.internal.name)
    }
}
