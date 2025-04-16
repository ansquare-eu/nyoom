use crate::{scan::Token, NyoomError, TokenIter, AST};

pub fn parse(tokens: Vec<Token>) -> Result<AST, NyoomError> {
    let mut iter = TokenIter::from(tokens.iter().copied());
    let mut vec = Vec::new();
    while let Some(_) = iter.peek() {
        let inst = parse_inst(&mut iter, false);
        if let Err(e) = inst {
            println!("It returned an error it did not finish: {e:?}");
            return Ok(vec);
        }
        vec.push(inst?);
    }
    Ok(vec)
}
#[derive(Debug)]
pub enum Id {
    NyNy(u16),
    NyNu(u16),
}
#[derive(Debug)]
pub enum Expr {
    Var(Id),
    Int(u32),
    CallLoc(FnCall),
    CallBuiltin(FnCall),
    Concat(Vec<Expr>),
    Arg,
}
#[derive(Debug)]
pub struct FnCall(Id, Box<Expr>);
#[derive(Debug)]
pub struct Block(Vec<Inst>);
#[derive(Debug)]
pub enum Inst {
    Import(String),
    Alias(Id, Id),
    SetVar(Id, Expr),
    Eval(Expr),
    FnDef(Id, Block),
    If(Expr, Block, Option<Block>),
    While(Expr, Block),
    Return(Expr),
}

