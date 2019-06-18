use crate::front::callables::Callable;
use crate::front::expr::{Value, Literal};
use crate::front::interpreter::Interpreter;
use crate::front::stmt::FunctionDecl;
use crate::front::errors::RuntimeError;
use crate::front::statement_result::StatementResult;

use std::fmt;
use core::borrow::BorrowMut;

pub struct Function {
    declaration: FunctionDecl,
}

impl Function {
    pub fn new(declaration: FunctionDecl) -> Function {
        Function { declaration }
    }
}

impl Callable for Function {
    fn name(&self) -> &str {
        &self.declaration.name.lexeme
    }

    fn arity(&self) -> usize {
        self.declaration.params.len()
    }

    fn call(&self, interpreter: &mut Interpreter, arguments: Vec<Value>) -> Result<Value, Box<dyn RuntimeError>> {
        let mut environment = interpreter.globals.clone();
        environment.push();
        for (param, arg) in self.declaration.params.iter().zip(arguments.iter()) {
            environment.define(param.lexeme.clone(), Some(arg.clone()));
        }
        if let Some(Value::Literal(Literal::Number(first))) = arguments.first() {
            if *first < -10.0 {
                panic!("RIP")
            }
        }
        let error = interpreter.execute_block(&self.declaration.body, Some(environment));
        match error {
            Some(res) => match res {
                StatementResult::Return(return_object) => Ok(return_object.value),
                StatementResult::RuntimeError(error) => Err(error),
                _ => panic!("Lmao got a break")
            },
            None => Ok(Value::Literal(Literal::Nil))
        }
    }
}

impl fmt::Display for Function {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "<fn {}>",
            self.declaration.name
        )
    }
}
