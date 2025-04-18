mod builtin;
mod treewalker;

use std::{cell::RefCell, collections::HashMap};

use crate::{
    ast::{self, Block, Id, Inst},
    NyoomError, AST,
};
use builtin::Builtins;
use phf::phf_map;
type Locals = HashMap<ast::Id, LocalFnContent>;
type Vars = HashMap<ast::Id, VarContent>;

static DEFAULT_BUILTINS: Builtins = phf_map! {
    1_u16 => builtin::builtin_add,
    2_u16 => builtin::builtin_subtract,
    3_u16 => builtin::builtin_multiply,
    4_u16 => builtin::builtin_divide,
    5_u16 => builtin::builtin_lenght,
    6_u16 => builtin::builtin_index,
    7_u16 => builtin::builtin_print
};
#[derive(Clone, Debug)]
pub enum VarContent {
    Nil,
    Int(i32),
    Array(Vec<VarContent>),
}
enum LocalFnContent {
    Block(Block),
    Alias(Id),
}
pub(crate) struct VmCtx {
    vars: RefCell<Vars>,
    locals: Locals,
    builtins: &'static Builtins,
}
impl VmCtx {
    fn get_local(&self, id: Id) -> Option<&LocalFnContent> {
        self.locals.get(&id)
    }
    fn set_var(&self, id: Id, value: VarContent) {
        self.vars.borrow_mut().insert(id, value);
    }
    fn get_var(&self, id: Id) -> Option<VarContent> {
        self.vars.borrow().get(&id).cloned()
    }
}
//Runs the tree-walking interpreter using the default builtin functions
pub fn run_tree_walk(mut ast: AST) -> Result<(), NyoomError> {
    let local_fns = create_fn_table(&mut ast);
    let ctx = VmCtx {
        vars: RefCell::new(HashMap::new()),
        locals: local_fns,
        builtins: &DEFAULT_BUILTINS,
    };
    treewalker::walk_tree(ctx, ast)?;
    Ok(())
}
//Extracts the local function definitions (defs and aliases) from the AST to pass them to the VM Context
fn create_fn_table(ast: &mut AST) -> Locals {
    let mut map = HashMap::new();
    let mut i = 0;
    while i < ast.len() {
        if let Inst::FnDef(_, _) = &ast[i] {
            if let Inst::FnDef(id, block) = ast.remove(i) {
                map.insert(id, LocalFnContent::Block(block));
            }
        } else if let Inst::Alias(_, _) = &ast[i] {
            if let Inst::Alias(id, id2) = ast.remove(i) {
                map.insert(id, LocalFnContent::Alias(id2));
            }
        } else {
            i += 1;
        }
    }
    map
}
