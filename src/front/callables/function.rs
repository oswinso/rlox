use crate::front::callables::{Callable, Instance};
use crate::front::errors::RuntimeError;
use crate::front::expr::{Literal, Value};
use crate::front::interpreter::Interpreter;
use crate::front::statement_result::StatementResult;
use crate::front::stmt::FunctionDecl;

use crate::front::environment::Environment;
use crate::front::token::Token;
use crate::front::token_type::TokenType;
use std::fmt;
use std::rc::Rc;

#[derive(Clone, Debug)]
pub struct Function {
    declaration: FunctionDecl,
    closure: Environment,
    is_initializer: bool,
}

impl Function {
    pub fn new(declaration: FunctionDecl, closure: Environment, is_initializer: bool) -> Function {
        Function {
            declaration,
            closure,
            is_initializer,
        }
    }

    pub fn bind(&self, instance: Rc<Value>) -> Self {
        let mut environment = self.closure.clone();
        environment.push();
        environment.define(
            "this".into(),
            Some(instance),
        );
        Function::new(self.declaration.clone(), environment, self.is_initializer)
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

        let error = interpreter.execute_block(&self.declaration.body, Some(environment));
        let ret = match error {
            Some(res) => match res {
                StatementResult::Return(return_object) => Ok(Rc::new(return_object.value)),
                StatementResult::RuntimeError(error) => Err(error),
                _ => panic!("Lmao got a break"),
            },
            None => {
                let ret = if self.is_initializer {
                    self.closure.get_at(&Token {
                        token_type: TokenType::This,
                        lexeme: "this".to_string(),
                        line: 314159
                    }, 0)?
                } else {
                    Rc::new(Value::Literal(Literal::Nil))
                };
                Ok(ret)
            },
        };

        if self.is_initializer {
            let this = self.closure.get_at(
                &Token {
                    token_type: TokenType::This,
                    lexeme: "this".to_string(),
                    line: 314159,
                },
                0,
            );
            println!(">> This: ");
            this
        } else {
            ret
        }
    }
}

impl fmt::Display for Function {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "<fn {}>", self.declaration.name)
    }
}
