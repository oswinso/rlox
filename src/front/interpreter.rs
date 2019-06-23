use crate::front::expr::{self, Assign, Binary, Call, Expr, Get, Grouping, Literal, Set, Ternary, This, Unary, Value, Variable, Super};
use crate::front::stmt::{
    self, Block, ClassDecl, Declaration, FunctionDecl, If, Return, Stmt, While,
};
use crate::front::token::Token;
use crate::front::token_type::TokenType;

use crate::front::errors::{IncorrectArgumentsError, RuntimeError, TypeError, UndefinedPropertyError};

use crate::front::callables::{Callable, Class, Clock, Function};
use crate::front::environment::Environment;
use crate::front::return_object::ReturnObject;
use crate::front::statement_result::StatementResult;
use crate::runtime_error;
use core::borrow::{Borrow, BorrowMut};
use std::collections::HashMap;
use std::rc::Rc;

pub struct Interpreter {
    pub globals: Environment,
    pub environment: Environment,
}

type RuntimeResult = Result<Rc<Value>, Box<dyn RuntimeError>>;

impl Interpreter {
    pub fn new() -> Interpreter {
        let mut interpreter = Interpreter {
            globals: Environment::new(),
            environment: Environment::empty_env(),
        };
        interpreter.define_globals();
        interpreter.environment.clone_from(&interpreter.globals);
        interpreter
    }

    pub fn define_globals(&mut self) {
        let funcs: Vec<Rc<Box<dyn Callable>>> = vec![Clock::new()];

        for callable in funcs {
            self.environment.define(
                callable.name().to_owned(),
                Some(Rc::new(Value::Callable(callable))),
            )
        }
    }

    pub fn interpret(&mut self, stmts: &Vec<Stmt>) {
        for stmt in stmts {
            if let Some(res) = self.execute(stmt) {
                if let StatementResult::RuntimeError(error) = res {
                    runtime_error(error)
                }
            }
        }
    }

    pub fn evaluate(&mut self, expr: &Expr) -> RuntimeResult {
        expr.accept(self)
    }

    pub fn execute(&mut self, stmt: &Stmt) -> Option<StatementResult> {
        stmt.accept(self)
    }

