use crate::vm::errors::{CompileError, RuntimeError};

pub enum VMError {
    CompileError(CompileError),
    RuntimeError(RuntimeError),
}
