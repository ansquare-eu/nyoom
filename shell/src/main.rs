use std::path::Path;


fn main() {
    let bytes = std::fs::read(Path::new("nyoom.nyny")).unwrap();
    let result = nyoom::run(bytes);
    match result {
        Err(nyoom::NyoomError::CompileError(str, i)) => println!("Compile error: {str} at position {i}"),
        Err(nyoom::NyoomError::RuntimeError(str)) => println!("Runtime error: {str}"),
        Ok(_) => println!("Success!")
    };
}
