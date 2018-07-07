use std::error::Error;
use std::fmt;
use std::cmp;
use std::io;
use std::collections::BTreeMap;
use std::rc::Rc;
use std::cell::RefCell;

use printer;
use env::Env;

#[derive(Clone)]
pub enum MalType {
    Nil,
    True,
    False,
    Number(i64),
    Keyword(String),
    String(String),
    Symbol(String),
    List(Vec<MalType>, Box<MalType>),
    Vector(Vec<MalType>, Box<MalType>),
    HashMap(BTreeMap<MalType, MalType>, Box<MalType>),
    Function {
        func: Box<fn(&mut Vec<MalType>, Option<Env>) -> MalResult>,
        env: Option<Env>,
        metadata: Box<MalType>,
    },
    Lambda {
        env: Env,
        args: Vec<MalType>,
        body: Vec<MalType>,
        is_macro: bool,
        metadata: Box<MalType>,
    },
    Atom(Rc<RefCell<MalType>>),
}

impl MalType {
    pub fn function(
        func: Box<fn(&mut Vec<MalType>, Option<Env>) -> MalResult>,
        env: Option<Env>,
    ) -> MalType {
        MalType::Function {
            env,
            func,
            metadata: Box::new(MalType::Nil),
        }
    }

    pub fn lambda(env: Env, args: Vec<MalType>, body: Vec<MalType>) -> MalType {
        MalType::Lambda {
            env,
            args,
            body,
            is_macro: false,
            metadata: Box::new(MalType::Nil),
        }
    }

    pub fn list(vec: Vec<MalType>) -> MalType {
        MalType::List(vec, Box::new(MalType::Nil))
    }

    pub fn vector(vec: Vec<MalType>) -> MalType {
        MalType::Vector(vec, Box::new(MalType::Nil))
    }

    pub fn hashmap(map: BTreeMap<MalType, MalType>) -> MalType {
        MalType::HashMap(map, Box::new(MalType::Nil))
    }
}

impl cmp::PartialEq for MalType {
    fn eq(&self, other: &MalType) -> bool {
        match (self, other) {
            (&MalType::Nil, &MalType::Nil) => true,
            (&MalType::True, &MalType::True) => true,
            (&MalType::False, &MalType::False) => true,
            (&MalType::Number(ref a), &MalType::Number(ref b)) => a == b,
            (&MalType::Keyword(ref a), &MalType::Keyword(ref b)) => a == b,
            (&MalType::String(ref a), &MalType::String(ref b)) => a == b,
            (&MalType::Symbol(ref a), &MalType::Symbol(ref b)) => a == b,
            (&MalType::List(ref a, _), &MalType::List(ref b, _)) => a == b,
            (&MalType::Vector(ref a, _), &MalType::Vector(ref b, _)) => a == b,
            (&MalType::HashMap(ref a, _), &MalType::HashMap(ref b, _)) => a == b,
            _ => false,
        }
    }
}

impl cmp::Eq for MalType {}

impl cmp::PartialOrd for MalType {
    fn partial_cmp(&self, other: &MalType) -> Option<cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl cmp::Ord for MalType {
    fn cmp(&self, other: &MalType) -> cmp::Ordering {
        printer::pr_str(self, true).cmp(&printer::pr_str(other, true))
    }
}

impl fmt::Debug for MalType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", printer::pr_str(self, true))
    }
}

pub type MalResult = Result<MalType, MalError>;

#[derive(Debug, PartialEq)]
pub enum MalError {
    Generic(MalType),
    Parse(String),
    SymbolUndefined(String),
    WrongArguments(String),
    NotAFunction(MalType),
    IO(String),
    NotANumber,
    DivideByZero,
    BlankLine,
    IndexOutOfBounds { size: usize, index: usize },
}

impl fmt::Display for MalError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            MalError::Generic(ref val) => write!(f, "{:?}", val),
            MalError::Parse(ref msg) => write!(f, "Parse error: {}", msg),
            MalError::SymbolUndefined(ref sym) => write!(f, "'{}' not found", sym),
            MalError::WrongArguments(ref msg) => write!(f, "Wrong arguments: {}", msg),
            MalError::NotAFunction(ref val) => write!(f, "Not a function: {:?}", val),
            MalError::IO(ref err) => write!(f, "IO Error: {}", err),
            MalError::NotANumber => write!(f, "Error: Not a number"),
            MalError::DivideByZero => write!(f, "Error: Divide by zero"),
            MalError::BlankLine => write!(f, "Blank line"),
            MalError::IndexOutOfBounds {
                ref index,
                ref size,
            } => write!(f, "Index ({:?}) out of bounds ({:?})", index, size),
        }
    }
}

impl Error for MalError {
    fn description(&self) -> &str {
        match *self {
            MalError::Generic(_) => "Error",
            MalError::Parse(_) => "Parse error",
            MalError::SymbolUndefined(_) => "Symbol undefined",
            MalError::WrongArguments(_) => "Wrong arguments",
            MalError::NotAFunction(_) => "Not a function",
            MalError::IO(_) => "IO Error",
            MalError::NotANumber => "Not a number",
            MalError::DivideByZero => "Divide by zero",
            MalError::BlankLine => "Blank line",
            MalError::IndexOutOfBounds { .. } => "Index out of bounds",
        }
    }

    fn cause(&self) -> Option<&Error> {
        None
    }
}

impl From<io::Error> for MalError {
    fn from(err: io::Error) -> MalError {
        MalError::IO(format!("{}", err))
    }
}

#[derive(Debug)]
pub enum TailPosition {
    Call(MalType, Option<Env>),
    Return(MalType),
}

pub type TailPositionResult = Result<TailPosition, MalError>;
