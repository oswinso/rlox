use std::collections::HashMap;

use crate::front::errors::{RuntimeError, UndefinedVariableError};
use crate::front::expr::{Literal, Value};
use crate::front::token::Token;
use crate::front::token_type::TokenType;

pub struct Environment {
    head: Option<Box<ScopedEnvironment>>,
}

pub struct Variable {
    pub defined: bool,
    pub value: Value,
}

impl Variable {
    pub fn new() -> Self {
        Variable {
            defined: false,
            value: Value::Literal(Literal::Nil),
        }
    }

    pub fn initialize(value: Value) -> Self {
        Variable {
            defined: true,
            value,
        }
    }

    pub fn assign(&mut self, value: Value) {
        self.defined = true;
        self.value = value;
    }
}

pub struct ScopedEnvironment {
    values: HashMap<String, Variable>,
    parent: Option<Box<ScopedEnvironment>>,
}

impl Environment {
    pub fn new() -> Environment {
        let mut env = Environment { head: None };
        env.push();
        env
    }

    pub fn push(&mut self) {
        let scoped_environment = Box::new(ScopedEnvironment::new(self.head.take()));
        self.head = Some(scoped_environment);
    }

    pub fn pop(&mut self) {
        self.head.take().map(|env| {
            self.head = env.parent;
        });
    }

    pub fn define(&mut self, name: String, value: Option<Value>) {
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
    pub fn new(parent: Option<Box<ScopedEnvironment>>) -> ScopedEnvironment {
        ScopedEnvironment {
            values: HashMap::new(),
            parent,
        }
    }

    pub fn define(&mut self, name: String, value: Option<Value>) {
        match value {
            Some(value) => self.values.insert(name, Variable::initialize(value)),
            None => self.values.insert(name, Variable::new()),
        };
    }

    pub fn assign(&mut self, token: &Token, value: Value) -> Option<Box<RuntimeError>> {
        if self.values.contains_key(&token.lexeme) {
            self.values.get_mut(&token.lexeme).unwrap().assign(value);
            None
        } else if let Some(parent) = self.parent.as_mut() {
            parent.assign(token, value)
        } else {
            Some(UndefinedVariableError::new(token.clone()).into())
        }
    }

    pub fn get(&self, token: &Token) -> Result<Value, Box<RuntimeError>> {
        if let TokenType::Identifier(name) = &token.token_type {
            if let Some(variable) = self.values.get(name) {
                if variable.defined {
                    Ok(variable.value.clone())
                } else {
                    Err(UndefinedVariableError::new(token.clone()).into())
                }
            } else if let Some(parent) = self.parent.as_ref() {
                parent.get(token)
            } else {
                Err(UndefinedVariableError::new(token.clone()).into())
            }
        } else {
            panic!("Non Identifier token used to get key from environment!")
        }
    }
}
