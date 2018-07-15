use std::error::Error;
use std::fmt;
use std::cmp;
use std::io;
use std::collections::BTreeMap;
use std::rc::Rc;
use std::cell::RefCell;

use printer;
use env::Env;
use core::eval_func;

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
    Function(Function),
    Lambda(Lambda),
    Atom(Rc<RefCell<MalType>>),
}

#[derive(Clone)]
pub struct Lambda {
    pub env: Env,
    pub args: Vec<MalType>,
    pub body: Vec<MalType>,
    pub is_macro: bool,
    pub metadata: Box<MalType>,
}

#[derive(Clone)]
pub struct Function {
    pub func: Box<fn(&mut Vec<MalType>, Option<Env>) -> MalResult>,
    pub env: Option<Env>,
    pub metadata: Box<MalType>,
}

impl MalType {
    pub fn nil() -> MalType {
        MalType::Nil
    }

    pub fn is_nil(&self) -> bool {
        match self {
            MalType::Nil => true,
            _ => false,
        }
    }

    pub fn bool_true() -> MalType {
        MalType::True
    }

    pub fn is_true(&self) -> bool {
        match self {
            MalType::True => true,
            _ => false,
        }
    }

    pub fn bool_false() -> MalType {
        MalType::False
    }

    pub fn is_false(&self) -> bool {
        match self {
            MalType::False => true,
            _ => false,
        }
    }

    pub fn is_falsey(&self) -> bool {
        match self {
            MalType::False | MalType::Nil => true,
            _ => false,
        }
    }

    pub fn number(val: i64) -> MalType {
        MalType::Number(val)
    }

    pub fn number_val(&self) -> Option<i64> {
        match self {
            MalType::Number(val) => Some(*val),
            _ => None,
        }
    }

    pub fn is_number(&self) -> bool {
        match self {
            MalType::Number(_) => true,
            _ => false,
        }
    }

    pub fn keyword<S: Into<String>>(val: S) -> MalType {
        MalType::Keyword(val.into())
    }

    pub fn keyword_val(&self) -> Option<&str> {
        match self {
            MalType::Keyword(ref val) => Some(val),
            _ => None,
        }
    }

    pub fn is_keyword(&self) -> bool {
        match self {
            MalType::Keyword(_) => true,
            _ => false,
        }
    }

    pub fn string<S: Into<String>>(val: S) -> MalType {
        MalType::String(val.into())
    }

    pub fn string_val(&self) -> Option<&str> {
        match self {
            MalType::String(ref val) => Some(val),
            _ => None,
        }
    }

    pub fn is_string(&self) -> bool {
        match self {
            MalType::String(_) => true,
            _ => false,
        }
    }

    pub fn symbol<S: Into<String>>(val: S) -> MalType {
        MalType::Symbol(val.into())
    }

    pub fn symbol_val(&self) -> Option<&str> {
        match self {
            MalType::Symbol(ref val) => Some(val),
            _ => None,
        }
    }

    pub fn is_symbol(&self) -> bool {
        match self {
            MalType::Symbol(_) => true,
            _ => false,
        }
    }

    pub fn list(vec: Vec<MalType>) -> MalType {
        MalType::List(vec, Box::new(MalType::Nil))
    }

    pub fn list_val(&self) -> Option<&Vec<MalType>> {
        match self {
            MalType::List(ref val, _) => Some(val),
            _ => None,
        }
    }

    pub fn is_list(&self) -> bool {
        match self {
            MalType::List(_, _) => true,
            _ => false,
        }
    }

    pub fn vector(vec: Vec<MalType>) -> MalType {
        MalType::Vector(vec, Box::new(MalType::Nil))
    }

    pub fn vector_val(&self) -> Option<&Vec<MalType>> {
        match self {
            MalType::Vector(ref val, _) => Some(val),
            _ => None,
        }
    }

    pub fn is_vector(&self) -> bool {
        match self {
            MalType::Vector(_, _) => true,
            _ => false,
        }
    }

    pub fn list_or_vector_val(&self) -> Option<&Vec<MalType>> {
        match self {
            MalType::List(ref val, _) | MalType::Vector(ref val, _) => Some(val),
            _ => None,
        }
    }

