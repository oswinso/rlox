use crate::bytecode::{Chunk, Opcode, Value};
use crate::vm::errors::*;

use crate::vm::Stack;
use std::convert::TryInto;
use std::iter::Enumerate;
use std::slice::Iter;

#[cfg(feature = "trace_execution")]
use crate::debug::Disassembler;

pub type VMResult = Result<(), RuntimeError>;

pub struct VM<'chunk> {
    chunk: &'chunk Chunk,
    ip: Enumerate<Iter<'chunk, u8>>,
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
            ip: chunk.code.iter().enumerate(),
            stack: Stack::new(),
            #[cfg(feature = "trace_execution")]
            disassembler: Disassembler::new(),
            #[cfg(feature = "trace_execution")]
            offset: 0,
        }
    }

    pub fn interpret(&mut self) -> VMResult {
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
            if let Some((line, &instruction)) = self.read_byte() {
                if let Ok(opcode) = instruction.try_into() {
                    match opcode {
                        Ret => {
                            let ret = self.stack.pop().unwrap_or(Value::Nil);
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
                            let val: f64 = {
                                match self.stack.last().unwrap() {
                                    Value::Number(_) => {
                                        self.stack.pop().unwrap().try_into().unwrap()
                                    }
                                    _ => {
                                        return Err(RuntimeError::new(
                                            line,
                                            "Operand must be a number",
                                        ))
                                    }
                                }
                            };
                            self.stack.push(Value::Number(val));
                        }
                        Add => self.binary_op(|left, right| Value::Number(left + right))?,
                        Sub => self.binary_op(|left, right| Value::Number(left - right))?,
                        Mul => self.binary_op(|left, right| Value::Number(left * right))?,
                        Div => self.binary_op(|left, right| Value::Number(left / right))?,
                        True => self.stack.push(Value::Bool(true)),
                        False => self.stack.push(Value::Bool(false)),
                        Nil => self.stack.push(Value::Nil),
                        Not => {
                            let is_falsey = VM::is_falsey(&self.stack.pop().unwrap());
                            self.stack.push(Value::Bool(is_falsey))
                        }
                        Eq => {
                            let a = self.stack.pop().unwrap();
                            let b = self.stack.pop().unwrap();
                            self.stack.push(Value::Bool(VM::values_equal(&a, &b)))
                        }
                        Gt => self.binary_op(|left, right|Value::Bool(left > right))?,
                        Lt => self.binary_op(|left, right|Value::Bool(left < right))?,
                    }
                } else {
                    panic!("Couldn't decode opcode {}", instruction);
                }
            } else {
                return Ok(());
            }
        }
    }

    fn binary_op<F>(&mut self, f: F) -> VMResult
    where
        F: FnOnce(f64, f64) -> Value,
    {
        match (self.stack.pop(), self.stack.pop()) {
            (Some(Value::Number(left)), Some(Value::Number(right))) => {
                self.stack.push(f(left, right));
                Ok(())
            }
            (Some(_), Some(_)) => {
                return Err(RuntimeError::new(0, "Expected two numbers on the stack"))
            }
            (None, _) | (_, None) => {
                return Err(RuntimeError::new(
                    0,
                    "Expected at least two items on the stack",
                ))
            }
        }
    }

    fn read_constant(&mut self) -> Value {
        let (line, byte) = self.read_byte().unwrap();
        let offset = *byte as usize;
        self.chunk.constants.values[offset]
    }

    fn read_byte(&mut self) -> Option<(usize, &u8)> {
        #[cfg(feature = "trace_execution")]
        {
            self.offset += 1;
        }
        self.ip.next()
    }

    fn is_falsey(value: &Value) -> bool {
        match value {
            Value::Nil => true,
            Value::Bool(b) => !b,
            Value::Number(n) => *n == 0.0,
        }
    }

    fn values_equal(left: &Value, right: &Value) -> bool {
        match (left, right) {
            (Value::Number(l), Value::Number(r)) => l == r,
            (Value::Bool(l), Value::Bool(r)) => l == r,
            (Value::Nil, Value::Nil) => true,
            _ => false
        }
    }
}
