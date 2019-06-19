use crate::front::callables::Callable;
use crate::front::expr::Value;
use crate::front::interpreter::Interpreter;

use crate::front::errors::RuntimeError;
use std::fmt;
use std::rc::Rc;
use std::time::{SystemTime, UNIX_EPOCH};

pub struct Clock {
    name: String,
    arity: usize,
}

impl Callable for Clock {
    fn name(&self) -> &str {
        &self.name
    }

    fn arity(&self) -> usize {
        self.arity
    }

    fn call(
        &self,
        interpreter: &mut Interpreter,
        arguments: Vec<Value>,
    ) -> Result<Value, Box<RuntimeError>> {
        let now = SystemTime::now();
        let since_epoch = now.duration_since(UNIX_EPOCH).unwrap();
        Ok(Value::Literal(since_epoch.as_secs_f64().into()))
    }
}

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        unimplemented!()
    }
}

impl Clock {
    pub fn new() -> Rc<Box<dyn Callable>> {
        Rc::new(Box::new(Clock {
            name: "clock".to_string(),
            arity: 0,
        }))
    }
}
