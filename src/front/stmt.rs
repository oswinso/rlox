use crate::front::expr::Expr;
use crate::front::token::Token;

#[derive(Debug, Clone)]
pub enum Stmt {
    Expression(Expr),
    Print(Expr),
    Declaration(Declaration),
}

#[derive(Debug, Clone)]
pub struct Declaration {
    pub name: Token,
    pub initializer: Option<Expr>,
}

pub trait Visitor<T> {
    fn visit_expression(&mut self, expression: &Expr) -> T;
    fn visit_print(&mut self, expression: &Expr) -> T;
    fn visit_declaration(&mut self, declaration: &Declaration) -> T;
}

impl Stmt {
    pub fn accept<T, V: Visitor<T>>(&self, visitor: &mut V) -> T {
        match self {
            Stmt::Expression(expr) => visitor.visit_expression(expr),
            Stmt::Print(expr) => visitor.visit_print(expr),
            Stmt::Declaration(declaration) => visitor.visit_declaration(declaration),
        }
    }
}
