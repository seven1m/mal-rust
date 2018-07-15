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
pub enum _MalType {
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
    Function(Function, Box<MalType>),
    Lambda(Lambda, Box<MalType>),
    Atom(Rc<RefCell<MalType>>),
}

#[derive(Clone)]
pub struct Lambda {
    pub env: Env,
    pub args: Vec<MalType>,
    pub body: Vec<MalType>,
    pub is_macro: bool,
}

#[derive(Clone)]
pub struct Function {
    pub func: Box<fn(&mut Vec<MalType>, Option<Env>) -> MalResult>,
    pub env: Option<Env>,
}

#[derive(Clone)]
pub struct MalType(Rc<_MalType>);

impl MalType {
    pub fn nil() -> MalType {
        MalType(Rc::new(_MalType::Nil))
    }

    pub fn is_nil(&self) -> bool {
        match *self.0 {
            _MalType::Nil => true,
            _ => false,
        }
    }

    pub fn bool_true() -> MalType {
        MalType(Rc::new(_MalType::True))
    }

    pub fn is_true(&self) -> bool {
        match *self.0 {
            _MalType::True => true,
            _ => false,
        }
    }

    pub fn bool_false() -> MalType {
        MalType(Rc::new(_MalType::False))
    }

    pub fn is_false(&self) -> bool {
        match *self.0 {
            _MalType::False => true,
            _ => false,
        }
    }

    pub fn is_falsey(&self) -> bool {
        match *self.0 {
            _MalType::False | _MalType::Nil => true,
            _ => false,
        }
    }

    pub fn number(val: i64) -> MalType {
        MalType(Rc::new(_MalType::Number(val)))
    }

    pub fn number_val(&self) -> Option<i64> {
        match *self.0 {
            _MalType::Number(val) => Some(val),
            _ => None,
        }
    }

    pub fn is_number(&self) -> bool {
        match *self.0 {
            _MalType::Number(_) => true,
            _ => false,
        }
    }

    pub fn keyword<S: Into<String>>(val: S) -> MalType {
        MalType(Rc::new(_MalType::Keyword(val.into())))
    }

    pub fn keyword_val(&self) -> Option<&str> {
        match *self.0 {
            _MalType::Keyword(ref val) => Some(val),
            _ => None,
        }
    }

    pub fn is_keyword(&self) -> bool {
        match *self.0 {
            _MalType::Keyword(_) => true,
            _ => false,
        }
    }

    pub fn string<S: Into<String>>(val: S) -> MalType {
        MalType(Rc::new(_MalType::String(val.into())))
    }

    pub fn string_val(&self) -> Option<&str> {
        match *self.0 {
            _MalType::String(ref val) => Some(val),
            _ => None,
        }
    }

    pub fn is_string(&self) -> bool {
        match *self.0 {
            _MalType::String(_) => true,
            _ => false,
        }
    }

    pub fn symbol<S: Into<String>>(val: S) -> MalType {
        MalType(Rc::new(_MalType::Symbol(val.into())))
    }

    pub fn symbol_val(&self) -> Option<&str> {
        match *self.0 {
            _MalType::Symbol(ref val) => Some(val),
            _ => None,
        }
    }

    pub fn is_symbol(&self) -> bool {
        match *self.0 {
            _MalType::Symbol(_) => true,
            _ => false,
        }
    }

    pub fn list(vec: Vec<MalType>) -> MalType {
        MalType::list_with_meta(vec, MalType::nil())
    }

    pub fn list_with_meta(vec: Vec<MalType>, meta: MalType) -> MalType {
        MalType(Rc::new(_MalType::List(vec, Box::new(meta))))
    }

    pub fn list_val(&self) -> Option<&Vec<MalType>> {
        match *self.0 {
            _MalType::List(ref val, _) => Some(val),
            _ => None,
        }
    }

    pub fn is_list(&self) -> bool {
        match *self.0 {
            _MalType::List(_, _) => true,
            _ => false,
        }
    }

    pub fn vector(vec: Vec<MalType>) -> MalType {
        MalType::vector_with_meta(vec, MalType::nil())
    }

    pub fn vector_with_meta(vec: Vec<MalType>, meta: MalType) -> MalType {
        MalType(Rc::new(_MalType::Vector(vec, Box::new(meta))))
    }

    pub fn vector_val(&self) -> Option<&Vec<MalType>> {
        match *self.0 {
            _MalType::Vector(ref val, _) => Some(val),
            _ => None,
        }
    }

    pub fn is_vector(&self) -> bool {
        match *self.0 {
            _MalType::Vector(_, _) => true,
            _ => false,
        }
    }

    pub fn list_or_vector_val(&self) -> Option<&Vec<MalType>> {
        match *self.0 {
            _MalType::List(ref val, _) | _MalType::Vector(ref val, _) => Some(val),
            _ => None,
        }
    }

    pub fn is_list_or_vector(&self) -> bool {
        match *self.0 {
            _MalType::List(_, _) | _MalType::Vector(_, _) => true,
            _ => false,
        }
    }

    pub fn hashmap(map: BTreeMap<MalType, MalType>) -> MalType {
        MalType::hashmap_with_meta(map, MalType::nil())
    }

