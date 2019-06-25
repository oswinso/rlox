use std::convert::TryFrom;

pub type Value = f64;

pub(crate) type ConstantPointer = u8;

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
