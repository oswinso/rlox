use crate::front::expr::{self, Binary, Expr, Grouping, Literal, Ternary, Unary, Value, Variable};
use crate::front::stmt::{self, Declaration, Stmt};
use crate::front::token::Token;
use crate::front::token_type::TokenType;

use crate::front::errors::{RuntimeError, TypeError};

use crate::front::environment::Environment;
use crate::runtime_error;
use std::env::var;

pub struct Interpreter {
    environment: Environment,
}

type RuntimeResult = Result<Value, Box<RuntimeError>>;

impl Interpreter {
    pub fn new() -> Interpreter {
        Interpreter {
            environment: Environment::new(),
        }
    }

    pub fn interpret(&mut self, stmts: &Vec<Stmt>) {
        for stmt in stmts {
            if let Some(error) = self.execute(stmt) {
                runtime_error(error)
            }
        }
    }

    fn evaluate(&self, expr: &Expr) -> RuntimeResult {
        expr.accept(self)
    }

    fn execute(&mut self, stmt: &Stmt) -> Option<Box<RuntimeError>> {
        stmt.accept(self)
    }

    fn handle_unary(&self, token: &Token, value: Value) -> RuntimeResult {
        match token.token_type {
            TokenType::Minus => self.handle_minus(token, value),
            TokenType::Bang => self.handle_negate(token, value),
            _ => panic!("Unary can only be minus or bang"),
        }
    }

    fn handle_binary(&self, token: &Token, left: Value, right: Value) -> RuntimeResult {
        match token.token_type {
            TokenType::Minus => self.handle_subtract(token, left, right),
            TokenType::Plus => self.handle_add(token, left, right),
            TokenType::Slash => self.handle_divide(token, left, right),
            TokenType::Star => self.handle_multiply(token, left, right),
            TokenType::Greater => {
                if let (Some(left), Some(right)) =
                    (self.require_number(left), self.require_number(right))
                {
                    Ok(Value::Literal((left > right).into()))
                } else {
                    Err(
                        TypeError::new(token.clone(), "Greater requires two number operands")
                            .into(),
                    )
                }
            }
            TokenType::GreaterEqual => {
                if let (Some(left), Some(right)) =
                    (self.require_number(left), self.require_number(right))
                {
                    Ok(Value::Literal((left >= right).into()))
                } else {
                    Err(TypeError::new(token.clone(), "GEQ requires two number operands").into())
                }
            }
            TokenType::Less => {
                if let (Some(left), Some(right)) =
                    (self.require_number(left), self.require_number(right))
                {
                    Ok(Value::Literal((left < right).into()))
                } else {
                    Err(TypeError::new(token.clone(), "LE requires two number operands").into())
                }
            }
            TokenType::LessEqual => {
                if let (Some(left), Some(right)) =
                    (self.require_number(left), self.require_number(right))
                {
                    Ok(Value::Literal((left <= right).into()))
                } else {
                    Err(TypeError::new(token.clone(), "LEQ requires two number operands").into())
                }
            }
            TokenType::EqualEqual => Ok(Value::Literal((self.is_equal(left, right)).into())),
            TokenType::BangEqual => Ok(Value::Literal((!self.is_equal(left, right)).into())),
            _ => Err(TypeError::new(token.clone(), "Wrong token for binary expression").into()),
        }
    }

    fn handle_minus(&self, token: &Token, value: Value) -> RuntimeResult {
        if let Some(num) = self.require_number(value) {
            Ok(Value::Literal((-num).into()))
        } else {
            Err(TypeError::new(token.clone(), "Minus unary only defined on numbers.").into())
        }
    }

    fn handle_negate(&self, token: &Token, value: Value) -> RuntimeResult {
        Ok(Value::Literal(Literal::Bool(!self.is_truthy(value))))
    }

    fn is_truthy(&self, value: Value) -> bool {
        if let Value::Literal(Literal::Bool(b)) = value {
            b
        } else if let Value::Literal(Literal::Nil) = value {
            false
        } else {
            true
        }
    }

    fn is_equal(&self, left: Value, right: Value) -> bool {
        match (left, right) {
            (Value::Literal(left), Value::Literal(right)) => match (left, right) {
                (Literal::Bool(left), Literal::Bool(right)) => left == right,
                (Literal::String(left), Literal::String(right)) => left == right,
                (Literal::Number(left), Literal::Number(right)) => left == right,
                _ => false,
            },
        }
    }

