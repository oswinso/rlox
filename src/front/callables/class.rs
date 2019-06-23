use crate::front::callables::{Callable, Function, Instance};
use crate::front::errors::RuntimeError;
use crate::front::expr::Value;
use crate::front::interpreter::Interpreter;
use std::collections::HashMap;
use std::fmt;
use std::rc::Rc;
use core::borrow::Borrow;

#[derive(Clone)]
pub struct Class {
    name: String,
    methods: HashMap<String, Function>,
    superclass: Option<Box<Class>>
}

impl Class {
    pub fn new(name: String, superclass: Option<Box<Class>>, methods: HashMap<String, Function>) -> Class {
        Class { name, methods, superclass }
    }

    pub fn find_method(&self, name: &str) -> Option<Function> {
        if let Some(method) = self.methods.get(name) {
            Some(method.clone())
        } else if let Some(superclass) = &self.superclass {
            superclass.find_method(name)
        } else {
            None
        }
    }
}

impl Callable for Class {
    fn name(&self) -> &str {
        &self.name
    }

    fn arity(&self) -> usize {
        if let Some(initializer) = self.find_method(&self.name) {
            initializer.arity()
        } else {
            0
        }
    }

    fn call(
        &self,
        interpreter: &mut Interpreter,
        arguments: Vec<Rc<Value>>,
    ) -> Result<Rc<Value>, Box<dyn RuntimeError>> {
        let instance = Rc::new(Value::Instance(Rc::new(Instance::new(self.clone()))));
        if let Some(initializer) = self.find_method(&self.name) {
            initializer.bind(instance.clone()).call(interpreter, arguments)?;
        }
        Ok(instance)
    }
}

impl fmt::Display for Class {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} class", self.name)
    }
}

impl fmt::Debug for Class {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} class", self.name)
    }
}

impl PartialEq for Class {
    fn eq(&self, other: &Class) -> bool {
        self.name == other.name
    }
}
