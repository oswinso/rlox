use crate::front::callables::Class;

use crate::front::errors::{RuntimeError, UndefinedPropertyError};
use crate::front::expr::Value;
use crate::front::token::Token;
use std::cell::RefCell;
use std::collections::HashMap;
use std::fmt;
use std::rc::Rc;

#[derive(Clone)]
pub struct Instance {
    class: Class,
    pub fields: RefCell<HashMap<String, Value>>,
}

impl Instance {
    pub fn new(class: Class) -> Instance {
        Instance {
            class,
            fields: RefCell::new(HashMap::new()),
        }
    }

    pub fn get(&self, name: &Token, rc_to_self: Rc<Instance>) -> Result<Value, Box<dyn RuntimeError>> {
        if let Some(property) = self.fields.borrow().get(&name.lexeme) {
            return Ok(property.clone());
        }

        if let Some(method) = self.class.find_method(&name.lexeme) {
            return Ok(Value::Callable(Rc::new(Box::new(method.bind(Rc::new(Value::Instance(rc_to_self)))))));
        }

        Err(UndefinedPropertyError::new(format!("{}", self), name.clone()).into())
    }

    pub fn set(&self, name: &Token, value: Value) {
        self.fields.borrow_mut().insert(name.lexeme.clone(), value);
    }
}

impl fmt::Display for Instance {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} instance", self.class)
    }
}

impl fmt::Debug for Instance {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} instance. Fields: {:?}", self.class, self.fields)
    }
}

impl PartialEq for Instance {
    fn eq(&self, other: &Instance) -> bool {
        false
    }
}
