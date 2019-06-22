use crate::front::callables::Class;

use crate::front::errors::{RuntimeError, UndefinedPropertyError};
use crate::front::expr::Value;
use crate::front::token::Token;
use std::collections::HashMap;
use std::fmt;
use std::cell::RefCell;

#[derive(Clone)]
pub struct Instance {
    class: Class,
    fields: RefCell<HashMap<String, Value>>,
}

impl Instance {
    pub fn new(class: Class) -> Instance {
        Instance {
            class,
            fields: RefCell::new(HashMap::new()),
        }
    }

    pub fn get(&self, name: &Token) -> Result<Value, Box<dyn RuntimeError>> {
        if let Some(property) = self.fields.borrow().get(&name.lexeme) {
            Ok(property.clone())
        } else {
            Err(UndefinedPropertyError::new(format!("{}", self), name.clone()).into())
        }
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
        write!(f, "{} instance", self.class)
    }
}

impl PartialEq for Instance {
    fn eq(&self, other: &Instance) -> bool {
        false
    }
}
