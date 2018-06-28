use types::*;

use std::collections::HashMap;

lazy_static! {
    pub static ref NS: HashMap<String, fn(&mut Vec<MalType>) -> MalResult> = {
        let mut ns = HashMap::new();
        ns.insert("+".to_string(), add as fn(&mut Vec<MalType>) -> MalResult);
        ns.insert("-".to_string(), subtract as fn(&mut Vec<MalType>) -> MalResult);
        ns.insert("*".to_string(), multiply as fn(&mut Vec<MalType>) -> MalResult);
        ns.insert("/".to_string(), divide as fn(&mut Vec<MalType>) -> MalResult);
        ns
    };
}
/*

  = note: expected type `std::collections::HashMap<_, std::boxed::Box<for<'r> fn(&'r mut std::vec::Vec<types::MalType>) -> std::result::Result<types::MalType, types::MalError>>, _>`
             found type `std::collections::HashMap<_, std::boxed::Box<for<'r> fn(&'r mut std::vec::Vec<types::MalType>) -> std::result::Result<types::MalType, types::MalError> {core::add}>, _>`


*/

pub fn add(args: &mut Vec<MalType>) -> MalResult {
    if args.len() > 0 {
        let mut iter = MalNumberIter { items: args };
        let mut answer = iter.next().unwrap()?;
        for num in iter {
            answer += num?;
        }
        Ok(MalType::Number(answer))
    } else {
        Err(MalError::WrongArguments(
            "Must pass at least one number".to_string(),
        ))
    }
}

pub fn subtract(args: &mut Vec<MalType>) -> MalResult {
    if args.len() > 0 {
        let mut iter = MalNumberIter { items: args };
        let mut answer = iter.next().unwrap()?;
        for num in iter {
            answer -= num?;
        }
        Ok(MalType::Number(answer))
    } else {
        Err(MalError::WrongArguments(
            "Must pass at least one number".to_string(),
        ))
    }
}

pub fn multiply(args: &mut Vec<MalType>) -> MalResult {
    if args.len() > 0 {
        let mut iter = MalNumberIter { items: args };
        let mut answer = iter.next().unwrap()?;
        for num in iter {
            answer *= num?;
        }
        Ok(MalType::Number(answer))
    } else {
        Err(MalError::WrongArguments(
            "Must pass at least one number".to_string(),
        ))
    }
}

pub fn divide(args: &mut Vec<MalType>) -> MalResult {
    if args.len() > 0 {
        let mut iter = MalNumberIter { items: args };
        let mut answer = iter.next().unwrap()?;
        for num in iter {
            let num = num?;
            if num == 0 {
                return Err(MalError::DivideByZero);
            } else {
                answer /= num;
            }
        }
        Ok(MalType::Number(answer))
    } else {
        Err(MalError::WrongArguments(
            "Must pass at least one number".to_string(),
        ))
    }
}

struct MalNumberIter<'a> {
    items: &'a mut Vec<MalType>,
}

impl<'a> Iterator for MalNumberIter<'a> {
    type Item = Result<i64, MalError>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.items.len() == 0 {
            return None;
        }
        let item = self.items.remove(0);
        if let MalType::Number(num) = item {
            Some(Ok(num))
        } else {
            Some(Err(MalError::NotANumber))
        }
    }
}
