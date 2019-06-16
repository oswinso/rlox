use crate::front::expr::Expr;
use crate::front::token::Token;

#[derive(Debug, Clone)]
pub enum Stmt {
    Block(Block),
    Expression(Expr),
    Print(Expr),
    Declaration(Declaration),
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
    fn visit_print(&mut self, expression: &Expr) -> T;
    fn visit_declaration(&mut self, declaration: &Declaration) -> T;
}

impl Stmt {
    pub fn accept<T, V: Visitor<T>>(&self, visitor: &mut V) -> T {
        match self {
            Stmt::Block(block) => visitor.visit_block(block),
            Stmt::Expression(expr) => visitor.visit_expression(expr),
            Stmt::Print(expr) => visitor.visit_print(expr),
            Stmt::Declaration(declaration) => visitor.visit_declaration(declaration),
        }
    }
}

impl Block {
    pub fn new(statements: Vec<Stmt>) -> Block {
        Block { statements }
    }
}
