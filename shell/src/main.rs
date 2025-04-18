use std::path::PathBuf;

fn main() {
    let path = PathBuf::new();
    let bytes = std::fs::read(path.join("nyoom.nyny")).unwrap();
    let result = nyoom::run(bytes, path);
    match result {
        Err(nyoom::NyoomError::CompileError(str, i)) => {
            println!("Parser error: {str} at position {i}")
        }
        Err(nyoom::NyoomError::ScannerError(str, i, arg)) => {
            println!("Scanner error: {str} at position {i} of token literal {arg}")
        }
        Err(nyoom::NyoomError::LinkerError(str, path)) => {
            println!("Linker error: {str} while linking file {path:?}")
        }
        Err(nyoom::NyoomError::RuntimeError(str)) => println!("Runtime error: {str}"),
        Err(nyoom::NyoomError::NoSuchElementError(str, i, u)) => {
            println!("No such element {i}: {str}, array only has {u} elements")
        }
        Err(nyoom::NyoomError::NoSuchDefinitionError(str, i)) => {
            println!("No definition found for id {i:?}: {str}")
        }
        Err(nyoom::NyoomError::InvalidArgumentError(str, i)) => {
            println!("Invalid argument {i:?}: {str}")
        }
        Err(nyoom::NyoomError::BuiltinFnTypeError(str)) => {
            println!("Invalid builtin function type: {str}")
        }
        Ok(_) => println!("Success!"),
    };
}
