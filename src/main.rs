#![feature(duration_float)]
use std::env;
use std::fs::File;
use std::io::{self, Read, Write};

mod front;

use crate::front::ast_printer::AstPrinter;
use crate::front::errors::RuntimeError;
use crate::front::expr::*;
use crate::front::interpreter::Interpreter;
use crate::front::parser::Parser;
use crate::front::scanner::Scanner;
use crate::front::stmt::Stmt;
use crate::front::token::Token;
use crate::front::token_type::TokenType;

static mut HAD_ERROR: bool = false;
static mut HAD_RUNTIME_ERROR: bool = false;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() > 2 {
        println!("Usage: rlox [script]");
    } else if args.len() == 2 {
        run_file(&args[1]);
    } else {
        run_prompt();
    }
    //    test_ast();
}

fn test_ast() {
    let expr = Expr::Binary(Binary {
        left: Box::new(Expr::Unary(Unary {
            operator: Box::new(Token {
                token_type: TokenType::Minus,
                lexeme: "-".to_string(),
                line: 1,
            }),
            right: Box::new(Expr::Literal(Literal::Number(123.0))),
        })),
        operator: Box::new(Token {
            token_type: TokenType::Star,
            lexeme: "*".to_string(),
            line: 1,
        }),
        right: Box::new(Expr::Grouping(Grouping {
            expression: Box::new(Expr::Literal(Literal::Number(45.67))),
        })),
    });
    println!("{}", AstPrinter {}.print(&expr));
}

fn run_file(path: &str) {
    let mut file = File::open(&path).unwrap();

    let mut interpreter = Interpreter::new();

    let mut s = String::new();
    file.read_to_string(&mut s).unwrap();

    run(&s, &mut interpreter);

    if unsafe { HAD_ERROR } {
        std::process::exit(65);
    }
    if unsafe { HAD_RUNTIME_ERROR } {
        std::process::exit(70);
    }
}

fn run_prompt() {
    let mut input = String::new();
    let mut interpreter = Interpreter::new();

    loop {
        input.clear();
        print!("> ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut input).unwrap();
        input.pop();
        run(&input, &mut interpreter);

        unsafe { HAD_ERROR = false };
    }
}

fn run(source: &str, interpreter: &mut Interpreter) {
    let mut scanner = Scanner::new(source);
    let tokens = scanner.scan_tokens();

    //    println!("{:?}", tokens);

    let mut parser = Parser::new(tokens);
    let stmts = parser.parse();

    //    for stmt in &stmts {
    //        let exprs: Vec<&Expr> = match stmt {
    //            Stmt::Expression(expr) => vec![expr],
    //            Stmt::Print(expr) => vec![expr],
    //            Stmt::Block(block) => vec![],
    //            Stmt::Declaration(decl) => decl.initializer.iter().collect()
    //        };
    //        for expr in exprs {
    //            println!("{}", AstPrinter::new().print(expr));
    //        }
    //    }

    interpreter.interpret(&stmts);
}

fn error(line: usize, message: &str) {
    report(line, "", message);
}

fn report(line: usize, context: &str, message: &str) {
    eprintln!("[line {}] Error{}: {}", line, context, message);
    unsafe { HAD_ERROR = true };
}

fn runtime_error<T: ?Sized>(rte: Box<T>)
where
    T: RuntimeError,
{
    eprintln!("Runtime Error. {}", rte);
    unsafe { HAD_RUNTIME_ERROR = true };
}
