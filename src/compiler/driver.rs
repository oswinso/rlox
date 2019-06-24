use crate::compiler::{CompileResult, Compiler, Source};

pub fn compile(src: Source) -> CompileResult {
    let mut compiler = Compiler::new(src);
    compiler.compile()
}
