use crate::bytecode::{Chunk, Opcode, Value};
use crate::vm::errors::*;

use crate::debug::Disassembler;
use crate::vm::Stack;
use std::convert::TryInto;
use std::slice::Iter;

type InterpretResult = Result<(), VMError>;

pub struct VM<'chunk> {
    chunk: &'chunk Chunk,
    ip: Iter<'chunk, u8>,
    stack: Stack,

    #[cfg(feature = "trace_execution")]
    disassembler: Disassembler,
    #[cfg(feature = "trace_execution")]
    offset: usize,
}

impl<'chunk> VM<'chunk> {
    pub fn new(chunk: &'chunk Chunk) -> VM<'chunk> {
        VM {
            chunk,
            ip: chunk.code.iter(),
            stack: Stack::new(),
            #[cfg(feature = "trace_execution")]
            disassembler: Disassembler::new(),
            #[cfg(feature = "trace_execution")]
            offset: 0,
        }
    }

    pub fn interpret(&mut self) -> InterpretResult {
        use Opcode::*;

        loop {
            #[cfg(feature = "trace_execution")]
            {
                self.disassembler.print_stack(&self.stack);
                self.disassembler
                    .disassemble_instruction(self.chunk, self.offset);
                println!("{}", self.disassembler.result());
                self.disassembler.clear();
            }
            if let Some(&instruction) = self.read_byte() {
                if let Ok(opcode) = instruction.try_into() {
                    match opcode {
                        Ret => {
                            let ret = self.stack.pop().unwrap_or(0.0);
                            #[cfg(feature = "trace_execution")]
                            {
                                self.disassembler.print_value(&ret);
                                println!("{}", self.disassembler.result());
                                self.disassembler.clear();
                            }
                            return Ok(());
                        }
                        Push => {
                            let constant = self.read_constant();
                            self.stack.push(constant);
                        }
                        Neg => {
                            let val = -self.stack.pop().unwrap();
                            self.stack.push(val);
                        }
                        Add => self.binary_op(|left, right| left + right),
                        Sub => self.binary_op(|left, right| left - right),
                        Mul => self.binary_op(|left, right| left * right),
                        Div => self.binary_op(|left, right| left / right),
                    }
                } else {
                    panic!("Couldn't decode opcode {}", instruction);
                }
            } else {
                return Ok(());
            }
        }
    }

    fn binary_op<F>(&mut self, f: F)
    where
        F: FnOnce(Value, Value) -> Value,
    {
        let right = self.stack.pop().unwrap();
        let left = self.stack.pop().unwrap();
        self.stack.push(f(left, right));
    }

    fn read_constant(&mut self) -> Value {
        let offset: usize = *self.read_byte().unwrap() as usize;
        self.chunk.constants.values[offset]
    }

    fn read_byte(&mut self) -> Option<&u8> {
        #[cfg(feature = "trace_execution")]
        {
            self.offset += 1;
        }
        self.ip.next()
    }
}
