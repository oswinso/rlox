pub struct RuntimeError {
    pub line: usize,
    pub message: String,
}

impl RuntimeError {
    pub fn new(line: usize, message: &str) -> RuntimeError {
        RuntimeError {
            line,
            message: "".to_string(),
        }
    }
}
