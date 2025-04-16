use std::rc::Rc;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Id {
    NyNy(u16),
    NyNu(u16),
}
impl Id {
    pub fn to_builtin(self) -> u16 {
        match self {
            Id::NyNy(i) => i,
            Id::NyNu(i) => i,
        }
    }
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
pub struct FnCall(pub Id, pub Rc<Expr>);

#[derive(Debug)]
pub struct Block(pub Vec<Inst>);

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
