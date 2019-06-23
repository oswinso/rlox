use crate::front::callables::{Clock, Callable};

pub struct ExternalFunctions { }

impl ExternalFunctions {
    pub fn get() -> Vec<Box<dyn Callable>> {
        vec![Clock::new()]
    }
}

