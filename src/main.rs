use std::env;
use std::error::Error;
use std::fs::File;
use std::io::{self, Read, Write};
use std::path::Path;

mod front;

use crate::front::ast_printer::AstPrinter;
use crate::front::expr::*;
use crate::front::scanner::Scanner;
use crate::front::token::Token;
use crate::front::token_type::TokenType;
use crate::front::parser::Parser;

static mut HAD_ERROR: bool = false;

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

    let mut s = String::new();
    file.read_to_string(&mut s).unwrap();
    run(&s);

    if unsafe { HAD_ERROR } {
        std::process::exit(65);
    }
}

fn run_prompt() {
    let mut input = String::new();

    loop {
        input.clear();
        print!("> ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut input).unwrap();
        input.pop();
        run(&input);

        unsafe { HAD_ERROR = false };
    }
}

fn run(source: &str) {
    let mut scanner = Scanner::new(source);
    let tokens = scanner.scan_tokens();

    println!("{:?}", tokens);
    
    let mut parser = Parser::new(tokens);
    let expr = parser.parse().unwrap();

    println!("{}", AstPrinter::new().print(&expr));
}

fn error(line: usize, message: &str) {
    report(line, "", message);
}

fn report(line: usize, context: &str, message: &str) {
    eprintln!("[line {}] Error{}: {}", line, context, message);
    unsafe { HAD_ERROR = true };
}
