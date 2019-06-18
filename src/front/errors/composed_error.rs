use crate::front::errors::runtime_error::RuntimeError;
use crate::front::token::Token;
use std::fmt;

#[derive(Debug)]
pub struct ComposedError {
    errors: Vec<Box<dyn RuntimeError>>,
}

impl ComposedError {
    pub fn new(errors: Vec<Box<dyn RuntimeError>>) -> ComposedError {
        ComposedError { errors }
    }

    pub fn from(errors: Vec<Box<dyn RuntimeError>>) -> Option<Box<dyn RuntimeError>> {
        if errors.is_empty() {
            None
        } else {
            Some(Box::new(ComposedError::new(errors)))
        }
    }
}

impl RuntimeError for ComposedError {
    fn get_token(&self) -> &Token {
        self.errors.first().unwrap().get_token()
    }
}

impl fmt::Display for ComposedError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for error in &self.errors {
            write!(f, "{}", error).unwrap();
        }
        write!(f, "")
    }
}
