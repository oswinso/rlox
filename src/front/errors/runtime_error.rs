use crate::front::token::Token;
use std::error;

pub trait RuntimeError: std::fmt::Display + std::fmt::Debug {
    fn get_token(&self) -> &Token;
}

impl error::Error for dyn RuntimeError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        None
    }
}

impl<T: 'static + RuntimeError> From<T> for Box<dyn RuntimeError> {
    fn from(rte: T) -> Self {
        Box::new(rte)
    }
}
