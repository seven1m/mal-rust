use std::error::Error;
use std::fmt;

#[derive(Debug,PartialEq)]
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
}

pub type MalResult = Result<MalType, MalError>;

#[derive(Debug,PartialEq)]
pub enum MalError {
    Parse(String),
}

impl fmt::Display for MalError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            MalError::Parse(ref msg) => write!(f, "Parse error: {}", msg),
        }
    }
}

impl Error for MalError {
    fn description(&self) -> &str {
        match *self {
            MalError::Parse(ref msg) => msg,
        }
    }

    fn cause(&self) -> Option<&Error> {
        None
    }
}
