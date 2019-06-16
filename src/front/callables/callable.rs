use crate::front::interpreter::Interpreter;
use crate::front::expr::Value;

use std::fmt;

pub trait Callable : std::fmt::Display {
    fn name(&self) -> &str;
    fn arity(&self) -> usize;
    fn call(&self, interpreter: &Interpreter, arguments: Vec<Value>) -> Value;
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