    pub fn execute_block(
        &mut self,
        statements: &Vec<Stmt>,
        mut environment: Option<Environment>,
    ) -> Option<StatementResult> {
        if let Some(ref mut env) = environment {
            self.environment.swap(env);
        }
        for statement in statements {
            if let Some(result) = self.execute(statement) {
                if let Some(ref mut env) = environment {
                    self.environment.swap(env);
                }
                return Some(result);
            }
        }
        if let Some(ref mut env) = environment {
            self.environment.swap(env);
        }
        None
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
                    Ok(Rc::new(Value::Literal((left > right).into())))
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
                    Ok(Rc::new(Value::Literal((left >= right).into())))
                } else {
                    Err(TypeError::new(token.clone(), "GEQ requires two number operands").into())
                }
            }
            TokenType::Less => {
                if let (Some(left), Some(right)) =
                    (self.require_number(left), self.require_number(right))
                {
                    Ok(Rc::new(Value::Literal((left < right).into())))
                } else {
                    Err(TypeError::new(token.clone(), "LE requires two number operands").into())
                }
            }
            TokenType::LessEqual => {
                if let (Some(left), Some(right)) =
                    (self.require_number(left), self.require_number(right))
                {
                    Ok(Rc::new(Value::Literal((left <= right).into())))
                } else {
                    Err(TypeError::new(token.clone(), "LEQ requires two number operands").into())
                }
            }
            TokenType::EqualEqual => {
                Ok(Rc::new(Value::Literal((self.is_equal(left, right)).into())))
            }
            TokenType::BangEqual => Ok(Rc::new(Value::Literal(
                (!self.is_equal(left, right)).into(),
            ))),
            _ => Err(TypeError::new(token.clone(), "Wrong token for binary expression").into()),
        }
    }

    fn handle_minus(&self, token: &Token, value: Value) -> RuntimeResult {
        if let Some(num) = self.require_number(value) {
            Ok(Rc::new(Value::Literal((-num).into())))
        } else {
            Err(TypeError::new(token.clone(), "Minus unary only defined on numbers.").into())
        }
    }

    fn handle_negate(&self, _token: &Token, value: Value) -> RuntimeResult {
        Ok(Rc::new(Value::Literal(Literal::Bool(
            !self.is_truthy(&value),
        ))))
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
            _ => false,
        }
    }

    fn handle_subtract(&self, token: &Token, left: Value, right: Value) -> RuntimeResult {
        if let (Some(left), Some(right)) = (self.require_number(left), self.require_number(right)) {
            Ok(Rc::new(Value::Literal((left - right).into())))
        } else {
            Err(TypeError::new(token.clone(), "Subtract requires two number operands").into())
        }
    }

    fn handle_add(&self, token: &Token, left: Value, right: Value) -> RuntimeResult {
        if let (Some(left), Some(right)) = (
            self.require_number(left.clone()),
            self.require_number(right.clone()),
        ) {
            Ok(Rc::new(Value::Literal((left + right).into())))
        } else if let (Some(left), Some(right)) = (
            self.require_string(left.clone()),
            self.require_string(right.clone()),
        ) {
            Ok(Rc::new(Value::Literal((left + &right).into())))
        } else {
            Err(TypeError::new(token.clone(), "Two numbers or two strings required.").into())
        }
    }

    fn handle_divide(&self, token: &Token, left: Value, right: Value) -> RuntimeResult {
        if let (Some(left), Some(right)) = (self.require_number(left), self.require_number(right)) {
            Ok(Rc::new(Value::Literal(Literal::Number(left / right))))
        } else {
            Err(TypeError::new(token.clone(), "Divide requires two number operands").into())
        }
    }

    fn handle_multiply(&self, token: &Token, left: Value, right: Value) -> RuntimeResult {
        if let (Some(left), Some(right)) = (self.require_number(left), self.require_number(right)) {
            Ok(Rc::new(Value::Literal(Literal::Number(left * right))))
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

    fn lookup_variable(&self, variable: &Variable) -> RuntimeResult {
        if let Some(depth) = variable.depth {
            self.environment.get_at(&variable.name, depth)
        } else {
            panic!(
                "Somehow resolver failed to resolve lookup for {}",
                variable.name.lexeme
            )
        }
    }

    fn assign_variable(&self, variable: &Variable, value: &Value) {
        if let Some(depth) = variable.depth {
            self.environment
                .assign_at(&variable.name, value.clone(), depth);
        } else {
            panic!(
                "Somehow resolver failed to resolve assign for {}",
                variable.name.lexeme
            )
        }
    }

    fn try_call(
        &mut self,
        token: Token,
        callable: &Box<dyn Callable>,
        arguments: Vec<Rc<Value>>,
    ) -> RuntimeResult {
        if arguments.len() != callable.arity() {
            return Err(
                IncorrectArgumentsError::new(token, callable.arity(), arguments.len()).into(),
            );
        }
        callable.call(self, arguments)
    }
}

impl expr::Visitor<'_, RuntimeResult> for Interpreter {
    fn visit_assign(&mut self, assign: &Assign) -> RuntimeResult {
        let value = self.evaluate(&assign.value)?;
        self.assign_variable(&assign.variable, &value);
        Ok(value)
    }

    fn visit_binary(&mut self, binary: &Binary) -> RuntimeResult {
        let left = self.evaluate(&binary.left)?;
        let right = self.evaluate(&binary.right)?;
        self.handle_binary(
            &binary.operator,
            left.as_ref().clone(),
            right.as_ref().clone(),
        )
    }

    fn visit_call(&mut self, call: &Call) -> RuntimeResult {
        let callee = self.evaluate(&call.callee)?;

        let mut arguments = Vec::new();
        for argument in &call.arguments {
            let arg = self.evaluate(&argument)?;
            arguments.push(arg);
        }

        match callee.borrow() {
            Value::Callable(callable) => {
                self.try_call(*call.paren.clone(), callable.borrow(), arguments)
            }
            Value::Class(class) => {
                let callable: Box<dyn Callable> = Box::new(class.clone());
                self.try_call(*call.paren.clone(), &callable, arguments)
            }
            _ => Err(
                TypeError::new(*call.paren.clone(), "Can only call functions and classes!").into(),
            ),
        }
    }

    fn visit_get(&mut self, get: &Get) -> RuntimeResult {
        if let Value::Instance(ref instance) = self.evaluate(&get.object)?.borrow() {
            Ok(Rc::new(instance.get(get.name.borrow(), instance.clone())?))
        } else {
            Err(TypeError::new(*get.name.clone(), "Only instances have properties").into())
        }
    }

    fn visit_grouping(&mut self, grouping: &Grouping) -> RuntimeResult {
        self.evaluate(&grouping.expression)
    }

    fn visit_literal(&mut self, literal: &Literal) -> RuntimeResult {
        Ok(Rc::new(Value::Literal(literal.clone())))
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
        let x = &*right.clone();
        self.handle_unary(&unary.operator, x.clone())
    }

    fn visit_set(&mut self, set: &Set) -> RuntimeResult {
        let x = self.evaluate(&set.object)?;
        println!("Setting {}", x);
        if let Value::Instance(ref instance) = x.borrow() {
            let value = self.evaluate(&set.value)?;
            let y = &*value.clone();
            instance.set(&set.name, y.clone());
            dbg!(instance);
            Ok(value)
        } else {
            Err(TypeError::new(*set.name.clone(), "Only instances have fields").into())
        }
    }

    fn visit_super(&mut self, super_expr: &Super) -> Result<Rc<Value>, Box<dyn RuntimeError>> {
        let depth = super_expr.keyword.depth.unwrap();
        let superclass = self.environment.get_at(&super_expr.keyword.name, depth)?;
        let object = self.environment.get_at(&Token {
            token_type: TokenType::This,
            lexeme: "this".to_string(),
            line: 314159
        }, depth - 1)?;
        if let Value::Class(class) = superclass.borrow() {
            let method = class.find_method(&super_expr.method.lexeme);
            if let Some(method) = method {
                if let Value::Instance(this) = object.borrow() {
                    let bound_method = method.bind(object);
                    Ok(Rc::new(Value::Callable(Rc::new(Box::new(bound_method)))))
                } else {
                    panic!("'this' isn't an instance...")
                }
            } else {
                Err(UndefinedPropertyError::new(class.name().into(), *super_expr.method.clone()).into())
            }
        } else {
            panic!("Super wasn't a class...")
        }
    }

    fn visit_ternary(&mut self, ternary: &Ternary) -> RuntimeResult {
        let condition = self.evaluate(&ternary.condition)?;
        if self.is_truthy(&condition) {
            self.evaluate(&ternary.true_branch)
        } else {
            self.evaluate(&ternary.false_branch)
        }
    }

    fn visit_this(&mut self, this: &This) -> Result<Rc<Value>, Box<dyn RuntimeError>> {
        self.lookup_variable(&this.variable)
    }

    fn visit_variable(&mut self, variable: &Variable) -> RuntimeResult {
        self.lookup_variable(variable)
    }
}

