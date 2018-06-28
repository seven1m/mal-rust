use std::error::Error;
use std::fmt;
use std::cmp;
use std::collections::BTreeMap;

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
    List(Vec<MalType>),
    Vector(Vec<MalType>),
    HashMap(BTreeMap<MalType, MalType>),
    Function(Box<fn(&mut Vec<MalType>) -> MalResult>),
    Lambda {
        env: Env,
        args: Vec<MalType>,
        body: Vec<MalType>,
    },
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
            (&MalType::List(ref a), &MalType::List(ref b)) => a == b,
            (&MalType::Vector(ref a), &MalType::Vector(ref b)) => a == b,
            (&MalType::HashMap(ref a), &MalType::HashMap(ref b)) => a == b,
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
    Parse(String),
    SymbolUndefined(String),
    WrongArguments(String),
    NotAFunction(MalType),
    NotANumber,
    DivideByZero,
    BlankLine,
}

impl fmt::Display for MalError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            MalError::Parse(ref msg) => write!(f, "Parse error: {}", msg),
            MalError::SymbolUndefined(ref sym) => write!(f, "Symbol undefined: {}", sym),
            MalError::WrongArguments(ref msg) => write!(f, "Wrong arguments: {}", msg),
            MalError::NotAFunction(ref val) => write!(f, "Error: Not a function: {:?}", val),
            MalError::NotANumber => write!(f, "Error: Not a number"),
            MalError::DivideByZero => write!(f, "Error: Divide by zero"),
            MalError::BlankLine => write!(f, "Blank line"),
        }
    }
}

impl Error for MalError {
    fn description(&self) -> &str {
        match *self {
            MalError::Parse(_) => "Parse error",
            MalError::SymbolUndefined(_) => "Symbol undefined",
            MalError::WrongArguments(_) => "Wrong arguments",
            MalError::NotAFunction(_) => "Not a function",
            MalError::NotANumber => "Not a number",
            MalError::DivideByZero => "Divide by zero",
            MalError::BlankLine => "Blank line",
        }
    }

    fn cause(&self) -> Option<&Error> {
        None
    }
}
