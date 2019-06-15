use crate::front::errors::runtime_error::RuntimeError;
use crate::front::token::Token;
use std::fmt;

#[derive(Debug, Clone)]
pub struct UndefinedVariableError {
    token: Token,
}

impl UndefinedVariableError {
    pub fn new(token: Token) -> Self {
        UndefinedVariableError { token }
    }
}

impl RuntimeError for UndefinedVariableError {
    fn get_token(&self) -> &Token {
        &self.token
    }
}

impl fmt::Display for UndefinedVariableError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Variable {} is undefined.\n[line {}]",
            self.token.lexeme, self.token.line
        )
    }
}
