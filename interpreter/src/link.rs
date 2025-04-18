use std::{
    collections::HashSet,
    path::{Path, PathBuf},
};

use crate::{ast::Inst, parse, scan::scan, NyoomError, AST};

pub fn link(
    unlinked: AST,
    source_path: &PathBuf,
    linked: &mut HashSet<PathBuf>,
) -> Result<AST, NyoomError> {
    let mut linked_ast: AST = Vec::new();
    for inst in unlinked.into_iter() {
        if let Inst::Import(string) = inst {
            let path = source_path.join(PathBuf::from(string));
            if !linked.contains(&path) {
                let bytes = read_bytes(&path)?;
                let tokens = scan(bytes)?;
                linked.insert(path);
                let mut ast = link(parse::parse(tokens)?, source_path, linked)?;
                linked_ast.append(&mut ast);
            }
        } else {
            linked_ast.push(inst);
        }
    }
    Ok(linked_ast)
}
fn read_bytes(path: &Path) -> Result<Vec<u8>, NyoomError> {
    std::fs::read(path)
        .map_err(|_| NyoomError::LinkerError("Error reading file", PathBuf::from(path)))
}
