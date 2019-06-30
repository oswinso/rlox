use std::fmt;
use std::rc::Rc;

#[derive(Clone, Debug)]
pub enum Obj {
    String(Rc<String>)
}

impl PartialEq for Obj {
    fn eq(&self, other: &Obj) -> bool {
        match (self, other) {
            (Obj::String(l), Obj::String(r)) => l == r
        }
    }
}

impl fmt::Display for Obj {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Obj::String(s) => write!(f, "{}", s),
        }
    }
}