    fn handle_subtract(&self, token: &Token, left: Value, right: Value) -> RuntimeResult {
        if let (Some(left), Some(right)) = (self.require_number(left), self.require_number(right)) {
            Ok(Value::Literal((left - right).into()))
        } else {
            Err(TypeError::new(token.clone(), "Subtract requires two number operands").into())
        }
    }

    fn handle_add(&self, token: &Token, left: Value, right: Value) -> RuntimeResult {
        if let (Some(left), Some(right)) = (
            self.require_number(left.clone()),
            self.require_number(right.clone()),
        ) {
            Ok(Value::Literal((left + right).into()))
        } else if let (Some(left), Some(right)) = (
            self.require_string(left.clone()),
            self.require_string(right.clone()),
        ) {
            Ok(Value::Literal((left + &right).into()))
        } else {
            Err(TypeError::new(token.clone(), "Two numbers or two strings required.").into())
        }
    }

    fn handle_divide(&self, token: &Token, left: Value, right: Value) -> RuntimeResult {
        if let (Some(left), Some(right)) = (self.require_number(left), self.require_number(right)) {
            Ok(Value::Literal(Literal::Number(left / right)))
        } else {
            Err(TypeError::new(token.clone(), "Divide requires two number operands").into())
        }
    }

    fn handle_multiply(&self, token: &Token, left: Value, right: Value) -> RuntimeResult {
        if let (Some(left), Some(right)) = (self.require_number(left), self.require_number(right)) {
            Ok(Value::Literal(Literal::Number(left * right)))
        } else {
            Err(TypeError::new(token.clone(), "Multiplication requires two number operands").into())
        }
    }

    fn require_number(&self, value: Value) -> Option<f64> {
        if let Value::Literal(Literal::Number(num)) = value {
            Some(num)
        } else {
            None
        }
    }

    fn require_string(&self, value: Value) -> Option<String> {
        if let Value::Literal(Literal::String(s)) = value {
            Some(s)
        } else {
            None
        }
    }
}

impl expr::Visitor<RuntimeResult> for Interpreter {
    fn visit_binary(&self, binary: &Binary) -> RuntimeResult {
        let left = self.evaluate(&binary.left)?;
        let right = self.evaluate(&binary.right)?;
        self.handle_binary(&binary.operator, left, right)
    }

    fn visit_grouping(&self, grouping: &Grouping) -> RuntimeResult {
        self.evaluate(&grouping.expression)
    }

    fn visit_literal(&self, literal: &Literal) -> RuntimeResult {
        Ok(Value::Literal(literal.clone()))
    }

    fn visit_unary(&self, unary: &Unary) -> RuntimeResult {
        let right = self.evaluate(&unary.right)?;
        self.handle_unary(&unary.operator, right)
    }

    fn visit_ternary(&self, ternary: &Ternary) -> RuntimeResult {
        let condition = self.evaluate(&ternary.condition)?;
        if self.is_truthy(condition) {
            self.evaluate(&ternary.true_branch)
        } else {
            self.evaluate(&ternary.false_branch)
        }
    }

    fn visit_variable(&self, variable: &Variable) -> RuntimeResult {
        self.environment.get(&variable.name)
    }
}

impl stmt::Visitor<Option<Box<RuntimeError>>> for Interpreter {
    fn visit_expression(&mut self, expression: &Expr) -> Option<Box<RuntimeError>> {
        match self.evaluate(expression) {
            Ok(result) => None,
            Err(error) => Some(error),
        }
    }

    fn visit_print(&mut self, expression: &Expr) -> Option<Box<RuntimeError>> {
        match self.evaluate(expression) {
            Ok(result) => {
                println!("{}", result);
                None
            }
            Err(error) => Some(error),
        }
    }

    fn visit_declaration(&mut self, declaration: &Declaration) -> Option<Box<RuntimeError>> {
        let initializer = &declaration.initializer;

        let value = match initializer {
            Some(expr) => self.evaluate(expr),
            None => Ok(Value::Literal(Literal::Nil))
        };

        match value {
            Ok(value) => {
                self.environment
                    .define(declaration.name.lexeme.clone(), value.clone());
                None
            }
            Err(error) => Some(error),
        }
    }
}
