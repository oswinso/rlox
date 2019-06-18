use crate::front::callables::Callable;
use crate::front::token::Token;

use std::fmt;
use std::rc::Rc;

#[derive(Debug, Clone)]
pub enum Expr {
    Assign(Assign),
    Binary(Binary),
    Call(Call),
    Grouping(Grouping),
    Literal(Literal),
    Logical(Binary),
    Unary(Unary),
    Ternary(Ternary),
    Variable(Variable),
}
#[derive(Debug, Clone)]
pub struct Assign {
    pub name: Box<Token>,
    pub value: Box<Expr>,
}

#[derive(Debug, Clone)]
pub struct Binary {
    pub left: Box<Expr>,
    pub operator: Box<Token>,
    pub right: Box<Expr>,
}

#[derive(Debug, Clone)]
pub struct Call {
    pub callee: Box<Expr>,
    pub paren: Box<Token>,
    pub arguments: Vec<Expr>,
}

#[derive(Debug, Clone)]
pub struct Ternary {
    pub condition: Box<Expr>,
    pub true_branch: Box<Expr>,
    pub false_branch: Box<Expr>,
}

#[derive(Debug, Clone)]
pub struct Grouping {
    pub expression: Box<Expr>,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Literal {
    String(String),
    Number(f64),
    Bool(bool),
    Nil,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Value {
    Literal(Literal),
    Callable(Rc<Box<dyn Callable>>),
}

#[derive(Debug, Clone)]
pub struct Unary {
    pub operator: Box<Token>,
    pub right: Box<Expr>,
}

#[derive(Debug, Clone)]
pub struct Variable {
    pub name: Token,
}

pub trait Visitor<T> {
    fn visit_assign(&mut self, assign: &Assign) -> T;
    fn visit_binary(&mut self, binary: &Binary) -> T;
    fn visit_call(&mut self, call: &Call) -> T;
    fn visit_grouping(&mut self, grouping: &Grouping) -> T;
    fn visit_literal(&mut self, literal: &Literal) -> T;
    fn visit_logical(&mut self, logical: &Binary) -> T;
    fn visit_unary(&mut self, unary: &Unary) -> T;
    fn visit_ternary(&mut self, ternary: &Ternary) -> T;
    fn visit_variable(&mut self, variable: &Variable) -> T;
}

impl Expr {
    pub fn accept<T, V: Visitor<T>>(&self, visitor: &mut V) -> T {
        match self {
            Expr::Assign(assign) => visitor.visit_assign(assign),
            Expr::Binary(binary) => visitor.visit_binary(binary),
            Expr::Call(call) => visitor.visit_call(call),
            Expr::Grouping(grouping) => visitor.visit_grouping(grouping),
            Expr::Literal(literal) => visitor.visit_literal(literal),
            Expr::Logical(logical) => visitor.visit_logical(logical),
            Expr::Unary(unary) => visitor.visit_unary(unary),
            Expr::Ternary(ternary) => visitor.visit_ternary(ternary),
            Expr::Variable(variable) => visitor.visit_variable(variable),
        }
    }
}

impl Assign {
    pub fn new(name: Token, value: Expr) -> Assign {
        Assign {
            name: Box::new(name),
            value: Box::new(value),
        }
    }
}

impl Binary {
    pub fn new(left: Expr, operator: Token, right: Expr) -> Binary {
        Binary {
            left: Box::new(left),
            operator: Box::new(operator),
            right: Box::new(right),
        }
    }
}

impl Call {
    pub fn new(callee: Expr, paren: Token, arguments: Vec<Expr>) -> Call {
        Call {
            callee: Box::new(callee),
            paren: Box::new(paren),
            arguments,
        }
    }
}

impl Grouping {
    pub fn new(expr: Expr) -> Grouping {
        Grouping {
            expression: Box::new(expr),
        }
    }
}

impl Unary {
    pub fn new(operator: Token, expr: Expr) -> Unary {
        Unary {
            operator: Box::new(operator),
            right: Box::new(expr),
        }
    }
}

impl Ternary {
    pub fn new(condition: Expr, true_branch: Expr, false_branch: Expr) -> Ternary {
        Ternary {
            condition: Box::new(condition),
            true_branch: Box::new(true_branch),
            false_branch: Box::new(false_branch),
        }
    }
}

impl fmt::Display for Literal {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        match self {
            Literal::String(s) => write!(f, "{}", s),
            Literal::Number(n) => write!(f, "{}", n),
            Literal::Bool(b) => write!(f, "{}", b),
            Literal::Nil => write!(f, "Nil"),
        }
    }
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Value::Literal(literal) => write!(f, "{}", literal),
            Value::Callable(c) => write!(f, "{}", c),
        }
    }
}

impl From<f64> for Literal {
    fn from(num: f64) -> Self {
        Literal::Number(num)
    }
}

impl From<String> for Literal {
    fn from(s: String) -> Self {
        Literal::String(s)
    }
}

impl From<bool> for Literal {
    fn from(b: bool) -> Self {
        Literal::Bool(b)
    }
}

impl From<Literal> for Value {
    fn from(literal: Literal) -> Self {
        Value::Literal(literal)
    }
}

impl From<Literal> for Expr {
    fn from(literal: Literal) -> Self {
        Expr::Literal(literal)
    }
}
