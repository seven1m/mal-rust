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
        let mut is_more = false;
        for bind in binds {
            if let MalType::Symbol(name) = bind {
                if name == "&" {
                    is_more = true;
                } else if is_more {
                    env.set(&name, MalType::List(exprs));
                    break;
                } else if exprs.len() > 0 {
                    env.set(&name, exprs.remove(0));
                }
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
