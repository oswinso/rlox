use crate::front::errors::{RuntimeError, ComposedError};
use crate::front::return_object::ReturnObject;

use std::fmt;

#[derive(Debug)]
pub enum StatementResult {
    RuntimeError(Box<dyn RuntimeError>),
    Return(ReturnObject),
    Break
}

impl From<Box<dyn RuntimeError>> for StatementResult {
    fn from(error: Box<RuntimeError>) -> Self {
        StatementResult::RuntimeError(error)
    }
}

impl From<ReturnObject> for StatementResult {
    fn from(return_object: ReturnObject) -> Self {
        StatementResult::Return(return_object)
    }
}

impl StatementResult {
    pub fn combine_errors(mut vec: Vec<StatementResult>) -> Option<StatementResult> {
        if vec.is_empty() {
            return None;
        }

        if let Some(first) = vec.first() {
            if let StatementResult::Return(return_object) = first {
                if vec.len() != 1 {
                    panic!("Return is first argument but more than 1 element in vec")
                }
                return Some(StatementResult::Return(return_object.clone()));
            }
        }
        let errors: Vec<Box<dyn RuntimeError>> = vec.into_iter().filter_map(|res| {
            if let StatementResult::RuntimeError(error) = res {
                Some(error)
            } else {
                None
            }
        }).collect();
        Some(StatementResult::RuntimeError(Box::new(ComposedError::new(errors))))
    }
}
