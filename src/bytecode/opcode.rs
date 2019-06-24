use std::convert::TryFrom;
use std::fmt;

#[derive(Copy, Clone, PartialEq, Eq)]
#[repr(u8)]
pub enum Opcode {
    Ret,
    Push,
    Neg,
    Add,
    Sub,
    Mul,
    Div,
}

impl fmt::Display for Opcode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use Opcode::*;
        let string = match self {
            Ret => "RET",
            Push => "PUSH",
            Neg => "NEG",
            Add => "ADD",
            Sub => "SUB",
            Mul => "MUL",
            Div => "DIV",
        };
        write!(f, "{}", string)
    }
}

impl From<Opcode> for u8 {
    fn from(opcode: Opcode) -> Self {
        opcode as u8
    }
}

impl TryFrom<u8> for Opcode {
    type Error = ();

    fn try_from(byte: u8) -> Result<Self, Self::Error> {
        use Opcode::*;

        match byte {
            0 => Ok(Ret),
            1 => Ok(Push),
            2 => Ok(Neg),
            3 => Ok(Add),
            4 => Ok(Sub),
            5 => Ok(Mul),
            6 => Ok(Div),
            _ => Err(()),
        }
    }
}
