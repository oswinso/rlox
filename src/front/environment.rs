use std::collections::HashMap;

use crate::front::errors::{FatalError, RuntimeError, UndefinedVariableError};
use crate::front::expr::{Literal, Value};
use crate::front::token::Token;
use crate::front::token_type::TokenType;
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Clone)]
pub struct Environment {
    head: Link,
}

type Link = Option<Rc<RefCell<ScopedEnvironment>>>;

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
    parent: Link,
}

impl Environment {
    pub fn new() -> Environment {
        let mut env = Environment { head: None };
        env.push();
        env
    }

    pub fn empty_env() -> Environment {
        let env = Environment { head: None };
        env
    }

    pub fn is_global(&self) -> bool {
        self.head.as_ref().unwrap().borrow().parent.is_none()
    }

    /// Swaps the head with other
    pub fn swap(&mut self, other: &mut Environment) {
        std::mem::swap(&mut self.head, &mut other.head);
    }

    pub fn push(&mut self) {
        let new_node = Rc::new(RefCell::new(ScopedEnvironment::new(None)));
        match self.head.take() {
            Some(old_head) => {
                new_node.borrow_mut().parent = Some(old_head);
                self.head = Some(new_node)
            }
            None => self.head = Some(new_node),
        }
    }

    pub fn pop(&mut self) {
        self.head
            .take()
            .map(|old_head| match old_head.borrow_mut().parent.take() {
                Some(new_head) => self.head = Some(new_head),
                None => panic!("Tried to remove global env"),
            });
    }

    pub fn define(&mut self, name: String, value: Option<Value>) {
        self.head
            .as_mut()
            .map(|env| env.borrow_mut().define(name, value));
    }

    pub fn assign(&mut self, token: &Token, value: Value) -> Option<Box<dyn RuntimeError>> {
        self.head
            .as_mut()
            .unwrap()
            .borrow_mut()
            .assign(token, value)
    }

    pub fn get(&self, token: &Token) -> Result<Value, Box<dyn RuntimeError>> {
        self.head.as_ref().unwrap().borrow().get(token)
    }

    pub fn get_at(&self, token: &Token, depth: usize) -> Result<Value, Box<dyn RuntimeError>> {
        self.ancestor(depth).map_or(
            Err(FatalError::new(
                token.clone(),
                "Variable tried to resolve to environment that is deeper than Adele".into(),
            )
            .into()),
            |scoped_env| scoped_env.borrow().get(token),
        )
    }

    pub fn assign_at(&self, token: &Token, value: Value, depth: usize) -> Option<Box<dyn RuntimeError>> {
        self.ancestor(depth).map_or(
            Some(FatalError::new(
                token.clone(),
                "Variable tried to resolve to environment that is deeper than Adele".into(),
            )
                .into()),
            |scoped_env| scoped_env.borrow_mut().assign(token, value),
        )
    }

    pub fn ancestor(&self, distance: usize) -> Link {
        let mut scoped_env  = self.head.as_ref()?.clone();
        for _ in 0..distance {
            let new_env = scoped_env.borrow().parent.clone()?;
            scoped_env = new_env;
        }
        Some(scoped_env)
    }
}

impl ScopedEnvironment {
    pub fn new(parent: Link) -> ScopedEnvironment {
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

    pub fn assign(&mut self, token: &Token, value: Value) -> Option<Box<dyn RuntimeError>> {
        if self.values.contains_key(&token.lexeme) {
            self.values.get_mut(&token.lexeme).unwrap().assign(value);
            None
        } else if let Some(parent) = self.parent.as_mut() {
            parent.borrow_mut().assign(token, value)
        } else {
            Some(UndefinedVariableError::new(token.clone()).into())
        }
    }

    pub fn get(&self, token: &Token) -> Result<Value, Box<dyn RuntimeError>> {
        if let TokenType::Identifier(name) = &token.token_type {
            if let Some(variable) = self.values.get(name) {
                if variable.defined {
                    Ok(variable.value.clone())
                } else {
                    Err(UndefinedVariableError::new(token.clone()).into())
                }
            } else if let Some(parent) = self.parent.as_ref() {
                parent.borrow().get(token)
            } else {
                Err(UndefinedVariableError::new(token.clone()).into())
            }
        } else {
            panic!("Non Identifier token used to get key from environment!")
        }
    }
}
