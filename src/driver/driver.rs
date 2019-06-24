use crate::compiler::compile;
use crate::utils::PrettyPrinter;
use crate::vm::InterpretResult;
use std::fs::File;
use std::io;
use std::io::Read;

pub fn repl() {
    let mut input = String::new();
    let mut pretty_printer = PrettyPrinter::new(String::new());

    loop {
        pretty_printer.prompt().print();
        io::stdin().read_line(&mut input).unwrap();
        input.pop();

        interpret(&input);
    }
}

pub fn run_file(path: &str) {
    let mut s = String::new();
    File::open(&path).unwrap().read_to_string(&mut s).unwrap();

    let result = interpret(&s);
}

pub fn interpret(source: &str) -> InterpretResult {
    compile(source);
    Ok(())
}
