use types::*;
use printer::pr_str;
use reader::read_str;
use env::Env;
use util::*;
use readline::Readline;

use std::fs::File;
use std::io::prelude::*;
use std::collections::HashMap;
use std::rc::Rc;
use std::cell::RefCell;
use std::collections::BTreeMap;

lazy_static! {
    pub static ref NS: HashMap<String, fn(&mut Vec<MalType>, Option<Env>) -> MalResult> = {
        let mut ns: HashMap<String, fn(&mut Vec<MalType>, Option<Env>) -> MalResult>
            = HashMap::new();
        ns.insert("+".to_string(), add);
        ns.insert("-".to_string(), subtract);
        ns.insert("*".to_string(), multiply);
        ns.insert("/".to_string(), divide);
        ns.insert("prn".to_string(), prn);
        ns.insert("println".to_string(), println_fn);
        ns.insert("str".to_string(), str_fn);
        ns.insert("pr-str".to_string(), pr_str_fn);
        ns.insert("list".to_string(), list);
        ns.insert("list?".to_string(), is_list);
        ns.insert("vector".to_string(), vector);
        ns.insert("vector?".to_string(), is_vector);
        ns.insert("empty?".to_string(), is_empty);
        ns.insert("count".to_string(), count);
        ns.insert("=".to_string(), is_equal);
        ns.insert("<".to_string(), is_lt);
        ns.insert("<=".to_string(), is_lte);
        ns.insert(">".to_string(), is_gt);
        ns.insert(">=".to_string(), is_gte);
        ns.insert("not".to_string(), not);
        ns.insert("read-string".to_string(), read_string);
        ns.insert("slurp".to_string(), slurp);
        ns.insert("atom".to_string(), atom);
        ns.insert("atom?".to_string(), is_atom);
        ns.insert("deref".to_string(), deref);
        ns.insert("reset!".to_string(), reset);
        ns.insert("swap!".to_string(), swap);
        ns.insert("cons".to_string(), cons);
        ns.insert("concat".to_string(), concat);
        ns.insert("nth".to_string(), nth);
        ns.insert("first".to_string(), first);
        ns.insert("rest".to_string(), rest);
        ns.insert("throw".to_string(), throw);
        ns.insert("apply".to_string(), apply);
        ns.insert("map".to_string(), map);
        ns.insert("nil?".to_string(), is_nil);
        ns.insert("true?".to_string(), is_true);
        ns.insert("false?".to_string(), is_false);
        ns.insert("symbol".to_string(), symbol);
        ns.insert("symbol?".to_string(), is_symbol);
        ns.insert("keyword".to_string(), keyword);
        ns.insert("keyword?".to_string(), is_keyword);
        ns.insert("hash-map".to_string(), hash_map);
        ns.insert("map?".to_string(), is_map);
        ns.insert("assoc".to_string(), assoc);
        ns.insert("dissoc".to_string(), dissoc);
        ns.insert("get".to_string(), get);
        ns.insert("contains?".to_string(), contains);
        ns.insert("keys".to_string(), keys);
        ns.insert("vals".to_string(), vals);
        ns.insert("sequential?".to_string(), is_sequental);
        ns.insert("readline".to_string(), readline);
        ns.insert("meta".to_string(), meta);
        ns.insert("with-meta".to_string(), with_meta);
        ns.insert("string?".to_string(), is_string);
        ns.insert("number?".to_string(), is_number);
        ns.insert("fn?".to_string(), is_fn);
        ns.insert("macro?".to_string(), is_macro);
        ns.insert("conj".to_string(), conj);
        ns.insert("seq".to_string(), seq);
        ns
    };
}

