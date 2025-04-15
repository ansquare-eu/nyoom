use std::iter::Peekable;

use scan::{scan, Token};


mod scan;
mod parse;

pub fn run(bytes: Vec<u8>) -> Result<(), NyoomError> {
    for s in scan(bytes)? {
        println!("{:?}", s)
    }
    Ok(())
}
#[derive(Debug)]
pub enum NyoomError {
    CompileError(&'static str, usize),
    RuntimeError(&'static str)
}

#[cfg(test)]
mod tests {
}

pub struct TokenIter<U : Iterator<Item = Token>> {
    inner : Peekable<U>
}
impl<U : Iterator<Item = Token>> TokenIter <U> {
    fn next_err(&mut self) -> Result<Token, NyoomError>{
        self.next().ok_or(NyoomError::CompileError("Iterator did not find token", 0))
    }
    pub fn peek(&mut self) -> Option<&Token> {
        self.inner.peek()
    }
}
impl<U: Iterator<Item = Token>> Iterator for TokenIter <U>{
    
    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next()
    }
    type Item = Token;
}

impl<U: Iterator<Item = Token>> From<U> for TokenIter <U> {
    fn from(value: U) -> Self {
        TokenIter { inner: value.peekable() }
    }
}