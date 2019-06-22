use crate::front::callables::Class;

use std::fmt;

#[derive(Clone)]
pub struct Instance {
    class: Class,
}

impl Instance {
    pub fn new(class: Class) -> Instance {
        Instance {
            class
        }
    }
}

impl fmt::Display for Instance {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} instance", self.class)
    }
}

impl fmt::Debug for Instance {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} instance", self.class)
    }
}

impl PartialEq for Instance {
    fn eq(&self, other: &Instance) -> bool {
        false
    }
}
