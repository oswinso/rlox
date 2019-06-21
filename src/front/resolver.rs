use crate::{error, warn};
use crate::front::expr::{
    self, Assign, Binary, Call, Expr, Grouping, Literal, Ternary, Unary, Variable,
};
use crate::front::stmt::{self, Block, Declaration, FunctionDecl, If, Return, Stmt, While};
use crate::front::token::Token;
use std::collections::HashMap;

#[derive(PartialEq)]
enum FunctionType {
    None,
    Function
}

struct VariableStatus {
    declaration_token: Token,
    defined: bool,
    used: bool
}

impl VariableStatus {
    pub fn new(declaration_token: Token) -> Self {
        VariableStatus {
            declaration_token,
            defined: false,
            used: false
        }
    }
}

pub struct Resolver {
    scopes: Vec<HashMap<String, VariableStatus>>,
    current_function: FunctionType,
}

impl Resolver {
    pub fn new() -> Resolver {
        Resolver {
            scopes: Vec::new(),
            current_function: FunctionType::None,
        }
    }

    pub fn resolve(&mut self, statements: & mut [Stmt]) {
        self.begin_scope();
        self.resolve_stmts(statements);
        self.end_scope();
    }

    fn resolve_stmt(&mut self, statement: & mut Stmt) {
        statement.accept_mutable(self);
    }

    fn resolve_stmts(&mut self, statements: & mut [Stmt]) {
        for statement in statements {
            self.resolve_stmt(statement);
        }
    }

    fn resolve_expr(&mut self, expr: & mut Expr) {
        expr.accept_mutable(self);
    }

    fn resolve_local(&mut self, variable: & mut Variable) {
        for (i, scope) in self.scopes.iter_mut().rev().enumerate() {
            if scope.contains_key(&variable.name.lexeme) {
                scope.get_mut(&variable.name.lexeme).unwrap().used = true;
                variable.depth = Some(i);
                return;
            }
        }
        // Went through all scopes and didn't find anything
        error(
            variable.name.line,
            &format!("Couldn't resolve variable {}", variable.name.lexeme),
        );
    }

    fn resolve_function(&mut self, function_decl: & mut FunctionDecl, mut function_type: FunctionType) {
        std::mem::swap(&mut function_type, &mut self.current_function);
        self.begin_scope();
        for param in &function_decl.params {
            self.declare(&param);
            self.define(&param);
        }
        self.resolve_stmts(&mut function_decl.body);
        self.end_scope();
        std::mem::swap(&mut function_type, &mut self.current_function);
    }

    fn begin_scope(&mut self) {
        self.scopes.push(HashMap::new());
    }

    fn end_scope(&mut self) {
        if let Some(scope) = self.scopes.pop() {
            for (_, entry) in scope.into_iter() {
                if !entry.used {
                    warn(entry.declaration_token.line, &format!("Variable {} is declared but never used.", entry.declaration_token.lexeme))
                }
            }
        }
    }

    fn declare(&mut self, token: &Token) {
        if let Some(scope) = self.scopes.last_mut() {
            if scope.contains_key(&token.lexeme) {
                error(token.line, &format!("Variable with name {} is already declared in this scope.", token.lexeme));
            } else {
                scope.insert(token.lexeme.clone(), VariableStatus::new(token.clone()));
            }
        }
    }

    fn define(&mut self, token: &Token) {
        if let Some(scope) = self.scopes.last_mut() {
            scope.get_mut(&token.lexeme).unwrap().defined = true;
        }
    }
}

impl<'a> stmt::MutableVisitor<'a, ()> for Resolver {
    fn visit_block(&mut self, block: &mut Block) -> () {
        self.begin_scope();
        self.resolve_stmts(&mut block.statements);
        self.end_scope();
    }

    fn visit_expression(&mut self, expression: &mut Expr) -> () {
        self.resolve_expr(expression);
    }

    fn visit_function(&mut self, function: &mut FunctionDecl) -> () {
        self.declare(&function.name);
        self.define(&function.name);

        self.resolve_function(function, FunctionType::Function);
    }

    fn visit_if(&mut self, if_stmt: &mut If) -> () {
        self.resolve_expr(&mut if_stmt.condition);
        self.resolve_stmt(&mut if_stmt.then_branch);
        if let Some(ref mut stmt) = if_stmt.else_branch {
            self.resolve_stmt(stmt);
        }
    }

    fn visit_print(&mut self, expression: &mut Expr) -> () {
        self.resolve_expr(expression);
    }

    fn visit_return(&mut self, ret: &mut Return) -> () {
        if self.current_function == FunctionType::None {
            error(ret.keyword.line, "Cannot return from top level code.");
            return;
        }

        if let Some(ref mut expr) = ret.value {
            self.resolve_expr(expr);
        }
    }

    fn visit_declaration(&mut self, declaration: &mut Declaration) -> () {
        self.declare(&declaration.name);
        if let Some(initializer) = &mut declaration.initializer {
            self.resolve_expr(initializer);
        }
        self.define(&declaration.name);
    }

    fn visit_while(&mut self, while_stmt: &mut While) -> () {
        self.resolve_expr(&mut while_stmt.condition);
        self.resolve_stmt(&mut while_stmt.body);
    }
}

impl<'a> expr::MutableVisitor<'a, ()> for Resolver {
    fn visit_assign(&mut self, assign: &mut Assign) -> () {
        self.resolve_expr(&mut assign.value);
        self.resolve_local(&mut assign.variable);
    }

    fn visit_binary(&mut self, binary: &mut Binary) -> () {
        self.resolve_expr(&mut binary.left);
        self.resolve_expr(&mut binary.right);
    }

    fn visit_call(&mut self, call: &mut Call) -> () {
        self.resolve_expr(&mut call.callee);

        for arg in &mut call.arguments {
            self.resolve_expr(arg);
        }
    }

    fn visit_grouping(&mut self, grouping: &mut Grouping) -> () {
        self.resolve_expr(&mut grouping.expression);
    }

    fn visit_literal(&mut self, literal: &mut Literal) -> () {}

    fn visit_logical(&mut self, logical: &mut Binary) -> () {
        self.resolve_expr(&mut logical.left);
        self.resolve_expr(&mut logical.right);
    }

    fn visit_unary(&mut self, unary: &mut Unary) -> () {
        self.resolve_expr(&mut unary.right);
    }

    fn visit_ternary(&mut self, ternary: &mut Ternary) -> () {
        self.resolve_expr(&mut ternary.condition);
        self.resolve_expr(&mut ternary.true_branch);
        self.resolve_expr(&mut ternary.false_branch);
    }

    fn visit_variable(&mut self, variable: &mut Variable) -> () {
        if let Some(scope) = self.scopes.last() {
            if let Some(status) = scope.get(&variable.name.lexeme) {
                if !status.defined {
                    error(
                        variable.name.line,
                        "Cannot read local variable in own initializer",
                    )
                }
            }
        }

        self.resolve_local(variable);
    }
}
