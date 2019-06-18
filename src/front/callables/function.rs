use crate::front::callables::Callable;
use crate::front::expr::{Value, Literal};
use crate::front::interpreter::Interpreter;
use crate::front::stmt::FunctionDecl;

use std::fmt;

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

    fn call(&self, interpreter: &mut Interpreter, arguments: Vec<Value>) -> Value {
        let mut environment = interpreter.globals.clone();
        environment.push();
        for (param, arg) in self.declaration.params.iter().zip(arguments.iter()) {
            environment.define(param.lexeme.clone(), Some(arg.clone()));
        }
        interpreter.execute_block(&self.declaration.body, Some(environment));
        Value::Literal(Literal::Nil)
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
