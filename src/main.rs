extern crate ansi_term;

mod bytecode;
mod compiler;
mod debug;
mod driver;
mod utils;
mod vm;

use crate::driver::{repl, run_file};
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() == 1 {
        repl();
    } else if args.len() == 2 {
        run_file(&args[1]);
    } else {
        println!("Usage: rlox [path]");
    }
}
