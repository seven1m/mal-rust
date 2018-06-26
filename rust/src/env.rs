use types::*;
use std::collections::HashMap;

#[derive(Debug)]
pub struct Env<'a> {
    pub outer: Option<&'a Env<'a>>,
    pub data: HashMap<String, MalType>,
}

impl<'a> Env<'a> {
    pub fn new(outer: Option<&'a Env>) -> Env<'a> {
        Env {
            outer: outer,
            data: HashMap::new(),
        }
    }

    pub fn set(&mut self, key: &str, val: MalType) {
        self.data.insert(key.to_string(), val);
    }

    pub fn find(&self, key: &str) -> Option<&Env> {
        if self.data.contains_key(key) {
            Some(self)
        } else if let Some(ref outer) = self.outer {
            outer.find(key)
        } else {
            None
        }
    }

    pub fn get(&self, key: &str) -> Result<MalType, MalError> {
        if let Some(env) = self.find(key) {
            if let Some(val) = env.data.get(key) {
                return Ok(val.clone());
            }
        }
        Err(MalError::SymbolUndefined(key.to_string()))
    }
}

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
