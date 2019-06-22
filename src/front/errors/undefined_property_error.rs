use crate::front::errors::runtime_error::RuntimeError;
use crate::front::token::Token;
use std::fmt;

#[derive(Debug, Clone)]
pub struct UndefinedPropertyError {
    instance_class: String,
    property: Token,
}

impl UndefinedPropertyError {
    pub fn new(instance_class: String, property: Token) -> Self {
        UndefinedPropertyError {
            instance_class,
            property,
        }
    }
}

impl RuntimeError for UndefinedPropertyError {
    fn get_token(&self) -> &Token {
        &self.property
    }
}

impl fmt::Display for UndefinedPropertyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Undefined property {} for instance of class {}.\n[line {}]",
            self.property.lexeme, self.instance_class, self.property.line
        )
    }
}
