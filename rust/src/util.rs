use types::*;

pub fn num_result(arg: &MalType) -> Result<i64, MalError> {
    if let Some(num) = arg.number_val() {
        Ok(num)
    } else {
        Err(MalError::WrongArguments(
            format!("Expected a number but got: {:?}", arg).to_string(),
        ))
    }
}

pub fn vec_result(arg: &MalType) -> Result<Vec<MalType>, MalError> {
    if let Some(vec) = arg.list_or_vector_val() {
        Ok(vec.clone())
    } else {
        Err(MalError::WrongArguments(
            format!("Expected a list or vector but got: {:?}", arg).to_string(),
        ))
    }
}
