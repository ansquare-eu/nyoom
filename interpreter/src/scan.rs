use crate::NyoomError;

fn split(bytes: Vec<u8>) -> Result<Vec<String>, NyoomError> {
    let string = String::from_utf8(bytes).unwrap();
    Ok(string.lines().map(|x| {
        let mut vec = x.split_whitespace().collect::<Vec<&str>>();
        vec.push("\n");
        vec
    }).flatten().map(|x| String::from(x)).collect())
}
pub fn scan(bytes: Vec<u8>) -> Result<Vec<Token>, NyoomError> {
    let split = split(bytes)?;
    let mut is_comment = false;
    let mut vec:Vec<Token> = Vec::new();
    for (i, str) in split.iter().enumerate() {
        if str.contains("//") {
            is_comment = true;
            continue;
        } else if is_comment {
            if str.contains('\n') {
                is_comment = false
            }
            continue;
        }
        vec.push( match str.as_str() {
            "nyoom" => Token::Prim,
            "nugget" => Token::Sec,
            "I" => Token::Split,
            _ => return Err(NyoomError::CompileError("Invalid token", i))
        });
    }
    Ok(vec)
}
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Token {
    Prim,
    Sec,
    Split,
    //TODO Add NyNu support with hashed ids
}
