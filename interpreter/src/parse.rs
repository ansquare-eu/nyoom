use crate::{scan::Token, NyoomError, TokenIter};

pub fn parse(tokens: Vec<Token>) -> Result<Vec<Inst>, NyoomError> {
    let mut iter = TokenIter::from(tokens.iter().copied());
    let mut vec : Vec<Inst> = Vec::new();
    while let Some(_) = iter.peek() {
        let inst = read_inst(&mut iter, 0);
        vec.push(inst?);
    }
    Ok(vec)
}
pub enum Id{
    NyNy(u16),
    NyNu(u16)
}
pub enum Expr {
    Var(Id),
    Int(usize),
    CallLoc(FnCall),
    CallBuiltin(FnCall),
    Concat(Vec<Expr>),
    Arg
}
pub struct FnCall(Id, Box<Expr>);
pub struct Block(Vec<Inst>);
pub enum Inst {
    Import(String),
    Alias(Id, Id),
    SetVar(Id, Expr),
    Eval(FnCall),
    FnDef(Id, Block),
    If(Expr, Block),
    While(Expr, Block),
    Return(Expr)
}

fn read_inst<U: Iterator<Item = Token>>(iter: & mut TokenIter<U>, nestedness: u8) -> Result<Inst, NyoomError> {
    match iter.next_err()? {
        Token::Prim => {
            match iter.next_err()? {
                Token::Prim => {
                    match iter.next_err()? {
                        Token::Sec => {
                            match iter.next_err()? {
                                Token::Prim => {
                                    let import = count_prims_and_advance(iter)?;
                                    return if check_if_split_and_advance(iter)? {
                                        Ok(Inst::Import(concat_file_name(import)))
                                    } else {
                                        Err(NyoomError::CompileError("Unexpected end of statement", 0))
                                    }
                                }
                                Token::Sec => {
                                    match iter.next_err()? {
                                        Token::Prim => {
                                            let alias = count_prims_and_advance(iter)?;
                                            match iter.next_err()? {
                                                Token::Sec => {
                                                    let source = count_prims_and_advance(iter)?;
                                                    return if check_if_split_and_advance(iter)? {
                                                         Ok(Inst::Alias(Id::NyNy(alias), Id::NyNy(source)))
                                                    } else {
                                                        Err(NyoomError::CompileError("Unexpected end of statement", 0))
                                                    }
                                                }
                                                _ => return Err(NyoomError::CompileError("Unexpected token", 0))
                                            }
                                        }
                                        _ => return Err(NyoomError::CompileError("Unexpected token", 0))
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        Token::Sec => {

        }
        Token::Split => {

        }
    }
    return Err(NyoomError::CompileError("Not done yet", 0));
}
fn concat_file_name(import: u16) -> String {
    let mut string = String::new();
    for i in 0..import-1 {
        string.push_str("nyoom-");
    }
    string.push_str("nyoom.nyny");
    string
}
fn count_prims_and_advance<U: Iterator<Item = Token>>(iter: & mut TokenIter<U>) -> Result<u16, NyoomError> {
    let mut count = 1_u16;
    while let Some(Token::Prim) = iter.next() {
        if let Some(i) = count.checked_add(1) {
            count = i;
        } else {
            return Err(NyoomError::CompileError("Identifier too long", 0));
        }
    };
    Ok(count)
}
fn check_if_split_and_advance<U: Iterator<Item = Token>>(iter: & mut TokenIter<U>) -> Result<bool, NyoomError> {
    Ok(iter.next_err()? == Token::Split)
}
