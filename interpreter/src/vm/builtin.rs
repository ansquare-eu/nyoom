use crate::NyoomError;

use super::VarContent;

type BuiltinFn = fn(VarContent) -> FnResult;
pub type Builtins = phf::Map<u16, BuiltinFn>;
pub type FnResult = Result<VarContent, NyoomError>;
pub fn builtin_add(arg: VarContent) -> FnResult {
    if let VarContent::Array(vec) = arg {
        if let (Some(VarContent::Int(a)), Some(VarContent::Int(b))) = (vec.first(), vec.get(1)) {
            return Ok(VarContent::Int(a + b));
        }
    }
    Err(NyoomError::BuiltinFnTypeError(
        "Expected type ARRAY OF INT AND INT for ADD",
    ))
}
pub fn builtin_subtract(arg: VarContent) -> FnResult {
    if let VarContent::Array(vec) = arg {
        if let (Some(VarContent::Int(a)), Some(VarContent::Int(b))) = (vec.first(), vec.get(1)) {
            return Ok(VarContent::Int(a - b));
        }
    }
    Err(NyoomError::BuiltinFnTypeError(
        "Expected type ARRAY OF INT AND INT for SUBTRACT",
    ))
}
pub fn builtin_multiply(arg: VarContent) -> FnResult {
    if let VarContent::Array(vec) = arg {
        if let (Some(VarContent::Int(a)), Some(VarContent::Int(b))) = (vec.first(), vec.get(1)) {
            return Ok(VarContent::Int(a * b));
        }
    }
    Err(NyoomError::BuiltinFnTypeError(
        "Expected type ARRAY OF INT AND INT for MULTIPLY",
    ))
}
pub fn builtin_divide(arg: VarContent) -> FnResult {
    if let VarContent::Array(vec) = arg {
        if let (Some(VarContent::Int(a)), Some(VarContent::Int(b))) = (vec.first(), vec.get(1)) {
            return Ok(VarContent::Int(a * b));
        }
    }
    Err(NyoomError::BuiltinFnTypeError(
        "Expected type ARRAY OF INT AND INT for DIVIDE",
    ))
}
pub fn builtin_lenght(arg: VarContent) -> FnResult {
    if let VarContent::Array(vec) = arg {
        //If somebody somehow makes an array longer that 2^31 - 1 this will sure prank them
        return Ok(VarContent::Int(vec.len() as i32));
    }
    Err(NyoomError::BuiltinFnTypeError(
        "Expected type ARRAY for ARRAY LENGHT",
    ))
}
pub fn builtin_index(arg: VarContent) -> FnResult {
    if let VarContent::Array(vec) = arg {
        if let (Some(VarContent::Int(i)), Some(VarContent::Array(vec))) = (vec.first(), vec.get(1))
        {
            if i.is_negative() {
                return Err(NyoomError::NoSuchElementError(
                    "Index is negative",
                    *i,
                    vec.len(),
                ));
            }
            return vec
                .get(*i as usize)
                .cloned()
                .ok_or(NyoomError::NoSuchElementError(
                    "Array does not contain this element",
                    *i,
                    vec.len(),
                ));
        }
    }
    Err(NyoomError::BuiltinFnTypeError(
        "Expected type ARRAY OF INT AND ARRAY for INDEX",
    ))
}
pub fn builtin_print(arg: VarContent) -> FnResult {
    if let VarContent::Int(i) = arg {
        if i.is_positive() {
            if let Some(c) = char::from_u32(i as u32) {
                print!("{}", c);
                return Ok(VarContent::Nil);
            }
        }
        return Err(NyoomError::InvalidArgumentError(
            "INT must be a valid UTF-8 scalar value to be printed",
            arg,
        ));
    }
    Err(NyoomError::BuiltinFnTypeError(
        "Expected type INT for PRINT",
    ))
}