impl stmt::Visitor<Option<StatementResult>> for Interpreter {
    fn visit_block(&mut self, block: &Block) -> Option<StatementResult> {
        self.environment.push();
        let res = self.execute_block(&block.statements, None);
        self.environment.pop();
        res.map(StatementResult::from)
    }

    fn visit_class(&mut self, class_decl: &ClassDecl) -> Option<StatementResult> {
        let superclass = if let Some(variable) = &class_decl.superclass {
            let superclass = self.evaluate(&Expr::Variable(*variable.clone()));
            match superclass {
                Ok(value) => {
                    if let Value::Class(class) = value.borrow() {
                        Some(Box::new(class.clone()))
                    } else {
                        return Some(StatementResult::RuntimeError(TypeError::new(variable.name.clone(), "Superclass must be a class").into()));
                    }
                }
                Err(err) => return Some(StatementResult::RuntimeError(err))
            }
        } else {
            None
        };

        self.environment
            .define(class_decl.name.lexeme.clone(), None);

        if let Some(superclass) = &superclass {
            self.environment.push();
            self.environment.define("super".into(), Some(Rc::new(Value::Class(*superclass.clone()))));
        }

        let mut methods = HashMap::new();
        for method in &class_decl.methods {
            let function = Function::new(method.clone(), self.environment.clone(), method.name.lexeme == class_decl.name.lexeme);
            methods.insert(method.name.lexeme.clone(), function);
        }

        let class = Class::new(class_decl.name.lexeme.clone(), superclass.clone(), methods);

        if let Some(_) = superclass {
            self.environment.pop();
        }

        self.environment
            .assign(&class_decl.name, Value::Class(class));
        None
    }