pub fn add(args: &mut Vec<MalType>, _env: Option<Env>) -> MalResult {
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

pub fn subtract(args: &mut Vec<MalType>, _env: Option<Env>) -> MalResult {
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

pub fn multiply(args: &mut Vec<MalType>, _env: Option<Env>) -> MalResult {
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

pub fn divide(args: &mut Vec<MalType>, _env: Option<Env>) -> MalResult {
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

fn println_fn(args: &mut Vec<MalType>, _env: Option<Env>) -> MalResult {
    _println(args, false, " ")
}

fn prn(args: &mut Vec<MalType>, _env: Option<Env>) -> MalResult {
    _println(args, true, " ")
}

fn _str_fn(args: &mut Vec<MalType>, print_readably: bool, joiner: &str) -> MalResult {
    let results: Vec<String> = args.iter().map(|arg| pr_str(arg, print_readably)).collect();
    Ok(MalType::String(results.join(joiner)))
}

fn str_fn(args: &mut Vec<MalType>, _env: Option<Env>) -> MalResult {
    _str_fn(args, false, "")
}

fn pr_str_fn(args: &mut Vec<MalType>, _env: Option<Env>) -> MalResult {
    _str_fn(args, true, " ")
}

fn list(args: &mut Vec<MalType>, _env: Option<Env>) -> MalResult {
    Ok(MalType::List(args.clone()))
}

fn mal_bool(b: bool) -> MalType {
    if b {
        MalType::True
    } else {
        MalType::False
    }
}

fn is_list(args: &mut Vec<MalType>, _env: Option<Env>) -> MalResult {
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

fn vector(args: &mut Vec<MalType>, _env: Option<Env>) -> MalResult {
    Ok(MalType::Vector(args.clone()))
}

fn is_vector(args: &mut Vec<MalType>, _env: Option<Env>) -> MalResult {
    if args.len() > 0 {
        let arg = args.remove(0);
        if let MalType::Vector(_) = arg {
            Ok(MalType::True)
        } else {
            Ok(MalType::False)
        }
    } else {
        Err(MalError::WrongArguments(
            "Must pass at least one argument to vector?".to_string(),
        ))
    }
}

fn is_empty(args: &mut Vec<MalType>, _env: Option<Env>) -> MalResult {
    if args.len() > 0 {
        let arg = args.remove(0);
        let vec = raw_vec(&arg)?;
        Ok(mal_bool(vec.len() == 0))
    } else {
        Err(MalError::WrongArguments(
            "Must pass at least one argument to empty?".to_string(),
        ))
    }
}

fn count(args: &mut Vec<MalType>, _env: Option<Env>) -> MalResult {
    if args.len() > 0 {
        let arg = args.remove(0);
        let len = match arg {
            MalType::List(_) | MalType::Vector(_) => {
                let vec = raw_vec(&arg)?;
                vec.len()
            }
            MalType::Nil => 0,
            _ => {
                return Err(MalError::WrongArguments(
                    "Must pass a list or vector to count".to_string(),
                ))
            }
        };
        Ok(MalType::Number(len as i64))
    } else {
        Err(MalError::WrongArguments(
            "Must pass at least one argument to count".to_string(),
        ))
    }
}

fn is_list_like(val: &MalType) -> bool {
    match *val {
        MalType::List(_) | MalType::Vector(_) => true,
        _ => false,
    }
}

fn is_hash_map(val: &MalType) -> bool {
    match *val {
        MalType::HashMap(_, _) => true,
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

fn are_hash_maps_equal(map1: &MalType, map2: &MalType) -> bool {
    match (map1, map2) {
        (&MalType::HashMap(ref map1, _), &MalType::HashMap(ref map2, _)) => {
            if map1.len() == map2.len() {
                for (key1, item1) in map1.iter() {
                    if let Some(ref item2) = map2.get(key1) {
                        if !is_equal_bool(item1, item2) {
                            return false;
                        }
                    } else {
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
    } else if is_hash_map(&val1) && is_hash_map(&val2) {
        are_hash_maps_equal(&val1, &val2)
    } else {
        val1 == val2
    }
}

fn is_equal(args: &mut Vec<MalType>, _env: Option<Env>) -> MalResult {
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
        let n1 = raw_num(&args.remove(0))?;
        let n2 = raw_num(&args.remove(0))?;
        Ok(mal_bool(compare(n1, n2)))
    } else {
        Err(MalError::WrongArguments(
            "Must pass exactly two arguments to compare".to_string(),
        ))
    }
}

fn is_lt(args: &mut Vec<MalType>, _env: Option<Env>) -> MalResult {
    num_compare(args, &|n1, n2| n1 < n2)
}

fn is_lte(args: &mut Vec<MalType>, _env: Option<Env>) -> MalResult {
    num_compare(args, &|n1, n2| n1 <= n2)
}

fn is_gt(args: &mut Vec<MalType>, _env: Option<Env>) -> MalResult {
    num_compare(args, &|n1, n2| n1 > n2)
}

fn is_gte(args: &mut Vec<MalType>, _env: Option<Env>) -> MalResult {
    num_compare(args, &|n1, n2| n1 >= n2)
}

fn not(args: &mut Vec<MalType>, _env: Option<Env>) -> MalResult {
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

fn read_string(args: &mut Vec<MalType>, _env: Option<Env>) -> MalResult {
    if args.len() > 0 {
        if let MalType::String(code) = args.remove(0) {
            read_str(&code)
        } else {
            Err(MalError::WrongArguments(
                "Must pass a string to read_string".to_string(),
            ))
        }
    } else {
        Err(MalError::WrongArguments(
            "Must pass at least one argument to read_string".to_string(),
        ))
    }
}

fn slurp(args: &mut Vec<MalType>, _env: Option<Env>) -> MalResult {
    if args.len() > 0 {
        if let MalType::String(path) = args.remove(0) {
            let mut file = File::open(path)?;
            let mut contents = String::new();
            file.read_to_string(&mut contents)?;
            Ok(MalType::String(contents))
        } else {
            Err(MalError::WrongArguments(
                "Must pass a string to slurp".to_string(),
            ))
        }
    } else {
        Err(MalError::WrongArguments(
            "Must pass at least one argument to slurp".to_string(),
        ))
    }
}

fn atom(args: &mut Vec<MalType>, _env: Option<Env>) -> MalResult {
    if args.len() > 0 {
        Ok(MalType::Atom(Rc::new(RefCell::new(args.remove(0)))))
    } else {
        Err(MalError::WrongArguments(
            "Must pass at least one argument to atom".to_string(),
        ))
    }
}

fn is_atom(args: &mut Vec<MalType>, _env: Option<Env>) -> MalResult {
    if args.len() > 0 {
        if let MalType::Atom(_) = args.remove(0) {
            Ok(MalType::True)
        } else {
            Ok(MalType::False)
        }
    } else {
        Err(MalError::WrongArguments(
            "Must pass at least one argument to atom?".to_string(),
        ))
    }
}

fn deref(args: &mut Vec<MalType>, _env: Option<Env>) -> MalResult {
    if args.len() > 0 {
        if let MalType::Atom(val) = args.remove(0) {
            Ok(val.borrow().clone())
        } else {
            Err(MalError::WrongArguments(
                "Must pass an atom to deref".to_string(),
            ))
        }
    } else {
        Err(MalError::WrongArguments(
            "Must pass at least one argument to deref".to_string(),
        ))
    }
}

fn reset(args: &mut Vec<MalType>, _env: Option<Env>) -> MalResult {
    if args.len() > 1 {
        let mut atom = args.remove(0);
        let new_val = args.remove(0);
        if let MalType::Atom(ref mut val) = atom {
            val.replace(new_val.clone());
            Ok(new_val)
        } else {
            Err(MalError::WrongArguments(
                "Must pass an atom to reset".to_string(),
            ))
        }
    } else {
        Err(MalError::WrongArguments(
            "Must pass at least two arguments to reset!".to_string(),
        ))
    }
}

fn swap(mut args: &mut Vec<MalType>, env: Option<Env>) -> MalResult {
    let top_env = env.expect("Expected Env passed to swap fn");
    if args.len() > 1 {
        let mut atom = args.remove(0);
        let func = args.remove(0);
        if let MalType::Atom(ref mut val) = atom {
            args.insert(0, val.borrow().to_owned());
            if let Ok(MalType::Function(eval_fn, _)) = top_env.get("eval") {
                match func {
                    MalType::Lambda {
                        env,
                        args: binds,
                        mut body,
                        ..
                    } => {
                        let env = Env::with_binds(Some(&env), binds, args.to_owned());
                        let expr = body.remove(0);
                        let mut eval_args = vec![expr];
                        let new_val = eval_fn(&mut eval_args, Some(env))?;
                        val.replace(new_val.clone());
                        Ok(new_val)
                    }
                    MalType::Function(func, env) => {
                        let new_val = func(&mut args, env)?;
                        val.replace(new_val.clone());
                        Ok(new_val)
                    }
                    _ => Err(MalError::WrongArguments(
                        "Must pass a function to reset".to_string(),
                    )),
                }
            } else {
                panic!("eval not a function!");
            }
        } else {
            Err(MalError::WrongArguments(
                "Must pass an atom to reset".to_string(),
            ))
        }
    } else {
        Err(MalError::WrongArguments(
            "Must pass at least two arguments to swap!".to_string(),
        ))
    }
}

fn cons(args: &mut Vec<MalType>, _env: Option<Env>) -> MalResult {
    if args.len() >= 2 {
        let item = args.remove(0);
        let list = args.remove(0);
        let mut vec = raw_vec(&list)?;
        vec.insert(0, item);
        Ok(MalType::List(vec))
    } else {
        Err(MalError::WrongArguments(
            "Must pass at least two arguments to cons".to_string(),
        ))
    }
}

fn concat(args: &mut Vec<MalType>, _env: Option<Env>) -> MalResult {
    let mut result = vec![];
    while args.len() > 0 {
        let vec = raw_vec(&args.remove(0))?;
        for item in vec {
            result.push(item);
        }
    }
    Ok(MalType::List(result))
}

fn nth(args: &mut Vec<MalType>, _env: Option<Env>) -> MalResult {
    if args.len() >= 2 {
        let list = args.remove(0);
        let index = raw_num(&args.remove(0))? as usize;
        let vec = raw_vec(&list)?;
        if vec.len() > index {
            Ok(vec[index].clone())
        } else {
            Err(MalError::IndexOutOfBounds {
                size: vec.len(),
                index,
            })
        }
    } else {
        Err(MalError::WrongArguments(
            "Must pass at least two arguments to nth".to_string(),
        ))
    }
}

fn first(args: &mut Vec<MalType>, _env: Option<Env>) -> MalResult {
    if args.len() >= 1 {
        let list = args.remove(0);
        match list {
            MalType::List(vec) | MalType::Vector(vec) => {
                if vec.len() > 0 {
                    Ok(vec[0].clone())
                } else {
                    Ok(MalType::Nil)
                }
            }
            MalType::Nil => Ok(MalType::Nil),
            _ => Err(MalError::WrongArguments(
                format!("Expected a list passed to first but got: {:?}", list).to_string(),
            )),
        }
    } else {
        Err(MalError::WrongArguments(
            "Must pass at least one argument to first".to_string(),
        ))
    }
}

fn rest(args: &mut Vec<MalType>, _env: Option<Env>) -> MalResult {
    if args.len() >= 1 {
        let list = args.remove(0);
        match list {
            MalType::List(mut vec) | MalType::Vector(mut vec) => {
                if vec.len() > 0 {
                    vec.remove(0);
                    Ok(MalType::List(vec))
                } else {
                    Ok(MalType::List(vec![]))
                }
            }
            MalType::Nil => Ok(MalType::List(vec![])),
            _ => Err(MalError::WrongArguments(
                format!("Expected a list passed to rest but got: {:?}", list).to_string(),
            )),
        }
    } else {
        Err(MalError::WrongArguments(
            "Must pass at least one argument to rest".to_string(),
        ))
    }
}

fn throw(args: &mut Vec<MalType>, _env: Option<Env>) -> MalResult {
    if args.len() >= 1 {
        let val = args.remove(0);
        Err(MalError::Generic(val))
    } else {
        Err(MalError::WrongArguments(
            "Must pass at least one argument to throw".to_string(),
        ))
    }
}

fn apply(args: &mut Vec<MalType>, _env: Option<Env>) -> MalResult {
    if args.len() >= 2 {
        let func = args.remove(0);
        let last_index = args.len() - 1;
        let list = args.remove(last_index);
        for item in raw_vec(&list)? {
            args.push(item);
        }
        eval_func(func, args)
    } else {
        Err(MalError::WrongArguments(
            "Must pass at least two arguments to apply".to_string(),
        ))
    }
}

fn map(args: &mut Vec<MalType>, _env: Option<Env>) -> MalResult {
    if args.len() >= 2 {
        let lambda = args.remove(0);
        let list = args.remove(0);
        let mut result_list = vec![];
        for item in raw_vec(&list)? {
            let mut args = vec![item];
            let result = eval_func(lambda.clone(), &mut args)?;
            result_list.push(result);
        }
        Ok(MalType::List(result_list))
    } else {
        Err(MalError::WrongArguments(
            "Must pass at least two arguments to map".to_string(),
        ))
    }
}

fn is_nil(args: &mut Vec<MalType>, _env: Option<Env>) -> MalResult {
    if args.len() > 0 {
        match args.remove(0) {
            MalType::Nil => Ok(MalType::True),
            _ => Ok(MalType::False),
        }
    } else {
        Err(MalError::WrongArguments(
            "Must pass at least one argument to nil?".to_string(),
        ))
    }
}

fn is_true(args: &mut Vec<MalType>, _env: Option<Env>) -> MalResult {
    if args.len() > 0 {
        match args.remove(0) {
            MalType::True => Ok(MalType::True),
            _ => Ok(MalType::False),
        }
    } else {
        Err(MalError::WrongArguments(
            "Must pass at least one argument to true?".to_string(),
        ))
    }
}

fn is_false(args: &mut Vec<MalType>, _env: Option<Env>) -> MalResult {
    if args.len() > 0 {
        match args.remove(0) {
            MalType::False => Ok(MalType::True),
            _ => Ok(MalType::False),
        }
    } else {
        Err(MalError::WrongArguments(
            "Must pass at least one argument to false?".to_string(),
        ))
    }
}

fn symbol(args: &mut Vec<MalType>, _env: Option<Env>) -> MalResult {
    if args.len() > 0 {
        if let MalType::String(name) = args.remove(0) {
            Ok(MalType::Symbol(name))
        } else {
            Err(MalError::WrongArguments(
                "Must pass a string to symbol".to_string(),
            ))
        }
    } else {
        Err(MalError::WrongArguments(
            "Must pass at least one argument to symbol".to_string(),
        ))
    }
}

fn is_symbol(args: &mut Vec<MalType>, _env: Option<Env>) -> MalResult {
    if args.len() > 0 {
        match args.remove(0) {
            MalType::Symbol(_) => Ok(MalType::True),
            _ => Ok(MalType::False),
        }
    } else {
        Err(MalError::WrongArguments(
            "Must pass at least one argument to symbol?".to_string(),
        ))
    }
}

fn keyword(args: &mut Vec<MalType>, _env: Option<Env>) -> MalResult {
    if args.len() > 0 {
        let arg = args.remove(0);
        match arg {
            MalType::String(name) => Ok(MalType::Keyword(name)),
            MalType::Keyword(_) => Ok(arg),
            _ => Err(MalError::WrongArguments(
                "Must pass a string to keyword".to_string(),
            )),
        }
    } else {
        Err(MalError::WrongArguments(
            "Must pass at least one argument to keyword".to_string(),
        ))
    }
}

fn is_keyword(args: &mut Vec<MalType>, _env: Option<Env>) -> MalResult {
    if args.len() > 0 {
        match args.remove(0) {
            MalType::Keyword(_) => Ok(MalType::True),
            _ => Ok(MalType::False),
        }
    } else {
        Err(MalError::WrongArguments(
            "Must pass at least one argument to keyword?".to_string(),
        ))
    }
}

fn hash_map(args: &mut Vec<MalType>, _env: Option<Env>) -> MalResult {
    if args.len() % 2 == 0 {
        let mut map = BTreeMap::new();
        let mut list_iter = args.iter();
        loop {
            if let Some(key) = list_iter.next() {
                let val = list_iter.next().unwrap();
                map.insert(key.clone(), val.clone());
            } else {
                break;
            }
        }
        Ok(MalType::hashmap(map))
    } else {
        Err(MalError::WrongArguments(
            "Must pass an even number of arguments to hash-map".to_string(),
        ))
    }
}

fn is_map(args: &mut Vec<MalType>, _env: Option<Env>) -> MalResult {
    if args.len() > 0 {
        match args.remove(0) {
            MalType::HashMap(_, _) => Ok(MalType::True),
            _ => Ok(MalType::False),
        }
    } else {
        Err(MalError::WrongArguments(
            "Must pass at least one argument to map?".to_string(),
        ))
    }
}

fn assoc(args: &mut Vec<MalType>, _env: Option<Env>) -> MalResult {
    if args.len() % 2 == 1 {
        let map = args.remove(0);
        if let MalType::HashMap(map, metadata) = map {
            let mut map = map.clone();
            let mut list_iter = args.iter();
            loop {
                if let Some(key) = list_iter.next() {
                    let val = list_iter.next().unwrap();
                    map.insert(key.clone(), val.clone());
                } else {
                    break;
                }
            }
            Ok(MalType::HashMap(map, metadata))
        } else {
            Err(MalError::WrongArguments(
                "First argument must be a hash-map".to_string(),
            ))
        }
    } else {
        Err(MalError::WrongArguments(
            "Must pass an odd number of arguments to assoc".to_string(),
        ))
    }
}

fn dissoc(args: &mut Vec<MalType>, _env: Option<Env>) -> MalResult {
    if args.len() >= 1 {
        let map = args.remove(0);
        if let MalType::HashMap(map, metadata) = map {
            let mut map = map.clone();
            let mut list_iter = args.iter();
            loop {
                if let Some(key) = list_iter.next() {
                    if map.contains_key(key) {
                        map.remove(key);
                    }
                } else {
                    break;
                }
            }
            Ok(MalType::HashMap(map, metadata))
        } else {
            Err(MalError::WrongArguments(
                "First argument must be a hash-map".to_string(),
            ))
        }
    } else {
        Err(MalError::WrongArguments(
            "Must pass at least one argument to dissoc".to_string(),
        ))
    }
}

fn get(args: &mut Vec<MalType>, _env: Option<Env>) -> MalResult {
    if args.len() >= 2 {
        let map = args.remove(0);
        match map {
            MalType::HashMap(map, _) => {
                let key = args.remove(0);
                match map.get(&key) {
                    Some(val) => Ok(val.clone()),
                    None => Ok(MalType::Nil),
                }
            }
            MalType::Nil => Ok(MalType::Nil),
            _ => Err(MalError::WrongArguments(
                "First argument must be a hash-map".to_string(),
            )),
        }
    } else {
        Err(MalError::WrongArguments(
            "Must pass at least two arguments to get".to_string(),
        ))
    }
}

fn contains(args: &mut Vec<MalType>, _env: Option<Env>) -> MalResult {
    if args.len() >= 2 {
        let map = args.remove(0);
        if let MalType::HashMap(map, _) = map {
            let key = args.remove(0);
            if map.contains_key(&key) {
                Ok(MalType::True)
            } else {
                Ok(MalType::False)
            }
        } else {
            Err(MalError::WrongArguments(
                "First argument must be a hash-map".to_string(),
            ))
        }
    } else {
        Err(MalError::WrongArguments(
            "Must pass at least two arguments to contains".to_string(),
        ))
    }
}

fn keys(args: &mut Vec<MalType>, _env: Option<Env>) -> MalResult {
    if args.len() >= 1 {
        let map = args.remove(0);
        if let MalType::HashMap(map, _) = map {
            let list = map.keys().map(|k| k.clone()).collect();
            Ok(MalType::List(list))
        } else {
            Err(MalError::WrongArguments(
                "First argument must be a hash-map".to_string(),
            ))
        }
    } else {
        Err(MalError::WrongArguments(
            "Must pass at least one argument to keys".to_string(),
        ))
    }
}

fn vals(args: &mut Vec<MalType>, _env: Option<Env>) -> MalResult {
    if args.len() >= 1 {
        let map = args.remove(0);
        if let MalType::HashMap(map, _) = map {
            let list = map.values().map(|k| k.clone()).collect();
            Ok(MalType::List(list))
        } else {
            Err(MalError::WrongArguments(
                "First argument must be a hash-map".to_string(),
            ))
        }
    } else {
        Err(MalError::WrongArguments(
            "Must pass at least one argument to vals".to_string(),
        ))
    }
}

fn is_sequental(args: &mut Vec<MalType>, _env: Option<Env>) -> MalResult {
    if args.len() >= 1 {
        let arg = args.remove(0);
        Ok(mal_bool(is_list_like(&arg)))
    } else {
        Err(MalError::WrongArguments(
            "Must pass at least one argument to sequential?".to_string(),
        ))
    }
}

fn readline(args: &mut Vec<MalType>, _env: Option<Env>) -> MalResult {
    let prompt = if args.len() >= 1 {
        let arg = args.remove(0);
        if let MalType::String(string) = arg {
            string
        } else {
            return Err(MalError::WrongArguments(
                "Must pass a string to readline".to_string(),
            ));
        }
    } else {
        ">".to_string()
    };
    let mut readline = Readline::new(&prompt);
    match readline.get() {
        Some(line) => Ok(MalType::String(line)),
        None => Ok(MalType::Nil),
    }
}

fn meta(args: &mut Vec<MalType>, _env: Option<Env>) -> MalResult {
    if args.len() >= 1 {
        let arg = args.remove(0);
        match arg {
            MalType::Lambda { metadata, .. } => Ok(*metadata),
            MalType::HashMap(_, metadata) => Ok(*metadata),
            _ => Ok(MalType::Nil),
        }
    } else {
        Err(MalError::WrongArguments(
            "Must pass at least one argument to meta".to_string(),
        ))
    }
}

fn with_meta(args: &mut Vec<MalType>, _env: Option<Env>) -> MalResult {
    if args.len() >= 2 {
        let mut func = args.remove(0).clone();
        let new_metadata = args.remove(0);
        match func {
            MalType::Lambda {
                ref mut metadata, ..
            } => {
                *metadata = Box::new(new_metadata.clone());
            }
            MalType::HashMap(_, ref mut metadata) => {
                *metadata = Box::new(new_metadata.clone());
            }
            _ => {}
        };
        Ok(func)
    } else {
        Err(MalError::WrongArguments(
            "Must pass at least two arguments to with-meta".to_string(),
        ))
    }
}

fn is_string(args: &mut Vec<MalType>, _env: Option<Env>) -> MalResult {
    if args.len() > 0 {
        match args.remove(0) {
            MalType::String(_) => Ok(MalType::True),
            _ => Ok(MalType::False),
        }
    } else {
        Err(MalError::WrongArguments(
            "Must pass at least one argument to string?".to_string(),
        ))
    }
}

fn is_number(args: &mut Vec<MalType>, _env: Option<Env>) -> MalResult {
    if args.len() > 0 {
        match args.remove(0) {
            MalType::Number(_) => Ok(MalType::True),
            _ => Ok(MalType::False),
        }
    } else {
        Err(MalError::WrongArguments(
            "Must pass at least one argument to number?".to_string(),
        ))
    }
}

fn is_fn(args: &mut Vec<MalType>, _env: Option<Env>) -> MalResult {
    if args.len() > 0 {
        match args.remove(0) {
            MalType::Function(_, _) => Ok(MalType::True),
            MalType::Lambda { is_macro, .. } => Ok(mal_bool(!is_macro)),
            _ => Ok(MalType::False),
        }
    } else {
        Err(MalError::WrongArguments(
            "Must pass at least one argument to fn?".to_string(),
        ))
    }
}

fn is_macro(args: &mut Vec<MalType>, _env: Option<Env>) -> MalResult {
    if args.len() > 0 {
        match args.remove(0) {
            MalType::Lambda { is_macro, .. } => Ok(mal_bool(is_macro)),
            _ => Ok(MalType::False),
        }
    } else {
        Err(MalError::WrongArguments(
            "Must pass at least one argument to fn?".to_string(),
        ))
    }
}

fn conj(args: &mut Vec<MalType>, _env: Option<Env>) -> MalResult {
    if args.len() >= 2 {
        let list = args.remove(0);
        match list {
            MalType::List(mut vec) => {
                for new_item in args {
                    vec.insert(0, new_item.clone());
                }
                Ok(MalType::List(vec))
            }
            MalType::Vector(mut vec) => {
                for new_item in args {
                    vec.push(new_item.clone());
                }
                Ok(MalType::Vector(vec))
            }
            _ => Err(MalError::WrongArguments(
                "Must pass a list or vector to conj".to_string(),
            )),
        }
    } else {
        Err(MalError::WrongArguments(
            "Must pass at least two arguments to conj".to_string(),
        ))
    }
}

fn seq(args: &mut Vec<MalType>, _env: Option<Env>) -> MalResult {
    if args.len() >= 1 {
        let arg = args.remove(0);
        match arg {
            MalType::String(string) => {
                if string.len() == 0 {
                    Ok(MalType::Nil)
                } else {
                    Ok(MalType::List(
                        string
                            .chars()
                            .map(|c| MalType::String(c.to_string()))
                            .collect(),
                    ))
                }
            }
            MalType::List(vec) | MalType::Vector(vec) => {
                if vec.len() == 0 {
                    Ok(MalType::Nil)
                } else {
                    Ok(MalType::List(vec))
                }
            }
            MalType::Nil => Ok(MalType::Nil),
            _ => Err(MalError::WrongArguments(
                "Must pass a string, list, or vector to seq".to_string(),
            )),
        }
    } else {
        Err(MalError::WrongArguments(
            "Must pass at least one argument to seq".to_string(),
        ))
    }
}

fn eval(mut args: Vec<MalType>, env: &Env) -> MalResult {
    if let Ok(MalType::Function(eval_fn, _)) = env.get("eval") {
        eval_fn(&mut args, Some(env.clone()))
    } else {
        panic!("eval not a function!");
    }
}

fn eval_func(func: MalType, mut args: &mut Vec<MalType>) -> MalResult {
    match func {
        MalType::Function(func, env) => func(&mut args, env),
        MalType::Lambda {
            env,
            args: binds,
            body,
            ..
        } => {
            let inner_env = Env::with_binds(Some(&env), binds, args.clone());
            eval(body, &inner_env)
        }
        _ => Err(MalError::NotAFunction(func)),
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
