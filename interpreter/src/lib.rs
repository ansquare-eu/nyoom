use std::{collections::HashSet, iter::Peekable, path::PathBuf};

use ast::{Id, Inst};
use link::link;
use parse::parse;
use scan::{scan, Token};
use vm::{run_tree_walk, VarContent};

mod ast;
mod link;
mod parse;
mod scan;
mod vm;

pub type AST = Vec<Inst>;
pub fn run(bytes: Vec<u8>, loader_file_path: PathBuf) -> Result<(), NyoomError> {
    let tokens = scan(bytes)?;
    let ast = parse(tokens)?;
    let linked = link(ast, &loader_file_path, &mut HashSet::new())?;
    println!("{:#?}", linked);
    println!("Started runner");
    run_tree_walk(linked)?;
    println!("Ended runner");
    Ok(())
}
#[derive(Debug)]
pub enum NyoomError {
    ScannerError(&'static str, usize, String),
    CompileError(&'static str, usize),
    LinkerError(&'static str, PathBuf),
    BuiltinFnTypeError(&'static str),
    NoSuchElementError(&'static str, i32, usize),
    InvalidArgumentError(&'static str, VarContent),
    RuntimeError(&'static str),
    NoSuchDefinitionError(&'static str, Id),
}

#[cfg(test)]
mod tests {}

pub struct TokenIter<U: Iterator<Item = Token>> {
    inner: Peekable<U>,
}
impl<U: Iterator<Item = Token>> TokenIter<U> {
    fn next_err(&mut self) -> Result<Token, NyoomError> {
        self.next()
            .ok_or(NyoomError::CompileError("Iterator did not find token", 0))
    }
    pub fn peek(&mut self) -> Option<&Token> {
        self.inner.peek()
    }
    pub fn peek_err(&mut self) -> Result<&Token, NyoomError> {
        self.inner
            .peek()
            .ok_or(NyoomError::CompileError("Iterator did not find token", 0))
    }
}
impl<U: Iterator<Item = Token>> Iterator for TokenIter<U> {
    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next()
    }
    type Item = Token;
}

impl<U: Iterator<Item = Token>> From<U> for TokenIter<U> {
    fn from(value: U) -> Self {
        TokenIter {
            inner: value.peekable(),
        }
    }
}
