use std::collections::HashMap;

use crate::front::errors::{UndefinedVariableError, RuntimeError};
use crate::front::expr::Value;
use crate::front::token::Token;
use crate::front::token_type::TokenType;

pub struct Environment {
    values: HashMap<String, Value>,
}

impl Environment {
    pub fn new() -> Environment {
        Environment {
            values: HashMap::new(),
        }
    }

    pub fn define(&mut self, name: String, value: Value) {
        self.values.insert(name, value);
    }

    pub fn get(&self, token: &Token) -> Result<Value, Box<RuntimeError>> {
        if let TokenType::Identifier(name) = &token.token_type {
            self.values
                .get(name).cloned()
                .ok_or(UndefinedVariableError::new(token.clone()).into())
        } else {
            panic!("Non Identifier token used to get key from environment!")
        }
    }
}