    fn visit_expression(&mut self, expression: &Expr) -> Option<StatementResult> {
        match self.evaluate(expression) {
            Ok(val) => {
                if !self.environment.is_global() {
                    return None;
                }
                match val.borrow() {
                    Value::Literal(literal) => {
                        match literal {
                            Literal::Nil => (),
                            _ => println!("{}", literal),
                        };
                        None
                    }
                    _ => None,
                }
            }
            Err(error) => Some(error.into()),
        }
    }

    fn visit_function(&mut self, function_decl: &FunctionDecl) -> Option<StatementResult> {
        let function = Function::new(function_decl.clone(), self.environment.clone(), false);
        let callable = Value::Callable(Rc::new(Box::new(function)));
        self.environment
            .define(function_decl.name.lexeme.clone(), Some(Rc::new(callable)));
        None
    }

    fn visit_if(&mut self, if_stmt: &If) -> Option<StatementResult> {
        let res = self.evaluate(&if_stmt.condition);
        match res {
            Ok(condition) => {
                if self.is_truthy(&condition) {
                    self.execute(&if_stmt.then_branch).map(|x| x.into())
                } else {
                    if_stmt
                        .else_branch
                        .as_ref()
                        .and_then(|else_branch| self.execute(&else_branch))
                        .map(|x| x.into())
                }
            }
            Err(error) => Some(error.into()),
        }
    }

    fn visit_print(&mut self, expression: &Expr) -> Option<StatementResult> {
        match self.evaluate(expression) {
            Ok(result) => {
                println!("{}", result);
                None
            }
            Err(error) => Some(error.into()),
        }
    }

    fn visit_return(&mut self, ret: &Return) -> Option<StatementResult> {
        let value = ret
            .value
            .as_ref()
            .map_or(Ok(Rc::new(Value::Literal(Literal::Nil))), |expr| {
                self.evaluate(&expr)
            });

        match value {
            Ok(value) => {
                let x = &*value.clone();
                Some(ReturnObject::new(x.clone()).into())
            }
            Err(error) => Some(error.into()),
        }
    }

    fn visit_declaration(&mut self, declaration: &Declaration) -> Option<StatementResult> {
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
            Err(error) => Some(error.into()),
        }
    }

    fn visit_while(&mut self, while_stmt: &While) -> Option<StatementResult> {
        while {
            let condition = self.evaluate(&while_stmt.condition);
            match condition {
                Ok(value) => self.is_truthy(&value),
                Err(error) => {
                    return Some(error.into());
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