fn parse_inst<U: Iterator<Item = Token>>(
    iter: &mut TokenIter<U>,
    is_fn: bool,
) -> Result<Inst, NyoomError> {
    match iter.next_err()? {
        Token::Prim => match iter.next_err()? {
            Token::Prim => match iter.next_err()? {
                Token::Sec => match iter.next_err()? {
                    //Import
                    Token::Prim => {
                        let import = count_prims_and_advance(iter)?;
                        return if check_if_split_and_advance(iter)? {
                            Ok(Inst::Import(concat_file_name(import)))
                        } else {
                            Err(NyoomError::CompileError("Unexpected end of import statement", 0))
                        };
                    }
                    //Alias
                    Token::Sec => match iter.next_err()? {
                        Token::Prim => {
                            let alias = count_prims_and_advance(iter)?;
                            match iter.next_err()? {
                                Token::Sec => {
                                    let source = count_prims_and_advance(iter)?;
                                    return if check_if_split_and_advance(iter)? {
                                        Ok(Inst::Alias(Id::NyNy(alias), Id::NyNy(source)))
                                    } else {
                                        Err(NyoomError::CompileError(
                                            "Unexpected end of alias statement",
                                            0,
                                        ))
                                    };
                                }
                                _ => return Err(NyoomError::CompileError("An alias statement needs a second name", 0)),
                            }
                        }
                        _ => return Err(NyoomError::CompileError("An alias statement needs a first name", 0)),
                    },
                    Token::Split => return Err(NyoomError::CompileError("Nyoom nyoom nugget alone is insufficient, did you mean to write an import?", 0)),
                },
                //If and while
                Token::Prim => {
                    let count = count_prims_and_advance(iter)?;
                    match iter.next_err()? {
                        Token::Sec => {
                            let expr = parse_expression_until_split(iter)?;
                            let block = parse_opened_block(iter)?;
                            //TODO Also add else block if i find it important enough
                            return match count {
                                1 => Ok(Inst::If(expr, block, None)),
                                2 => Ok(Inst::While(expr, block)),
                                _ => Err(NyoomError::CompileError("Unexpected strucure following a control flow declaration", 0))
                            }
                        },
                        _ => return Err(NyoomError::CompileError("Expression expected in control flow structure", 0))
                    }
                },
                Token::Split => return Err(NyoomError::CompileError("Nyoom nyoom alone is insufficient", 0)),
            },
            //Returns, evals and function defs
            Token::Sec => {
                if *iter.peek_err()? == Token::Sec {
                    iter.next();
                    let expr = parse_expression_until_split(iter)?;
                    return Ok(Inst::Eval(expr));
                }
                //As Nyoom does not support nested functions, returns and evals can have the same beginning syntax
                //Returs
                return if is_fn {
                    let expr = parse_expression_until_split(iter)?;
                    Ok(Inst::Return(expr))
                } // Function defs
                 else {
                    if iter.next_err()? == Token::Prim {
                        let count = count_prims_and_advance(iter)?;
                        if iter.next_err()? == Token::Split {
                            let block = parse_opened_block(iter)?;
                            Ok(Inst::FnDef(Id::NyNy(count), block))
                        } else {
                            Err(NyoomError::CompileError("Expected split to start fn block", 0))
                        }
                    } else {
                        Err(NyoomError::CompileError("Expected function name after function declaration", 0))
                    }
                };
            },
            Token::Split => return Err(NyoomError::CompileError("Nyoom alone is not enough", 0)),
        },
        //Variable declarations and mutations (it's the same)
        Token::Sec => {
            match iter.next_err()? {
                Token::Prim => {
                    let count = count_prims_and_advance(iter)?;
                    if iter.next_err()? == Token::Sec {
                        let expr = parse_expression_until_split(iter)?;
                        return Ok(Inst::SetVar(Id::NyNy(count), expr));
                    } else {
                        return Err(NyoomError::CompileError("Variable declared without value! This is not java you dont do this ever", 0));
                    }
                }
                _ => return Err(NyoomError::CompileError("Unexpected token in variable declaration", 0))
            }
        }
        Token::Split => return Err(NyoomError::CompileError("Nothing to split", 0))
    }
}
//Parses a block until it finds a block ending split, assumes block starting split already found
fn parse_opened_block<U: Iterator<Item = Token>>(
    iter: &mut TokenIter<U>,
) -> Result<Block, NyoomError> {
    let mut vec: Vec<Inst> = Vec::new();
    loop {
        if *iter.peek_err()? == Token::Split {
            iter.next();
            break;
        }
        let inst = parse_inst(iter, true)?;
        vec.push(inst);
        
    }
    Ok(Block(vec))
}
//Parses an expression, assuming it ends with a split
//Assumes the first sec was already found
fn parse_expression_until<U: Iterator<Item = Token>>(
    iter: &mut TokenIter<U>,
) -> Result<(Expr, Token), NyoomError> {
    let count = count_secs_and_advance(iter)?;
    match count {
        //Vars
        1 => {
            if iter.next_err()? == Token::Prim {
                let name = count_prims_and_advance(iter)?;
                Ok((Expr::Var(Id::NyNy(name)), iter.next_err()?))
            } else {
                Err(NyoomError::CompileError("Unexpected token instead of var name", 0))
            }
        },
        //Literals
        2 => {
            if iter.next_err()? == Token::Prim {
                let literal = count_prims_and_advance_for_literal(iter)?;
                Ok((Expr::Int(literal), iter.next_err()?))
            } else {
                Err(NyoomError::CompileError("Unexpected token instead of literal", 0))
            }
        },
        //Local fns
        3 => {
            if iter.next_err()? == Token::Prim {
                let name = count_prims_and_advance(iter)?;
                if iter.next_err()? == Token::Sec {
                    let expr = parse_expression_until(iter)?;
                    Ok((Expr::CallLoc(FnCall(Id::NyNy(name), Box::new(expr.0))), expr.1))
                } else {
                    Err(NyoomError::CompileError("Unexpected token following local fn name, what are you splitting?", 0))
                }
            } else {
                Err(NyoomError::CompileError("Unexpected token instead of local function", 0))
            }
        },
        //Builtin fns
        4 => {
            if iter.next_err()? == Token::Prim {
                let name = count_prims_and_advance(iter)?;
                if iter.next_err()? == Token::Sec {
                    let expr = parse_expression_until(iter)?;
                    Ok((Expr::CallBuiltin(FnCall(Id::NyNy(name), Box::new(expr.0))), expr.1))
                } else {
                    Err(NyoomError::CompileError("Unexpected token following global fn name, what are you splitting?", 0))
                }
            } else {
                Err(NyoomError::CompileError("Unexpected token instead of global function", 0))
            }
        },
        //Concatenation
        5 => {
            let mut vec : Vec<Expr> = Vec::new();
            loop {
                let expr = parse_expression_until(iter)?;
                vec.push(expr.0);
                if expr.1 == Token::Split {
                    break;
                }
            };
            Ok((Expr::Concat(vec), Token::Split))
        },
        //Arg reference
        6 => {
            if iter.next_err()? == Token::Prim {
                let next = iter.next_err()?;
                if next != Token::Prim {
                    Ok((Expr::Arg, next))
                } else {
                    Err(NyoomError::CompileError("A Prim token found in arg reference, this means it is misconstructed", 0))
                }
            } else {
                Ok((Expr::Arg, Token::Split))
            }
        },
        _ => Err(NyoomError::CompileError("Invalid expression type", 0))
    }
}
fn parse_expression_until_split<U: Iterator<Item = Token>>(
    iter: &mut TokenIter<U>,
) -> Result<Expr, NyoomError> {
    parse_expression_until(iter).map(|x|x.0)
}
fn concat_file_name(import: u16) -> String {
    let mut string = String::new();
    for _ in 0..import - 1 {
        string.push_str("nyoom-");
    }
    string.push_str("nyoom.nyny");
    string
}
fn count_prims_and_advance_for_literal<U: Iterator<Item = Token>>(
    iter: &mut TokenIter<U>,
) -> Result<u32, NyoomError> {
    let mut count = 1_u32;
    while let Some(Token::Prim) = iter.peek() {
        iter.next();
        if let Some(i) = count.checked_add(1) {
            count = i;
        } else {
            return Err(NyoomError::CompileError("Literal too long (thats a long nyoom file nice work)", 0));
        }
    }
    Ok(count)
}
fn count_prims_and_advance<U: Iterator<Item = Token>>(
    iter: &mut TokenIter<U>,
) -> Result<u16, NyoomError> {
    let mut count = 1_u16;
    while let Some(Token::Prim) = iter.peek() {
        iter.next();
        if let Some(i) = count.checked_add(1) {
            count = i;
        } else {
            return Err(NyoomError::CompileError("Identifier too long", 0));
        }
    }
    Ok(count)
}
fn count_secs_and_advance<U: Iterator<Item = Token>>(
    iter: &mut TokenIter<U>,
) -> Result<u16, NyoomError> {
    let mut count = 1_u16;
    while let Some(Token::Sec) = iter.peek() {
        iter.next();
        if let Some(i) = count.checked_add(1) {
            count = i;
        } else {
            return Err(NyoomError::CompileError("Identifier too long", 0));
        }
    }
    Ok(count)
}
fn check_if_split_and_advance<U: Iterator<Item = Token>>(
    iter: &mut TokenIter<U>,
) -> Result<bool, NyoomError> {
       Ok(iter.next_err()? == Token::Split)
}
