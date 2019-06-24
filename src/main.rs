extern crate ansi_term;

mod bytecode;
mod debug;
mod utils;
mod vm;

use crate::debug::Disassembler;
use bytecode::{Chunk, Opcode};

use crate::vm::VM;

fn main() {
    let mut chunk = Chunk::new();

    let constant = chunk.add_constant(1.2);
    chunk.write(Opcode::Push, 123);
    chunk.write(constant, 123);

    let constant = chunk.add_constant(3.4);
    chunk.write(Opcode::Push, 123);
    chunk.write(constant, 123);

    chunk.write(Opcode::Add, 123);

    let constant = chunk.add_constant(5.6);
    chunk.write(Opcode::Push, 123);
    chunk.write(constant, 123);

    chunk.write(Opcode::Div, 123);

    chunk.write(Opcode::Neg, 123);

    chunk.write(Opcode::Ret, 123);

    //    let mut disassembler = Disassembler::new();
    //    disassembler.disassemble_chunk(&chunk, "test chunk");
    //    println!("{}", disassembler.result());

    let mut vm = VM::new(&chunk);
    let result = vm.interpret();
}
