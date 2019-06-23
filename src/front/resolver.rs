use crate::front::expr::{self, Assign, Binary, Call, Expr, Get, Grouping, Literal, Set, Ternary, This, Unary, Variable, Super};
use crate::front::stmt::{
    self, Block, ClassDecl, Declaration, FunctionDecl, If, Return, Stmt, While,
};
use crate::front::token::Token;
use crate::{error, warn};
use core::borrow::BorrowMut;
use std::collections::HashMap;
use crate::front::callables::ExternalFunctions;
use crate::front::token_type::TokenType;

#[derive(PartialEq)]
enum FunctionType {
    None,
    Function,
    Initializer,
    Method,
}

#[derive(PartialEq)]
enum ClassType {
    None,
    Class,
    Subclass,
}

#[derive(Debug)]
struct VariableStatus {
    declaration_token: Token,
    defined: bool,
    used: bool,
}

impl VariableStatus {
    pub fn new(declaration_token: Token) -> Self {
        VariableStatus {
            declaration_token,
            defined: false,
            used: false,
        }
    }
}

pub struct Resolver {
    scopes: Vec<HashMap<String, VariableStatus>>,
    current_function: FunctionType,
    current_class: ClassType,
}

impl Resolver {
    pub fn new() -> Resolver {
        Resolver {
            scopes: Vec::new(),
            current_function: FunctionType::None,
            current_class: ClassType::None,
        }
    }

    pub fn resolve(&mut self, statements: &mut [Stmt]) {
        self.begin_scope();
        self.setup_external_functions();
        self.resolve_stmts(statements);
        self.end_scope();
    }

    pub fn setup_external_functions(&mut self) {
        for callable in ExternalFunctions::get() {
            let mut variable_status = VariableStatus::new(Token {
                token_type: TokenType::Identifier(callable.name().into()),
                lexeme: callable.name().into(),
                line: 0
            });
            variable_status.defined = true;
            variable_status.used = true;
            self.scopes.last_mut().unwrap().insert(callable.name().into(), variable_status);
        }
    }

    fn resolve_stmt(&mut self, statement: &mut Stmt) {
        statement.accept_mutable(self);
    }

    fn resolve_stmts(&mut self, statements: &mut [Stmt]) {
        for statement in statements {
            self.resolve_stmt(statement);
        }
    }

    fn resolve_expr(&mut self, expr: &mut Expr) {
        expr.accept_mutable(self);
    }

    fn resolve_local(&mut self, variable: &mut Variable) {
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

    fn resolve_function(
        &mut self,
        function_decl: &mut FunctionDecl,
        mut function_type: FunctionType,
    ) {
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

    fn begin_scope(&mut self) -> &HashMap<String, VariableStatus> {
        self.scopes.push(HashMap::new());
        self.scopes.last_mut().unwrap()
    }

    fn end_scope(&mut self) {
        if let Some(scope) = self.scopes.pop() {
            for (_, entry) in scope.into_iter() {
                if !entry.used {
                    warn(
                        entry.declaration_token.line,
                        &format!(
                            "Variable {} is declared but never used.",
                            entry.declaration_token.lexeme
                        ),
                    )
                }
            }
        }
    }

    fn declare(&mut self, token: &Token) {
        if let Some(scope) = self.scopes.last_mut() {
            if scope.contains_key(&token.lexeme) {
                error(
                    token.line,
                    &format!(
                        "Variable with name {} is already declared in this scope.",
                        token.lexeme
                    ),
                );
            } else {
                scope.insert(token.lexeme.clone(), VariableStatus::new(token.clone()));
            }
        }
    }

    fn define(&mut self, token: &Token) {
        if let Some(last) = self.scopes.last_mut() {
            if let Some(scope) = last.get_mut(&token.lexeme) {
                scope.defined = true;
            } else {
                panic!(
                    "Uh.... {} doesn't live in this scope?\n[line {}]",
                    token.lexeme, token.line
                )
            }
        }
    }
}

impl<'a> stmt::MutableVisitor<'a, ()> for Resolver {
    fn visit_block(&mut self, block: &mut Block) -> () {
        self.begin_scope();
        self.resolve_stmts(&mut block.statements);
        self.end_scope();
    }

    fn visit_class(&mut self, class: &mut ClassDecl) -> () {
        let enclosing_class = std::mem::replace(&mut self.current_class, ClassType::Class);
        self.declare(&class.name);
        self.define(&class.name);

        if let Some(superclass) = &mut class.superclass {
            if superclass.name.lexeme == class.name.lexeme {
                error(superclass.name.line, "A class cannot inherit from itself")
            } else {
                self.current_class = ClassType::Subclass;
                self.resolve_local(superclass);
                self.begin_scope();
                let mut variable_status = VariableStatus::new(superclass.name.clone());
                variable_status.defined = true;
                variable_status.used = true;
                self.scopes.last_mut().unwrap().insert("super".into(), variable_status);
            }
        }

        self.begin_scope();
        let mut self_variable = VariableStatus::new(*class.name.clone());
        self_variable.defined = true;
        self_variable.used = true;
        self.scopes
            .last_mut()
            .unwrap()
            .insert("this".into(), self_variable);

        for method in &mut class.methods {
            let function_type = if method.name.lexeme == class.name.lexeme {
                FunctionType::Initializer
            } else {
                FunctionType::Method
            };

            self.resolve_function(method, function_type);
        }
        self.end_scope();

        if let Some(superclass) = &mut class.superclass {
            self.end_scope();
        }

        std::mem::replace(&mut self.current_class, enclosing_class);
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
            if self.current_function == FunctionType::Initializer {
                error(ret.keyword.line, "Cannot return values from an initializer");
            }
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

    fn visit_get(&mut self, get: &mut Get) -> () {
        self.resolve_expr(get.object.borrow_mut());
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

    fn visit_set(&mut self, set: &mut Set) -> () {
        self.resolve_expr(set.object.borrow_mut());
        self.resolve_expr(set.value.borrow_mut());
    }

    fn visit_super(&mut self, super_expr: &'a mut Super) -> () {
        match self.current_class {
            ClassType::None => error(super_expr.keyword.name.line, "Cannot use 'super' outside of a class"),
            ClassType::Class => error(super_expr.keyword.name.line, "Cannot use 'super' in a class with no subclass"),
            ClassType::Subclass => ()
        }
        self.resolve_local(&mut super_expr.keyword)
    }

    fn visit_ternary(&mut self, ternary: &mut Ternary) -> () {
        self.resolve_expr(&mut ternary.condition);
        self.resolve_expr(&mut ternary.true_branch);
        self.resolve_expr(&mut ternary.false_branch);
    }

    fn visit_this(&mut self, this: &mut This) -> () {
        if self.current_class == ClassType::None {
            error(
                this.variable.name.line,
                "Cannot use 'this' keyword outside of a class.",
            )
        } else {
            self.resolve_local(&mut this.variable);
        }
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
