use crate::bytecode::Value;
use crate::compiler::{Token, TokenKind, Source};
use std::collections::HashMap;

pub type GlobalMap = HashMap<String, Value>;

#[derive(Clone)]
pub struct Local {
    name: Token,
    depth: usize,
}

impl Local {
    pub fn new(name: Token, depth: usize) -> Local {
        Local { name, depth }
    }
}

pub struct LocalMap {
    locals: Vec<Local>,
    scope_depth: usize,
}

impl LocalMap {
    pub fn new() -> LocalMap {
        LocalMap {
            locals: Vec::new(),
            scope_depth: 0,
        }
    }
}

impl LocalMap {
    pub fn begin_scope(&mut self) {
        self.scope_depth += 1;
    }

    pub fn end_scope(&mut self) -> u8 {
        self.scope_depth -= 1;

        let mut num_pops = 0;
        while !self.locals.is_empty() && self.locals.last().unwrap().depth > self.scope_depth {
            self.locals.pop();
            num_pops += 1;
        }
        num_pops
    }

    pub fn in_scope(&self) -> bool {
        self.scope_depth > 0
    }

    pub fn add(&mut self, name: Token, source: &Source) -> Result<(),()> {
        if self.locals.len() >= std::u8::MAX as usize {
            eprintln!("Too many local variables in current function");
            return Err(());
        }

        for local in &self.locals {
            if local.depth == self.scope_depth && source.get_lexeme(&local.name) == source.get_lexeme(&name) {
                eprintln!("Variable with this name already exists in this scope.");
                return Err(());
            }
        }
        self.locals.push(Local::new(name, std::usize::MAX));
        Ok(())
    }

    pub fn mark_initialized(&mut self) {
        if let Some(last) = self.locals.last_mut() {
            last.depth = self.scope_depth
        }
    }

    pub fn resolve(&self, name: &str, source: &Source) -> Option<u8> {
        for (index, local) in self.locals.iter().rev().enumerate() {
            if source.get_lexeme(&local.name) == name {
                if local.depth == std::usize::MAX {
                    eprintln!("Cannot read variable in own initializer.");
                }
                return Some(index as u8);
            }
        }
        None
    }
}
