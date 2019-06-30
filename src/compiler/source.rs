use crate::compiler::Token;

#[derive(Copy, Clone)]
pub struct Source<'src> {
    pub source: &'src str,
}

impl<'src> Source<'src> {
    pub fn new(source: &'src str) -> Source<'src> {
        Source { source }
    }

    pub fn get_lexeme(&self, token: &Token) -> &str {
        std::str::from_utf8(&self.source.as_bytes()[token.position.start..token.position.end])
            .unwrap()
    }

    pub fn get_string(&self, token: &Token) -> &str {
        std::str::from_utf8(&self.source.as_bytes()[token.position.start + 1..token.position.end - 1])
        .unwrap()
    }
}
