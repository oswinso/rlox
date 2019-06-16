use crate::front::expr::{
    self, Assign, Binary, Expr, Grouping, Literal, Ternary, Unary, Value, Variable,
};
use crate::front::stmt::{self, Block, Declaration, If, Stmt, While};
use crate::front::token::Token;
use crate::front::token_type::TokenType;

use crate::front::errors::{ComposedError, RuntimeError, TypeError};

use crate::front::environment::Environment;
use crate::runtime_error;

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

    fn evaluate(&mut self, expr: &Expr) -> RuntimeResult {
        expr.accept(self)
    }

    fn execute(&mut self, stmt: &Stmt) -> Option<Box<RuntimeError>> {
        stmt.accept(self)
    }

    fn execute_block<'b>(&mut self, statements: &Vec<Stmt>) -> Option<Box<RuntimeError>> {
        let error_vec: Vec<Box<RuntimeError>> = statements
            .iter()
            .map(|statement| self.execute(statement))
            .flatten()
            .collect();
        ComposedError::from(error_vec)
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
        Ok(Value::Literal(Literal::Bool(!self.is_truthy(&value))))
    }

    fn is_truthy(&self, value: &Value) -> bool {
        if let Value::Literal(Literal::Bool(b)) = value {
            b.clone()
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
    fn visit_assign(&mut self, assign: &Assign) -> Result<Value, Box<RuntimeError>> {
        let value = self.evaluate(&assign.value)?;
        self.environment.assign(&assign.name, value.clone());
        Ok(value)
    }

    fn visit_binary(&mut self, binary: &Binary) -> RuntimeResult {
        let left = self.evaluate(&binary.left)?;
        let right = self.evaluate(&binary.right)?;
        self.handle_binary(&binary.operator, left, right)
    }

    fn visit_grouping(&mut self, grouping: &Grouping) -> RuntimeResult {
        self.evaluate(&grouping.expression)
    }

    fn visit_literal(&mut self, literal: &Literal) -> RuntimeResult {
        Ok(Value::Literal(literal.clone()))
    }

    fn visit_logical(&mut self, logical: &Binary) -> RuntimeResult {
        let left = self.evaluate(&logical.left)?;

        if logical.operator.token_type == TokenType::Or {
            if self.is_truthy(&left) {
                return Ok(left);
            }
        } else if logical.operator.token_type == TokenType::And {
            if !self.is_truthy(&left) {
                return Ok(left);
            }
        }

        self.evaluate(&logical.right)
    }

    fn visit_unary(&mut self, unary: &Unary) -> RuntimeResult {
        let right = self.evaluate(&unary.right)?;
        self.handle_unary(&unary.operator, right)
    }

    fn visit_ternary(&mut self, ternary: &Ternary) -> RuntimeResult {
        let condition = self.evaluate(&ternary.condition)?;
        if self.is_truthy(&condition) {
            self.evaluate(&ternary.true_branch)
        } else {
            self.evaluate(&ternary.false_branch)
        }
    }

    fn visit_variable(&mut self, variable: &Variable) -> RuntimeResult {
        self.environment.get(&variable.name)
    }
}

impl stmt::Visitor<Option<Box<RuntimeError>>> for Interpreter {
    fn visit_block(&mut self, block: &Block) -> Option<Box<RuntimeError>> {
        self.environment.push();
        let res = self.execute_block(&block.statements);
        self.environment.pop();
        res
    }

    fn visit_expression(&mut self, expression: &Expr) -> Option<Box<RuntimeError>> {
        match self.evaluate(expression) {
            Ok(val) => match val {
                Value::Literal(literal) => {
                    println!("{}", literal);
                    None
                }
            },
            Err(error) => Some(error),
        }
    }

    fn visit_if(&mut self, if_stmt: &If) -> Option<Box<RuntimeError>> {
        let res = self.evaluate(&if_stmt.condition);
        match res {
            Ok(condition) => {
                if self.is_truthy(&condition) {
                    self.execute(&if_stmt.then_branch)
                } else {
                    if_stmt
                        .else_branch
                        .as_ref()
                        .and_then(|else_branch| self.execute(&else_branch))
                }
            }
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
            Some(expr) => self.evaluate(&expr).map(|val| Some(val)),
            None => Ok(None),
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

    fn visit_while(&mut self, while_stmt: &While) -> Option<Box<RuntimeError>> {
        while {
            let condition = self.evaluate(&while_stmt.condition);
            match condition {
                Ok(value) => self.is_truthy(&value),
                Err(error) => {
                    return Some(error);
                }
            }
        } {
            self.execute(&while_stmt.body).map(|error| {
                return error;
            });
        }
        None
    }
}
