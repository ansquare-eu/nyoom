use scan::scan;


mod scan;

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
