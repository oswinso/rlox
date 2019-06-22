use crate::front::callables::Class;

use std::fmt;
use std::collections::HashMap;
use crate::front::expr::Value;
use crate::front::errors::{RuntimeError, UndefinedPropertyError};
use crate::front::token::Token;

#[derive(Clone)]
pub struct Instance {
    class: Class,
    fields: HashMap<String, Value>
}

impl Instance {
    pub fn new(class: Class) -> Instance {
        Instance { class, fields: HashMap::new() }
    }

    pub fn get(&self, name: &Token) -> Result<Value, Box<dyn RuntimeError>> {
        dbg!(&self.fields);
        if let Some(property) = self.fields.get(&name.lexeme) {
            Ok(property.clone())
        } else {
            Err(UndefinedPropertyError::new(format!("{}", self), name.clone()).into())
        }
    }

    pub fn set(&mut self, name: &Token, value: Value) {
        dbg!(&self.fields);
        self.fields.insert(name.lexeme.clone(), value);
        dbg!(&self.fields);
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
