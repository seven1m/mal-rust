use types::*;

pub fn raw_num(arg: &MalType) -> Result<i64, MalError> {
    if let MalType::Number(num) = *arg {
        Ok(num)
    } else {
        Err(MalError::WrongArguments(
            format!("Expected a number but got: {:?}", arg).to_string(),
        ))
    }
}

pub fn raw_vec(arg: &MalType) -> Result<Vec<MalType>, MalError> {
    match *arg {
        MalType::List(ref vec, _) | MalType::Vector(ref vec, _) => Ok(vec.clone()),
        _ => Err(MalError::WrongArguments(
            format!("Expected a list or vector but got: {:?}", arg).to_string(),
        )),
    }
}
