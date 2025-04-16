use std::{collections::HashSet, iter::Peekable, path::PathBuf};

use link::link;
use parse::parse;
use scan::{scan, Token};

mod parse;
mod scan;
mod link;
mod vm;

pub type AST = Vec<parse::Inst>;
pub fn run(bytes: Vec<u8>, loader_file_path: PathBuf) -> Result<(), NyoomError> {
    let tokens = scan(bytes)?;
    let ast = parse(tokens)?;
    let linked = link(ast, &loader_file_path, &mut HashSet::new())?;
    println!("{:#?}", linked);
    Ok(())
}
#[derive(Debug)]
pub enum NyoomError {
    ScannerError(&'static str, usize,  String),
    CompileError(&'static str, usize),
    LinkerError(&'static str, PathBuf),
    RuntimeError(&'static str),
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
        self.inner.peek().ok_or(NyoomError::CompileError("Iterator did not find token", 0))
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
