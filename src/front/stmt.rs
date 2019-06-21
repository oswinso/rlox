use crate::front::expr::Expr;
use crate::front::token::Token;

#[derive(Debug, Clone)]
pub enum Stmt {
    Block(Block),
    Expression(Expr),
    Function(FunctionDecl),
    If(If),
    Print(Expr),
    Return(Return),
    While(While),
    Declaration(Declaration),
}
#[derive(Debug, Clone)]
pub struct Return {
    pub keyword: Box<Token>,
    pub value: Option<Box<Expr>>,
}

#[derive(Debug, Clone)]
pub struct FunctionDecl {
    pub name: Box<Token>,
    pub params: Vec<Token>,
    pub body: Vec<Stmt>,
}

#[derive(Debug, Clone)]
pub struct While {
    pub condition: Box<Expr>,
    pub body: Box<Stmt>,
}

#[derive(Debug, Clone)]
pub struct If {
    pub condition: Box<Expr>,
    pub then_branch: Box<Stmt>,
    pub else_branch: Option<Box<Stmt>>,
}

#[derive(Debug, Clone)]
pub struct Block {
    pub statements: Vec<Stmt>,
}

#[derive(Debug, Clone)]
pub struct Declaration {
    pub name: Token,
    pub initializer: Option<Expr>,
}

pub trait Visitor<T> {
    fn visit_block(&mut self, block: &Block) -> T;
    fn visit_expression(&mut self, expression: &Expr) -> T;
    fn visit_function(&mut self, function: &FunctionDecl) -> T;
    fn visit_if(&mut self, if_stmt: &If) -> T;
    fn visit_print(&mut self, expression: &Expr) -> T;
    fn visit_return(&mut self, ret: &Return) -> T;
    fn visit_declaration(&mut self, declaration: &Declaration) -> T;
    fn visit_while(&mut self, while_stmt: &While) -> T;
}

pub trait MutableVisitor<'a, T> {
    fn visit_block(&mut self, block: &'a mut Block) -> T;
    fn visit_expression(&mut self, expression: &'a mut Expr) -> T;
    fn visit_function(&mut self, function: &'a mut FunctionDecl) -> T;
    fn visit_if(&mut self, if_stmt: &'a mut If) -> T;
    fn visit_print(&mut self, expression: &'a mut Expr) -> T;
    fn visit_return(&mut self, ret: &'a mut Return) -> T;
    fn visit_declaration(&mut self, declaration: &'a mut Declaration) -> T;
    fn visit_while(&mut self, while_stmt: &'a mut While) -> T;
}

impl<'a> Stmt {
    pub fn accept<T, V: Visitor<T>>(&'a self, visitor: &mut V) -> T {
        match self {
            Stmt::Block(block) => visitor.visit_block(block),
            Stmt::Expression(expr) => visitor.visit_expression(expr),
            Stmt::Function(function) => visitor.visit_function(function),
            Stmt::If(if_stmt) => visitor.visit_if(if_stmt),
            Stmt::Print(expr) => visitor.visit_print(expr),
            Stmt::Return(ret) => visitor.visit_return(ret),
            Stmt::Declaration(declaration) => visitor.visit_declaration(declaration),
            Stmt::While(while_stmt) => visitor.visit_while(while_stmt),
        }
    }

    pub fn accept_mutable<T, V: MutableVisitor<'a, T>>(&'a mut self, visitor: &mut V) -> T {
        match self {
            Stmt::Block(block) => visitor.visit_block(block),
            Stmt::Expression(expr) => visitor.visit_expression(expr),
            Stmt::Function(function) => visitor.visit_function(function),
            Stmt::If(if_stmt) => visitor.visit_if(if_stmt),
            Stmt::Print(expr) => visitor.visit_print(expr),
            Stmt::Return(ret) => visitor.visit_return(ret),
            Stmt::Declaration(declaration) => visitor.visit_declaration(declaration),
            Stmt::While(while_stmt) => visitor.visit_while(while_stmt),
        }
    }
}

impl Block {
    pub fn new(statements: Vec<Stmt>) -> Block {
        Block { statements }
    }
}

impl If {
    pub fn new(condition: Expr, then_branch: Stmt, else_branch: Option<Stmt>) -> Self {
        If {
            condition: Box::new(condition),
            then_branch: Box::new(then_branch),
            else_branch: else_branch.map(|stmt| Box::new(stmt)),
        }
    }
}

impl While {
    pub fn new(condition: Expr, body: Stmt) -> Self {
        While {
            condition: Box::new(condition),
            body: Box::new(body),
        }
    }
}

impl FunctionDecl {
    pub fn new(name: &Token, params: Vec<Token>, body: Vec<Stmt>) -> Self {
        FunctionDecl {
            name: Box::new(name.clone()),
            params,
            body,
        }
    }
}

impl Return {
    pub fn new(keyword: Token, value: Option<Expr>) -> Return {
        Return {
            keyword: Box::new(keyword),
            value: value.map(|x| Box::new(x)),
        }
    }
}