    pub fn hashmap_with_meta(map: BTreeMap<MalType, MalType>, meta: MalType) -> MalType {
        MalType(Rc::new(_MalType::HashMap(map, Box::new(meta))))
    }

    pub fn hashmap_val(&self) -> Option<&BTreeMap<MalType, MalType>> {
        match *self.0 {
            _MalType::HashMap(ref val, _) => Some(val),
            _ => None,
        }
    }

    pub fn is_hashmap(&self) -> bool {
        match *self.0 {
            _MalType::HashMap(_, _) => true,
            _ => false,
        }
    }

    pub fn function(function: Function) -> MalType {
        MalType::function_with_meta(function, MalType::nil())
    }

    pub fn function_with_meta(function: Function, meta: MalType) -> MalType {
        MalType(Rc::new(_MalType::Function(function, Box::new(meta))))
    }

    pub fn function_val(&self) -> Option<&Function> {
        match *self.0 {
            _MalType::Function(ref function, _) => Some(function),
            _ => None,
        }
    }

    pub fn is_function(&self) -> bool {
        match *self.0 {
            _MalType::Function { .. } => true,
            _ => false,
        }
    }

    pub fn lambda(lambda: Lambda) -> MalType {
        MalType::lambda_with_meta(lambda, MalType::nil())
    }

    pub fn lambda_with_meta(lambda: Lambda, meta: MalType) -> MalType {
        MalType(Rc::new(_MalType::Lambda(lambda, Box::new(meta))))
    }

    pub fn lambda_val(&self) -> Option<&Lambda> {
        match *self.0 {
            _MalType::Lambda(ref lambda, _) => Some(lambda),
            _ => None,
        }
    }

    pub fn make_macro(&mut self) -> MalType {
        if let _MalType::Lambda(
            Lambda {
                ref env,
                ref args,
                ref body,
                ..
            },
            ref meta,
        ) = *self.0
        {
            MalType::lambda_with_meta(
                Lambda {
                    env: env.clone(),
                    args: args.clone(),
                    body: body.clone(),
                    is_macro: true,
                },
                (**meta).clone(),
            )
        } else {
            panic!("Not a lambda!");
        }
    }

    pub fn is_lambda(&self) -> bool {
        match *self.0 {
            _MalType::Lambda { .. } => true,
            _ => false,
        }
    }

    pub fn atom(val: MalType) -> MalType {
        MalType(Rc::new(_MalType::Atom(Rc::new(RefCell::new(val)))))
    }

    pub fn atom_val(&self) -> Option<&Rc<RefCell<MalType>>> {
        match *self.0 {
            _MalType::Atom(ref val) => Some(val),
            _ => None,
        }
    }

    pub fn is_atom(&self) -> bool {
        match *self.0 {
            _MalType::Atom(_) => true,
            _ => false,
        }
    }

    pub fn get_metadata(&self) -> Option<&MalType> {
        match *self.0 {
            _MalType::List(_, ref meta)
            | _MalType::Vector(_, ref meta)
            | _MalType::HashMap(_, ref meta)
            | _MalType::Function(_, ref meta)
            | _MalType::Lambda(_, ref meta) => Some(meta),
            _ => None,
        }
    }

    pub fn clone_with_meta(&mut self, meta: MalType) -> MalType {
        match *self.0 {
            _MalType::List(ref vec, _) => MalType::list_with_meta(vec.clone(), meta),
            _MalType::Vector(ref vec, _) => MalType::vector_with_meta(vec.clone(), meta),
            _MalType::HashMap(ref map, _) => MalType::hashmap_with_meta(map.clone(), meta),
            _MalType::Function(
                Function {
                    ref func, ref env, ..
                },
                _,
            ) => MalType::function_with_meta(
                Function {
                    func: func.clone(),
                    env: env.clone(),
                },
                meta,
            ),
            _MalType::Lambda(
                Lambda {
                    ref env,
                    ref args,
                    ref body,
                    ref is_macro,
                },
                _,
            ) => MalType::lambda_with_meta(
                Lambda {
                    env: env.clone(),
                    args: args.clone(),
                    body: body.clone(),
                    is_macro: *is_macro,
                },
                meta,
            ),
            _ => panic!("not a type with metadata"),
        }
    }

    pub fn swap(&mut self, func: MalType, args: &mut Vec<MalType>) -> MalResult {
        if let _MalType::Atom(ref val) = *self.0 {
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
        if self.is_nil() && other.is_nil() {
            true
        } else if self.is_true() && other.is_true() {
            true
        } else if self.is_false() && other.is_false() {
            true
        } else if let (Some(a), Some(b)) = (self.number_val(), other.number_val()) {
            a == b
        } else if let (Some(a), Some(b)) = (self.keyword_val(), other.keyword_val()) {
            a == b
        } else if let (Some(a), Some(b)) = (self.string_val(), other.string_val()) {
            a == b
        } else if let (Some(a), Some(b)) = (self.symbol_val(), other.symbol_val()) {
            a == b
        } else if let (Some(a), Some(b)) = (self.list_val(), other.list_val()) {
            a == b
        } else if let (Some(a), Some(b)) = (self.vector_val(), other.vector_val()) {
            a == b
        } else if let (Some(a), Some(b)) = (self.hashmap_val(), other.hashmap_val()) {
            a == b
        } else {
            false
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
