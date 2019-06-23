use crate::front::callables::{Callable, Function, Instance};
use crate::front::errors::RuntimeError;
use crate::front::expr::Value;
use crate::front::interpreter::Interpreter;
use std::collections::HashMap;
use std::fmt;
use std::rc::Rc;

#[derive(Clone)]
pub struct Class {
    name: String,
    methods: HashMap<String, Function>,
}

impl Class {
    pub fn new(name: String, methods: HashMap<String, Function>) -> Class {
        Class { name, methods }
    }

    pub fn find_method(&self, name: &str) -> Option<Function> {
        Some(self.methods.get(name)?.clone())
    }
}

impl Callable for Class {
    fn name(&self) -> &str {
        &self.name
    }

    fn arity(&self) -> usize {
        0
    }

    fn call(
        &self,
        _interpreter: &mut Interpreter,
        _arguments: Vec<Rc<Value>>,
    ) -> Result<Rc<Value>, Box<dyn RuntimeError>> {
        Ok(Rc::new(Value::Instance(Instance::new(self.clone()))))
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
