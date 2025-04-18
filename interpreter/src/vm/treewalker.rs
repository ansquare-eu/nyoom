use crate::{
    ast::{self},
    NyoomError, AST,
};
use std::rc::Rc;

use super::{LocalFnContent, VarContent, VmCtx};
//Runs the tree walking interpreter with the given VM context
pub(crate) fn walk_tree(ctx: VmCtx, ast: AST) -> Result<(), NyoomError> {
    for inst in ast.into_iter() {
        run_inst(&ctx, &inst, None)?;
    }
    println!();
    Ok(())
}
//These run individual instructions. They should ALWAYS forward the arg
fn run_inst(ctx: &VmCtx, inst: &ast::Inst, arg: Option<Rc<ast::Expr>>) -> Result<(), NyoomError> {
    match inst {
        ast::Inst::Import(_) | ast::Inst::Alias(_, _) | ast::Inst::FnDef(_, _) => {
            Err(NyoomError::RuntimeError("These insts should not be here"))
        }
        ast::Inst::SetVar(id, expr) => {
            let res = evaluate_expression(ctx, expr, arg)?;
            ctx.set_var(*id, res);
            Ok(())
        }
        ast::Inst::Eval(expr) => evaluate_expression(ctx, expr, arg).map(|_| ()),
        //Else blocks are not supported yet, so '_else' is always NONE
        ast::Inst::If(expr, block, _else) => {
            let var = evaluate_expression(ctx, expr, arg.clone())?;
            if let VarContent::Int(i) = var {
                if i != 0 {
                    evaluate_control_block(ctx, block, arg)?;
                }
                Ok(())
            } else {
                Err(NyoomError::InvalidArgumentError(
                    "An IF block needs an INT argument",
                    var,
                ))
            }
        }
        ast::Inst::While(expr, block) => loop {
            let var = evaluate_expression(ctx, expr, arg.clone())?;
            if let VarContent::Int(i) = var {
                if i != 0 {
                    evaluate_control_block(ctx, block, arg.clone())?;
                } else {
                    break Ok(());
                }
            } else {
                break Err(NyoomError::InvalidArgumentError(
                    "A WHILE block needs an INT argument",
                    var,
                ));
            }
        },
        //These in fact do nothing
        //Returns are processed by the function runner
        ast::Inst::Return(_) => Ok(()),
    }
}
fn evaluate_control_block(
    ctx: &VmCtx,
    block: &ast::Block,
    arg: Option<Rc<ast::Expr>>,
) -> Result<(), NyoomError> {
    for inst in block.0.iter() {
        run_inst(ctx, inst, arg.clone())?;
    }
    Ok(())
}
//These evaluate expression. They should forward the arg to sub-expression evals, but not to function calls
fn evaluate_expression(
    ctx: &VmCtx,
    expr: &ast::Expr,
    arg: Option<Rc<ast::Expr>>,
) -> Result<VarContent, NyoomError> {
    match expr {
        ast::Expr::Var(id) => ctx.get_var(*id).ok_or(NyoomError::NoSuchDefinitionError(
            "Tried to use unknown var as expression",
            *id,
        )),
        ast::Expr::Int(int) => Ok(VarContent::Int(*int as i32)),
        ast::Expr::CallLoc(fn_call) => {
            if let Some(function) = ctx.get_local(fn_call.0) {
                match function {
                    LocalFnContent::Block(block) => {
                        evaluate_local_function_block(ctx, block, fn_call.1.clone())
                    }
                    LocalFnContent::Alias(id) => {
                        let aliased_id = id.to_builtin();
                        if let Some(fun) = ctx.builtins.get(&aliased_id) {
                            fun(evaluate_expression(ctx, &fn_call.1, None)?)
                        } else {
                            Err(NyoomError::NoSuchDefinitionError(
                                "No builtin function defined",
                                *id,
                            ))
                        }
                    }
                }
            } else {
                Err(NyoomError::NoSuchDefinitionError(
                    "Tried to call unknown local function as expression",
                    fn_call.0,
                ))
            }
        }
        ast::Expr::CallBuiltin(fn_call) => {
            if let Some(fun) = ctx.builtins.get(&fn_call.0.to_builtin()) {
                fun(evaluate_expression(ctx, &fn_call.1, arg)?)
            } else {
                Err(NyoomError::NoSuchDefinitionError(
                    "Tried to call unknown local function as expression",
                    fn_call.0,
                ))
            }
        }
        ast::Expr::Concat(exprs) => {
            let mut vec = Vec::new();
            for expr in exprs.iter() {
                vec.push(evaluate_expression(ctx, expr, arg.clone())?);
            }
            Ok(VarContent::Array(vec))
        }
        ast::Expr::Arg => {
            if let Some(arg) = arg {
                evaluate_expression(ctx, &arg, None)
            } else {
                Err(NyoomError::RuntimeError(
                    "Used ARG expression when inappropriate",
                ))
            }
        }
    }
}
//These evaluate local functions
fn evaluate_local_function_block(
    ctx: &VmCtx,
    block: &ast::Block,
    arg: Rc<ast::Expr>,
) -> Result<VarContent, NyoomError> {
    for inst in block.0.iter() {
        if let ast::Inst::Return(expr) = inst {
            return evaluate_expression(ctx, expr, Some(arg.clone()));
        } else {
            run_inst(ctx, inst, Some(arg.clone()))?;
        }
    }
    Err(NyoomError::RuntimeError("Local function had no return!"))
}
