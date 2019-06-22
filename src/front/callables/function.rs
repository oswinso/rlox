use crate::front::callables::Callable;
use crate::front::errors::RuntimeError;
use crate::front::expr::{Literal, Value};
use crate::front::interpreter::Interpreter;
use crate::front::statement_result::StatementResult;
use crate::front::stmt::FunctionDecl;

use crate::front::environment::Environment;
use std::fmt;
use std::rc::Rc;

pub struct Function {
    declaration: FunctionDecl,
    closure: Environment,
}

impl Function {
    pub fn new(declaration: FunctionDecl, closure: Environment) -> Function {
        Function {
            declaration,
            closure,
        }
    }
}

impl Callable for Function {
    fn name(&self) -> &str {
        &self.declaration.name.lexeme
    }

    fn arity(&self) -> usize {
        self.declaration.params.len()
    }

    fn call(
        &self,
        interpreter: &mut Interpreter,
        arguments: Vec<Rc<Value>>,
    ) -> Result<Rc<Value>, Box<dyn RuntimeError>> {
        let mut environment = self.closure.clone();
        environment.push();
        for (param, arg) in self.declaration.params.iter().zip(arguments.iter()) {
            environment.define(param.lexeme.clone(), Some(arg.clone()));
        }
        // WTF I don't remember why
//        if let Some(Value::Literal(Literal::Number(first))) = arguments.first() {
//            if *first < -10.0 {
//                panic!("RIP")
//            }
//        }
        let error = interpreter.execute_block(&self.declaration.body, Some(environment));
        match error {
            Some(res) => match res {
                StatementResult::Return(return_object) => Ok(Rc::new(return_object.value)),
                StatementResult::RuntimeError(error) => Err(error),
                _ => panic!("Lmao got a break"),
            },
            None => Ok(Rc::new(Value::Literal(Literal::Nil))),
        }
    }
}

impl fmt::Display for Function {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "<fn {}>", self.declaration.name)
    }
}
