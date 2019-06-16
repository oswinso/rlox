use std::collections::HashMap;

use crate::front::errors::{RuntimeError, UndefinedVariableError};
use crate::front::expr::Value;
use crate::front::token::Token;
use crate::front::token_type::TokenType;

pub struct Environment {
    head: Option<Box<ScopedEnvironment>>,
}

pub struct ScopedEnvironment {
    values: HashMap<String, Value>,
    parent: Option<Box<ScopedEnvironment>>,
}

impl Environment {
    pub fn new() -> Environment {
        let mut env = Environment { head: None };
        env.push();
        env
    }

    pub fn push(&mut self) {
        let scoped_environment = Box::new(ScopedEnvironment {
            values: HashMap::new(),
            parent: self.head.take(),
        });
        self.head = Some(scoped_environment);
    }

    pub fn pop(&mut self) {
        self.head.take().map(|env| {
            self.head = env.parent;
        });
    }

    pub fn define(&mut self, name: String, value: Value) {
        self.head.as_mut().map(|env| env.define(name, value));
    }

    pub fn assign(&mut self, token: &Token, value: Value) -> Option<Box<RuntimeError>> {
        self.head.as_mut().unwrap().assign(token, value)
    }

    pub fn get(&self, token: &Token) -> Result<Value, Box<RuntimeError>> {
        self.head.as_ref().unwrap().get(token)
    }
}

impl ScopedEnvironment {
    pub fn new() -> ScopedEnvironment {
        ScopedEnvironment {
            values: HashMap::new(),
            parent: None,
        }
    }

    pub fn new_with_parent(parent: Box<ScopedEnvironment>) -> ScopedEnvironment {
        ScopedEnvironment {
            values: HashMap::new(),
            parent: Some(parent),
        }
    }

    pub fn define(&mut self, name: String, value: Value) {
        self.values.insert(name, value);
    }

    pub fn assign(&mut self, token: &Token, value: Value) -> Option<Box<RuntimeError>> {
        if self.values.contains_key(&token.lexeme) {
            *self.values.get_mut(&token.lexeme).unwrap() = value;
            None
        } else if let Some(parent) = self.parent.as_mut() {
            parent.assign(token, value)
        } else {
            Some(UndefinedVariableError::new(token.clone()).into())
        }
    }

    pub fn get(&self, token: &Token) -> Result<Value, Box<RuntimeError>> {
        if let TokenType::Identifier(name) = &token.token_type {
            self.values
                .get(name)
                .cloned()
                .ok_or(UndefinedVariableError::new(token.clone()).into())
        } else if let Some(parent) = self.parent.as_ref() {
            parent.get(token)
        } else {
            panic!("Non Identifier token used to get key from environment!")
        }
    }
}
