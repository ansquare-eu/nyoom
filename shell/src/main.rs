use std::path::PathBuf;

fn main() {
    let path = PathBuf::new();
    let bytes = std::fs::read(path.join("nyoom.nyny")).unwrap();
    let result = nyoom::run(bytes, path);
    match result {
        Err(nyoom::NyoomError::CompileError(str, i)) => {
            println!("Parser error: {str} at position {i}")
        },
        Err(nyoom::NyoomError::ScannerError(str, i, arg)) => {
            println!("Scanner error: {str} at position {i} of token literal {arg}")
        },
        Err(nyoom::NyoomError::LinkerError(str, path)) => {
            println!("Linker error: {str} while linking file {path:?}")
        },
        Err(nyoom::NyoomError::RuntimeError(str)) => println!("Runtime error: {str}"),
        Ok(_) => println!("Success!"),
    };
}