    pub fn is_list_or_vector(&self) -> bool {
        match self {
            MalType::List(_, _) | MalType::Vector(_, _) => true,
            _ => false,
        }
    }

    pub fn hashmap(map: BTreeMap<MalType, MalType>) -> MalType {
        MalType::HashMap(map, Box::new(MalType::Nil))
    }

    pub fn hashmap_val(&self) -> Option<&BTreeMap<MalType, MalType>> {
        match self {
            MalType::HashMap(ref val, _) => Some(val),
            _ => None,
        }
    }

    pub fn is_hashmap(&self) -> bool {
        match self {
            MalType::HashMap(_, _) => true,
            _ => false,
        }
    }

    pub fn function(
        func: Box<fn(&mut Vec<MalType>, Option<Env>) -> MalResult>,
        env: Option<Env>,
    ) -> MalType {
        MalType::Function(Function {
            env,
            func,
            metadata: Box::new(MalType::Nil),
        })
    }

    pub fn function_val(&self) -> Option<&Function> {
        match self {
            MalType::Function(function) => Some(function),
            _ => None,
        }
    }

    pub fn is_function(&self) -> bool {
        match self {
            MalType::Function { .. } => true,
            _ => false,
        }
    }

    pub fn lambda(env: Env, args: Vec<MalType>, body: Vec<MalType>) -> MalType {
        MalType::Lambda(Lambda {
            env,
            args,
            body,
            is_macro: false,
            metadata: Box::new(MalType::Nil),
        })
    }

    pub fn lambda_val(&self) -> Option<&Lambda> {
        match self {
            MalType::Lambda(lambda) => Some(lambda),
            _ => None,
        }
    }

    pub fn make_macro(&mut self) {
        if let MalType::Lambda(Lambda {
            ref mut is_macro, ..
        }) = self
        {
            *is_macro = true;
        } else {
            panic!("Not a lambda!");
        }
    }

    pub fn is_lambda(&self) -> bool {
        match self {
            MalType::Lambda { .. } => true,
            _ => false,
        }
    }

    pub fn atom(val: MalType) -> MalType {
        MalType::Atom(Rc::new(RefCell::new(val)))
    }

    pub fn atom_val(&self) -> Option<&Rc<RefCell<MalType>>> {
        match self {
            MalType::Atom(ref val) => Some(val),
            _ => None,
        }
    }

    pub fn is_atom(&self) -> bool {
        match self {
            MalType::Atom(_) => true,
            _ => false,
        }
    }

    pub fn get_metadata(&self) -> Option<&MalType> {
        match self {
            MalType::List(_, ref metadata)
            | MalType::Vector(_, ref metadata)
            | MalType::HashMap(_, ref metadata)
            | MalType::Function(Function { ref metadata, .. })
            | MalType::Lambda(Lambda { ref metadata, .. }) => Some(metadata),
            _ => None,
        }
    }

    pub fn set_metadata(&mut self, new_metadata: MalType) {
        match self {
            MalType::List(_, ref mut metadata)
            | MalType::Vector(_, ref mut metadata)
            | MalType::HashMap(_, ref mut metadata)
            | MalType::Function(Function {
                ref mut metadata, ..
            })
            | MalType::Lambda(Lambda {
                ref mut metadata, ..
            }) => *metadata = Box::new(new_metadata),
            _ => panic!("not a type with metadata"),
        }
    }

    pub fn copy_metadata(&mut self, other: &MalType) {
        if let Some(metadata) = other.get_metadata() {
            self.set_metadata(metadata.to_owned());
        } else {
            self.set_metadata(MalType::nil());
        }
    }

    pub fn swap(&mut self, func: MalType, args: &mut Vec<MalType>) -> MalResult {
        if let MalType::Atom(ref mut val) = *self {
            args.insert(0, val.borrow().to_owned());
            let new_val = eval_func(func, args)?;
            val.replace(new_val.clone());
            Ok(new_val)
        } else {
            Err(MalError::WrongArguments(
                "Must pass an atom to reset".to_string(),
            ))
        }
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
