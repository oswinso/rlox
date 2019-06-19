use crate::front::expr::Value;

#[derive(Debug, Clone)]
pub struct ReturnObject {
    pub value: Value,
}

impl ReturnObject {
    pub fn new(value: Value) -> Self {
        ReturnObject { value }
    }
}
