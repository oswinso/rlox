use std::convert::TryFrom;

use std::fmt;

pub(crate) type ConstantPointer = u8;

#[derive(Copy, Clone, Debug)]
pub enum Value {
    Nil,
    Bool(bool),
    Number(f64),
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Value::Nil => write!(f, "Nil"),
            Value::Bool(b) => write!(f, "{}", b),
            Value::Number(num) => write!(f, "{}", num),
        }
    }
}

impl TryFrom<Value> for f64 {
    type Error = ();

    fn try_from(value: Value) -> Result<Self, Self::Error> {
        match value {
            Value::Number(num) => Ok(num),
            _ => Err(()),
        }
    }
}

pub struct ValueArray {
    pub values: Vec<Value>,
}

impl ValueArray {
    pub fn new() -> ValueArray {
        ValueArray { values: Vec::new() }
    }

    pub fn write(&mut self, value: Value) -> Result<ConstantPointer, ()> {
        match u8::try_from(self.values.len()) {
            Ok(index) => {
                self.values.push(value);
                Ok(index)
            }
            Err(_) => Err(()),
        }
    }
}
