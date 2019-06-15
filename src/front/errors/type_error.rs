use crate::front::errors::runtime_error::RuntimeError;
use crate::front::token::Token;
use std::fmt;

#[derive(Debug, Clone)]
pub struct TypeError {
    token: Token,
    message: &'static str,
}

impl TypeError {
    pub fn new(token: Token, message: &'static str) -> Self {
        TypeError { token, message }
    }
}

impl RuntimeError for TypeError {
    fn get_token(&self) -> &Token {
        &self.token
    }
}

impl fmt::Display for TypeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Type error: {}\n[line {}]",
            self.message, self.token.line
        )
    }
}
