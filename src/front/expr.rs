use crate::front::callables::{Callable, Class, Instance};
use crate::front::token::Token;

use std::fmt;
use std::rc::Rc;

#[derive(Debug, Clone)]
pub enum Expr {
    Assign(Assign),
    Binary(Binary),
    Call(Call),
    Get(Get),
    Grouping(Grouping),
    Literal(Literal),
    Logical(Binary),
    Set(Set),
    Super(Super),
    This(This),
    Unary(Unary),
    Ternary(Ternary),
    Variable(Variable),
}
#[derive(Debug, Clone)]
pub struct Super {
    pub keyword: Box<Variable>,
    pub method: Box<Token>
}

#[derive(Debug, Clone)]
pub struct This {
    pub variable: Box<Variable>,
}

#[derive(Debug, Clone)]
pub struct Set {
    pub object: Box<Expr>,
    pub name: Box<Token>,
    pub value: Box<Expr>,
}

#[derive(Debug, Clone)]
pub struct Get {
    pub object: Box<Expr>,
    pub name: Box<Token>,
}

#[derive(Debug, Clone)]
pub struct Assign {
    pub variable: Box<Variable>,
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
    Class(Class),
    Instance(Rc<Instance>),
}

#[derive(Debug, Clone)]
pub struct Unary {
    pub operator: Box<Token>,
    pub right: Box<Expr>,
}

#[derive(Debug, Clone)]
pub struct Variable {
    pub name: Token,
    pub depth: Option<usize>,
}

impl Variable {
    pub fn new(name: Token) -> Self {
        Variable { name, depth: None }
    }
}

pub trait Visitor<'a, T> {
    fn visit_assign(&mut self, assign: &'a Assign) -> T;
    fn visit_binary(&mut self, binary: &'a Binary) -> T;
    fn visit_call(&mut self, call: &'a Call) -> T;
    fn visit_get(&mut self, get: &'a Get) -> T;
    fn visit_grouping(&mut self, grouping: &'a Grouping) -> T;
    fn visit_literal(&mut self, literal: &'a Literal) -> T;
    fn visit_logical(&mut self, logical: &'a Binary) -> T;
    fn visit_unary(&mut self, unary: &'a Unary) -> T;
    fn visit_set(&mut self, set: &'a Set) -> T;
    fn visit_super(&mut self, super_expr: &'a Super) -> T;
    fn visit_ternary(&mut self, ternary: &'a Ternary) -> T;
    fn visit_this(&mut self, this: &'a This) -> T;
    fn visit_variable(&mut self, variable: &'a Variable) -> T;
}

pub trait MutableVisitor<'a, T> {
    fn visit_assign(&mut self, assign: &'a mut Assign) -> T;
    fn visit_binary(&mut self, binary: &'a mut Binary) -> T;
    fn visit_call(&mut self, call: &'a mut Call) -> T;
    fn visit_get(&mut self, get: &'a mut Get) -> T;
    fn visit_grouping(&mut self, grouping: &'a mut Grouping) -> T;
    fn visit_literal(&mut self, literal: &'a mut Literal) -> T;
    fn visit_logical(&mut self, logical: &'a mut Binary) -> T;
    fn visit_unary(&mut self, unary: &'a mut Unary) -> T;
    fn visit_set(&mut self, set: &'a mut Set) -> T;
    fn visit_super(&mut self, super_expr: &'a mut Super) -> T;
    fn visit_ternary(&mut self, ternary: &'a mut Ternary) -> T;
    fn visit_this(&mut self, this: &'a mut This) -> T;
    fn visit_variable(&mut self, variable: &'a mut Variable) -> T;
}

impl<'a> Expr {
    pub fn accept<T, V: Visitor<'a, T>>(&'a self, visitor: &'a mut V) -> T {
        match self {
            Expr::Assign(assign) => visitor.visit_assign(assign),
            Expr::Binary(binary) => visitor.visit_binary(binary),
            Expr::Call(call) => visitor.visit_call(call),
            Expr::Get(get) => visitor.visit_get(get),
            Expr::Grouping(grouping) => visitor.visit_grouping(grouping),
            Expr::Literal(literal) => visitor.visit_literal(literal),
            Expr::Logical(logical) => visitor.visit_logical(logical),
            Expr::Unary(unary) => visitor.visit_unary(unary),
            Expr::Set(set) => visitor.visit_set(set),
            Expr::Super(super_expr) => visitor.visit_super(super_expr),
            Expr::Ternary(ternary) => visitor.visit_ternary(ternary),
            Expr::This(this) => visitor.visit_this(this),
            Expr::Variable(variable) => visitor.visit_variable(variable),
        }
    }

    pub fn accept_mutable<T, V: MutableVisitor<'a, T>>(&'a mut self, visitor: &mut V) -> T {
        match self {
            Expr::Assign(assign) => visitor.visit_assign(assign),
            Expr::Binary(binary) => visitor.visit_binary(binary),
            Expr::Call(call) => visitor.visit_call(call),
            Expr::Get(get) => visitor.visit_get(get),
            Expr::Grouping(grouping) => visitor.visit_grouping(grouping),
            Expr::Literal(literal) => visitor.visit_literal(literal),
            Expr::Logical(logical) => visitor.visit_logical(logical),
            Expr::Unary(unary) => visitor.visit_unary(unary),
            Expr::Set(set) => visitor.visit_set(set),
            Expr::Super(super_expr) => visitor.visit_super(super_expr),
            Expr::Ternary(ternary) => visitor.visit_ternary(ternary),
            Expr::This(this) => visitor.visit_this(this),
            Expr::Variable(variable) => visitor.visit_variable(variable),
        }
    }
}

impl Assign {
    pub fn new(name: Token, value: Expr) -> Assign {
        Assign {
            variable: Box::new(Variable::new(name)),
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

impl Get {
    pub fn new(expr: Expr, name: Token) -> Get {
        Get {
            object: Box::new(expr),
            name: Box::new(name),
        }
    }
}

impl Set {
    pub fn new(expr: Expr, name: Token, value: Expr) -> Set {
        Set {
            object: Box::new(expr),
            name: Box::new(name),
            value: Box::new(value),
        }
    }
}

impl This {
    pub fn new(token: Token) -> This {
        This {
            variable: Box::new(Variable::new(token)),
        }
    }
}

impl Super {
    pub fn new(keyword: Variable, method: Token) -> Super {
        Super {
            keyword: Box::new(keyword),
            method: Box::new(method)
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
            Value::Class(class) => write!(f, "{}", class),
            Value::Instance(instance) => write!(f, "{}", instance),
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
