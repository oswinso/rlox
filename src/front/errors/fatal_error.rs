use crate::front::errors::runtime_error::RuntimeError;
use crate::front::token::Token;
use std::fmt;

#[derive(Debug, Clone)]
pub struct FatalError {
    token: Token,
    message: String,
}

impl FatalError {
    pub fn new(token: Token, message: String) -> Self {
        FatalError { token, message }
    }
}

impl RuntimeError for FatalError {
    fn get_token(&self) -> &Token {
        &self.token
    }
}

impl fmt::Display for FatalError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}\n[line {}]", self.message, self.token.line)
    }
}
