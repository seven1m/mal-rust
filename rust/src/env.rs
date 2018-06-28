use types::*;
use std::collections::HashMap;
use std::rc::Rc;
use std::cell::RefCell;

#[derive(Debug)]
pub struct EnvType {
    pub outer: Option<Env>,
    pub data: HashMap<String, MalType>,
}

#[derive(Debug, Clone)]
pub struct Env(Rc<RefCell<EnvType>>);

impl Env {
    pub fn new(outer: Option<&Env>) -> Env {
        Env(Rc::new(RefCell::new(EnvType {
            outer: outer.map(|e| e.clone()),
            data: HashMap::new(),
        })))
    }

    pub fn with_binds(outer: Option<&Env>, binds: Vec<MalType>, mut exprs: Vec<MalType>) -> Env {
        let env = Env::new(outer);
        for bind in binds {
            if let MalType::Symbol(name) = bind {
                env.set(&name, exprs.remove(0));
            } else {
                panic!("Expected a MalType::Symbol!");
            }
        }
        env
    }

    pub fn set(&self, key: &str, val: MalType) {
        self.0.borrow_mut().data.insert(key.to_string(), val);
    }

    pub fn find(&self, key: &str) -> Option<Env> {
        if self.0.borrow().data.contains_key(key) {
            Some(self.clone())
        } else {
            let b = self.0.borrow();
            if let Some(ref outer) = b.outer {
                outer.find(key).map(|e| e.clone())
            } else {
                None
            }
        }
    }

    pub fn get(&self, key: &str) -> Result<MalType, MalError> {
        if let Some(env) = self.find(key) {
            if let Some(val) = env.0.borrow().data.get(key) {
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
