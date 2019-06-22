use crate::front::expr::Value;
use crate::front::interpreter::Interpreter;

use crate::front::errors::RuntimeError;
use std::fmt;
use std::rc::Rc;

pub trait Callable: std::fmt::Display {
    fn name(&self) -> &str;
    fn arity(&self) -> usize;
    fn call(
        &self,
        interpreter: &mut Interpreter,
        arguments: Vec<Rc<Value>>,
    ) -> Result<Rc<Value>, Box<dyn RuntimeError>>;
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
