use crate::front::errors::runtime_error::RuntimeError;
use crate::front::token::Token;
use std::fmt;

#[derive(Debug, Clone)]
pub struct IncorrectArgumentsError {
    token: Token,
    expected_num: usize,
    actual_num: usize,
}

impl IncorrectArgumentsError {
    pub fn new(token: Token, expected_num: usize, actual_num: usize) -> Self {
        IncorrectArgumentsError {
            token,
            expected_num,
            actual_num,
        }
    }
}

impl RuntimeError for IncorrectArgumentsError {
    fn get_token(&self) -> &Token {
        &self.token
    }
}

impl fmt::Display for IncorrectArgumentsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Incorrect number of arguments. Expected {} arguments but got {}.\n[line {}]",
            self.expected_num, self.actual_num, self.token.line
        )
    }
}
