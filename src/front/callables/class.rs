use crate::front::callables::{Callable, Instance};
use crate::front::errors::RuntimeError;
use crate::front::expr::Value;
use crate::front::interpreter::Interpreter;
use std::fmt;

#[derive(Clone, PartialEq)]
pub struct Class {
    name: String,
}

impl Class {
    pub fn new(name: String) -> Class {
        Class { name }
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
        _arguments: Vec<Value>,
    ) -> Result<Value, Box<dyn RuntimeError>> {
        Ok(Value::Instance(Instance::new(self.clone())))
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
