use crate::compiler::{compile, CompileError, Source};
use crate::utils::PrettyPrinter;
use crate::vm::{RuntimeError, VM};
use std::fs::File;
use std::io;
use std::io::Read;

pub enum InterpretError {
    CompileError(CompileError),
    RuntimeError(RuntimeError),
}

pub type InterpretResult = Result<(), InterpretError>;

pub fn repl() {
    let mut input = String::new();
    let mut pretty_printer = PrettyPrinter::new(String::new());

    loop {
        input.clear();
        pretty_printer.prompt().print();
        io::stdin().read_line(&mut input).unwrap();
        input.pop();
        if input.len() > 0 {
            interpret(&input)
                .err()
                .map(|err| pretty_printer.interpret_error(err).newline().print());
        } else {
            println!();
        }
    }
}

pub fn run_file(path: &str) {
    let mut s = String::new();
    File::open(&path).unwrap().read_to_string(&mut s).unwrap();

    let result = interpret(&s);
}

pub fn interpret(src: &str) -> InterpretResult {
    use InterpretError::*;

    let source = Source::new(src);

    let (chunk, strings) = match compile(source) {
        Ok(res) => res,
        Err(err) => return Err(CompileError(err)),
    };

    let mut vm = VM::new(&chunk, strings);
    match vm.interpret() {
        Ok(_) => (),
        Err(err) => return Err(RuntimeError(err)),
    };
    Ok(())
}
