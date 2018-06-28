use types::*;
use printer::pr_str;

use std::collections::HashMap;

lazy_static! {
    pub static ref NS: HashMap<String, fn(&mut Vec<MalType>) -> MalResult> = {
        let mut ns = HashMap::new();
        ns.insert("+".to_string(), add as fn(&mut Vec<MalType>) -> MalResult);
        ns.insert("-".to_string(), subtract as fn(&mut Vec<MalType>) -> MalResult);
        ns.insert("*".to_string(), multiply as fn(&mut Vec<MalType>) -> MalResult);
        ns.insert("/".to_string(), divide as fn(&mut Vec<MalType>) -> MalResult);
        ns.insert("prn".to_string(), prn as fn(&mut Vec<MalType>) -> MalResult);
        ns.insert("println".to_string(), println_fn as fn(&mut Vec<MalType>) -> MalResult);
        ns.insert("str".to_string(), str_fn as fn(&mut Vec<MalType>) -> MalResult);
        ns.insert("pr-str".to_string(), pr_str_fn as fn(&mut Vec<MalType>) -> MalResult);
        ns.insert("list".to_string(), list as fn(&mut Vec<MalType>) -> MalResult);
        ns.insert("list?".to_string(), is_list as fn(&mut Vec<MalType>) -> MalResult);
        ns.insert("empty?".to_string(), is_empty as fn(&mut Vec<MalType>) -> MalResult);
        ns.insert("count".to_string(), count as fn(&mut Vec<MalType>) -> MalResult);
        ns.insert("=".to_string(), is_equal as fn(&mut Vec<MalType>) -> MalResult);
        ns.insert("<".to_string(), is_lt as fn(&mut Vec<MalType>) -> MalResult);
        ns.insert("<=".to_string(), is_lte as fn(&mut Vec<MalType>) -> MalResult);
        ns.insert(">".to_string(), is_gt as fn(&mut Vec<MalType>) -> MalResult);
        ns.insert(">=".to_string(), is_gte as fn(&mut Vec<MalType>) -> MalResult);
        ns.insert("not".to_string(), not as fn(&mut Vec<MalType>) -> MalResult);
        ns
    };
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

fn _println(args: &mut Vec<MalType>, print_readably: bool, joiner: &str) -> MalResult {
    let results: Vec<String> = args.iter().map(|arg| pr_str(arg, print_readably)).collect();
    let out = results.join(joiner);
    println!("{}", out);
    Ok(MalType::Nil)
}

fn println_fn(args: &mut Vec<MalType>) -> MalResult {
    _println(args, false, " ")
}

fn prn(args: &mut Vec<MalType>) -> MalResult {
    _println(args, true, " ")
}

fn _str_fn(args: &mut Vec<MalType>, print_readably: bool, joiner: &str) -> MalResult {
    let results: Vec<String> = args.iter().map(|arg| pr_str(arg, print_readably)).collect();
    Ok(MalType::String(results.join(joiner)))
}

fn str_fn(args: &mut Vec<MalType>) -> MalResult {
    _str_fn(args, false, "")
}

fn pr_str_fn(args: &mut Vec<MalType>) -> MalResult {
    _str_fn(args, true, " ")
}

fn list(args: &mut Vec<MalType>) -> MalResult {
    Ok(MalType::List(args.clone()))
}

fn mal_bool(b: bool) -> MalType {
    if b {
        MalType::True
    } else {
        MalType::False
    }
}

fn is_list(args: &mut Vec<MalType>) -> MalResult {
    if args.len() > 0 {
        let arg = args.remove(0);
        if let MalType::List(_) = arg {
            Ok(MalType::True)
        } else {
            Ok(MalType::False)
        }
    } else {
        Err(MalError::WrongArguments(
            "Must pass at least one argument to list?".to_string(),
        ))
    }
}

fn is_empty(args: &mut Vec<MalType>) -> MalResult {
    if args.len() > 0 {
        let arg = args.remove(0);
        match &arg {
            &MalType::List(ref vec) | &MalType::Vector(ref vec) => Ok(mal_bool(vec.len() == 0)),
            _ => Err(MalError::WrongArguments(
                format!("Expected a list but got: {:?}", &arg).to_string(),
            )),
        }
    } else {
        Err(MalError::WrongArguments(
            "Must pass at least one argument to empty?".to_string(),
        ))
    }
}

fn count(args: &mut Vec<MalType>) -> MalResult {
    if args.len() > 0 {
        let arg = args.remove(0);
        match &arg {
            &MalType::List(ref vec) | &MalType::Vector(ref vec) => {
                Ok(MalType::Number(vec.len() as i64))
            }
            &MalType::Nil => Ok(MalType::Number(0)),
            _ => Err(MalError::WrongArguments(
                format!("Expected a list but got: {:?}", &arg).to_string(),
            )),
        }
    } else {
        Err(MalError::WrongArguments(
            "Must pass at least one argument to count".to_string(),
        ))
    }
}

fn is_list_like(val: &MalType) -> bool {
    match val {
        &MalType::List(_) | &MalType::Vector(_) => true,
        _ => false,
    }
}

fn are_lists_equal(list1: &MalType, list2: &MalType) -> bool {
    match (list1, list2) {
        (&MalType::List(ref vec1), &MalType::List(ref vec2))
        | (&MalType::List(ref vec1), &MalType::Vector(ref vec2))
        | (&MalType::Vector(ref vec1), &MalType::List(ref vec2))
        | (&MalType::Vector(ref vec1), &MalType::Vector(ref vec2)) => {
            if vec1.len() == vec2.len() {
                for (index, item1) in vec1.iter().enumerate() {
                    let item2 = &vec2[index];
                    if !is_equal_bool(item1, item2) {
                        return false;
                    }
                }
                true
            } else {
                false
            }
        }
        _ => false,
    }
}

fn is_equal_bool(val1: &MalType, val2: &MalType) -> bool {
    if is_list_like(&val1) && is_list_like(&val2) {
        are_lists_equal(&val1, &val2)
    } else {
        val1 == val2
    }
}

fn is_equal(args: &mut Vec<MalType>) -> MalResult {
    if args.len() == 2 {
        let arg1 = args.remove(0);
        let arg2 = args.remove(0);
        Ok(mal_bool(is_equal_bool(&arg1, &arg2)))
    } else {
        Err(MalError::WrongArguments(
            "Must pass exactly two arguments to =".to_string(),
        ))
    }
}

fn num_compare(args: &mut Vec<MalType>, compare: &Fn(i64, i64) -> bool) -> MalResult {
    if args.len() == 2 {
        let arg1 = args.remove(0);
        let arg2 = args.remove(0);
        if let (&MalType::Number(ref n1), &MalType::Number(ref n2)) = (&arg1, &arg2) {
            Ok(mal_bool(compare(*n1, *n2)))
        } else {
            Err(MalError::WrongArguments(
                format!("Expected numbers but got but got: {:?}, {:?}", &arg1, &arg2).to_string(),
            ))
        }
    } else {
        Err(MalError::WrongArguments(
            "Must pass exactly two arguments to compare".to_string(),
        ))
    }
}

fn is_lt(args: &mut Vec<MalType>) -> MalResult {
    num_compare(args, &|n1, n2| n1 < n2)
}

fn is_lte(args: &mut Vec<MalType>) -> MalResult {
    num_compare(args, &|n1, n2| n1 <= n2)
}

fn is_gt(args: &mut Vec<MalType>) -> MalResult {
    num_compare(args, &|n1, n2| n1 > n2)
}

fn is_gte(args: &mut Vec<MalType>) -> MalResult {
    num_compare(args, &|n1, n2| n1 >= n2)
}

fn not(args: &mut Vec<MalType>) -> MalResult {
    if args.len() > 0 {
        let arg = args.remove(0);
        Ok(match &arg {
            &MalType::False => MalType::True,
            _ => MalType::False,
        })
    } else {
        Err(MalError::WrongArguments(
            "Must pass at least one argument to not".to_string(),
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
