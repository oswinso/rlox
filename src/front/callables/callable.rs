use crate::front::expr::Value;
use crate::front::interpreter::Interpreter;

use std::fmt;
use crate::front::errors::RuntimeError;

pub trait Callable: std::fmt::Display {
    fn name(&self) -> &str;
    fn arity(&self) -> usize;
    fn call(&self, interpreter: &mut Interpreter, arguments: Vec<Value>) -> Result<Value, Box<dyn RuntimeError>>;
}

impl PartialEq for dyn Callable {
    fn eq(&self, other: &dyn Callable) -> bool {
        self.name() == other.name()
    }
}

impl fmt::Debug for dyn Callable {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.name())
    }
}
