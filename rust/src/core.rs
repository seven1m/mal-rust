use types::*;
use printer::pr_str;
use reader::read_str;
use env::Env;
use util::*;
use readline::Readline;

use std::fs::File;
use std::io::prelude::*;
use std::collections::HashMap;
use std::collections::BTreeMap;

use time::get_time;

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
        ns.insert("sequential?".to_string(), is_sequential);
        ns.insert("readline".to_string(), readline);
        ns.insert("meta".to_string(), meta);
        ns.insert("with-meta".to_string(), with_meta);
        ns.insert("string?".to_string(), is_string);
        ns.insert("number?".to_string(), is_number);
        ns.insert("fn?".to_string(), is_fn);
        ns.insert("macro?".to_string(), is_macro);
        ns.insert("conj".to_string(), conj);
        ns.insert("seq".to_string(), seq);
        ns.insert("gensym".to_string(), gensym);
        ns.insert("time-ms".to_string(), time_ms);
        ns
    };
}

fn assert_arg_count_gte(args: &Vec<MalType>, expected: usize, name: &str) -> Result<(), MalError> {
    let actual = args.len();
    if actual >= expected {
        Ok(())
    } else {
        Err(MalError::WrongArguments(
            format!(
                "Must pass at least {} argument(s) to {} but only got {}",
                expected, name, actual
            ).to_string(),
        ))
    }
}

pub fn add(args: &mut Vec<MalType>, _env: Option<Env>) -> MalResult {
    assert_arg_count_gte(args, 1, "+")?;
    let mut iter = MalNumberIter { items: args };
    let mut answer = iter.next().unwrap()?;
    for num in iter {
        answer += num?;
    }
    Ok(MalType::Number(answer))
}

pub fn subtract(args: &mut Vec<MalType>, _env: Option<Env>) -> MalResult {
    assert_arg_count_gte(args, 1, "-")?;
    let mut iter = MalNumberIter { items: args };
    let mut answer = iter.next().unwrap()?;
    for num in iter {
        answer -= num?;
    }
    Ok(MalType::Number(answer))
}

pub fn multiply(args: &mut Vec<MalType>, _env: Option<Env>) -> MalResult {
    assert_arg_count_gte(args, 1, "*")?;
    let mut iter = MalNumberIter { items: args };
    let mut answer = iter.next().unwrap()?;
    for num in iter {
        answer *= num?;
    }
    Ok(MalType::Number(answer))
}

pub fn divide(args: &mut Vec<MalType>, _env: Option<Env>) -> MalResult {
    assert_arg_count_gte(args, 1, "/")?;
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
    Ok(MalType::list(args.clone()))
}

fn mal_bool(b: bool) -> MalType {
    if b {
        MalType::True
    } else {
        MalType::False
    }
}

fn is_list(args: &mut Vec<MalType>, _env: Option<Env>) -> MalResult {
    assert_arg_count_gte(args, 1, "list?")?;
    let arg = args.remove(0);
    if let MalType::List(_, _) = arg {
        Ok(MalType::True)
    } else {
        Ok(MalType::False)
    }
}

fn vector(args: &mut Vec<MalType>, _env: Option<Env>) -> MalResult {
    Ok(MalType::vector(args.clone()))
}

fn is_vector(args: &mut Vec<MalType>, _env: Option<Env>) -> MalResult {
    assert_arg_count_gte(args, 1, "vector?")?;
    let arg = args.remove(0);
    if let MalType::Vector(_, _) = arg {
        Ok(MalType::True)
    } else {
        Ok(MalType::False)
    }
}

fn is_empty(args: &mut Vec<MalType>, _env: Option<Env>) -> MalResult {
    assert_arg_count_gte(args, 1, "empty?")?;
    let arg = args.remove(0);
    let vec = raw_vec(&arg)?;
    Ok(mal_bool(vec.len() == 0))
}

fn count(args: &mut Vec<MalType>, _env: Option<Env>) -> MalResult {
    assert_arg_count_gte(args, 1, "count")?;
    let arg = args.remove(0);
    let len = match arg {
        MalType::List(vec, _) | MalType::Vector(vec, _) => vec.len(),
        MalType::Nil => 0,
        _ => {
            return Err(MalError::WrongArguments(
                "Must pass a list or vector to count".to_string(),
            ))
        }
    };
    Ok(MalType::Number(len as i64))
}

fn is_hash_map(val: &MalType) -> bool {
    match *val {
        MalType::HashMap(_, _) => true,
        _ => false,
    }
}

fn are_lists_equal(list1: &MalType, list2: &MalType) -> bool {
    match (list1, list2) {
        (&MalType::List(ref vec1, _), &MalType::List(ref vec2, _))
        | (&MalType::List(ref vec1, _), &MalType::Vector(ref vec2, _))
        | (&MalType::Vector(ref vec1, _), &MalType::List(ref vec2, _))
        | (&MalType::Vector(ref vec1, _), &MalType::Vector(ref vec2, _)) => {
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
    if _is_sequential(&val1) && _is_sequential(&val2) {
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

fn read_string(args: &mut Vec<MalType>, _env: Option<Env>) -> MalResult {
    assert_arg_count_gte(args, 1, "read-string")?;
    if let MalType::String(code) = args.remove(0) {
        read_str(&code)
    } else {
        Err(MalError::WrongArguments(
            "Must pass a string to read_string".to_string(),
        ))
    }
}

fn slurp(args: &mut Vec<MalType>, _env: Option<Env>) -> MalResult {
    assert_arg_count_gte(args, 1, "slurp")?;
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
}

fn atom(args: &mut Vec<MalType>, _env: Option<Env>) -> MalResult {
    assert_arg_count_gte(args, 1, "atom")?;
    Ok(MalType::atom(args.remove(0)))
}

fn is_atom(args: &mut Vec<MalType>, _env: Option<Env>) -> MalResult {
    assert_arg_count_gte(args, 1, "atom?")?;
    if let MalType::Atom(_) = args.remove(0) {
        Ok(MalType::True)
    } else {
        Ok(MalType::False)
    }
}

fn deref(args: &mut Vec<MalType>, _env: Option<Env>) -> MalResult {
    assert_arg_count_gte(args, 1, "deref")?;
    if let MalType::Atom(val) = args.remove(0) {
        Ok(val.borrow().clone())
    } else {
        Err(MalError::WrongArguments(
            "Must pass an atom to deref".to_string(),
        ))
    }
}

fn reset(args: &mut Vec<MalType>, _env: Option<Env>) -> MalResult {
    assert_arg_count_gte(args, 2, "reset!")?;
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
}

fn swap(args: &mut Vec<MalType>, _env: Option<Env>) -> MalResult {
    assert_arg_count_gte(args, 2, "swap")?;
    let mut atom = args.remove(0);
    let func = args.remove(0);
    atom.swap(func, args)
}

fn cons(args: &mut Vec<MalType>, _env: Option<Env>) -> MalResult {
    assert_arg_count_gte(args, 2, "cons")?;
    let item = args.remove(0);
    let list = args.remove(0);
    let mut vec = raw_vec(&list)?;
    vec.insert(0, item);
    Ok(MalType::list(vec))
}

fn concat(args: &mut Vec<MalType>, _env: Option<Env>) -> MalResult {
    let mut result = vec![];
    while args.len() > 0 {
        let vec = raw_vec(&args.remove(0))?;
        for item in vec {
            result.push(item);
        }
    }
    Ok(MalType::list(result))
}

fn nth(args: &mut Vec<MalType>, _env: Option<Env>) -> MalResult {
    assert_arg_count_gte(args, 2, "nth")?;
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
}

fn first(args: &mut Vec<MalType>, _env: Option<Env>) -> MalResult {
    assert_arg_count_gte(args, 1, "first")?;
    let list = args.remove(0);
    match list {
        MalType::List(vec, _) | MalType::Vector(vec, _) => {
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
}

fn rest(args: &mut Vec<MalType>, _env: Option<Env>) -> MalResult {
    assert_arg_count_gte(args, 1, "rest")?;
    let list = args.remove(0);
    match list {
        MalType::List(mut vec, _) | MalType::Vector(mut vec, _) => {
            if vec.len() > 0 {
                vec.remove(0);
                Ok(MalType::list(vec))
            } else {
                Ok(MalType::list(vec![]))
            }
        }
        MalType::Nil => Ok(MalType::list(vec![])),
        _ => Err(MalError::WrongArguments(
            format!("Expected a list passed to rest but got: {:?}", list).to_string(),
        )),
    }
}

fn throw(args: &mut Vec<MalType>, _env: Option<Env>) -> MalResult {
    assert_arg_count_gte(args, 1, "throw")?;
    let val = args.remove(0);
    Err(MalError::Generic(val))
}

fn apply(args: &mut Vec<MalType>, _env: Option<Env>) -> MalResult {
    assert_arg_count_gte(args, 2, "apply")?;
    let func = args.remove(0);
    let last_index = args.len() - 1;
    let list = args.remove(last_index);
    for item in raw_vec(&list)? {
        args.push(item);
    }
    eval_func(func, args)
}

fn map(args: &mut Vec<MalType>, _env: Option<Env>) -> MalResult {
    assert_arg_count_gte(args, 2, "map")?;
    let lambda = args.remove(0);
    let list = args.remove(0);
    let mut result_list = vec![];
    for item in raw_vec(&list)? {
        let mut args = vec![item];
        let result = eval_func(lambda.clone(), &mut args)?;
        result_list.push(result);
    }
    Ok(MalType::list(result_list))
}

fn is_nil(args: &mut Vec<MalType>, _env: Option<Env>) -> MalResult {
    assert_arg_count_gte(args, 1, "nil?")?;
    match args.remove(0) {
        MalType::Nil => Ok(MalType::True),
        _ => Ok(MalType::False),
    }
}

fn is_true(args: &mut Vec<MalType>, _env: Option<Env>) -> MalResult {
    assert_arg_count_gte(args, 1, "true?")?;
    match args.remove(0) {
        MalType::True => Ok(MalType::True),
        _ => Ok(MalType::False),
    }
}

fn is_false(args: &mut Vec<MalType>, _env: Option<Env>) -> MalResult {
    assert_arg_count_gte(args, 1, "false?")?;
    match args.remove(0) {
        MalType::False => Ok(MalType::True),
        _ => Ok(MalType::False),
    }
}

fn symbol(args: &mut Vec<MalType>, _env: Option<Env>) -> MalResult {
    assert_arg_count_gte(args, 1, "symbol")?;
    if let MalType::String(name) = args.remove(0) {
        Ok(MalType::Symbol(name))
    } else {
        Err(MalError::WrongArguments(
            "Must pass a string to symbol".to_string(),
        ))
    }
}

fn is_symbol(args: &mut Vec<MalType>, _env: Option<Env>) -> MalResult {
    assert_arg_count_gte(args, 1, "symbol?")?;
    match args.remove(0) {
        MalType::Symbol(_) => Ok(MalType::True),
        _ => Ok(MalType::False),
    }
}

fn keyword(args: &mut Vec<MalType>, _env: Option<Env>) -> MalResult {
    assert_arg_count_gte(args, 1, "keyword")?;
    let arg = args.remove(0);
    match arg {
        MalType::String(name) => Ok(MalType::Keyword(name)),
        MalType::Keyword(_) => Ok(arg),
        _ => Err(MalError::WrongArguments(
            "Must pass a string to keyword".to_string(),
        )),
    }
}

fn is_keyword(args: &mut Vec<MalType>, _env: Option<Env>) -> MalResult {
    assert_arg_count_gte(args, 1, "keyword?")?;
    match args.remove(0) {
        MalType::Keyword(_) => Ok(MalType::True),
        _ => Ok(MalType::False),
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
    assert_arg_count_gte(args, 1, "map?")?;
    match args.remove(0) {
        MalType::HashMap(_, _) => Ok(MalType::True),
        _ => Ok(MalType::False),
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
    assert_arg_count_gte(args, 1, "dissoc")?;
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
}

fn get(args: &mut Vec<MalType>, _env: Option<Env>) -> MalResult {
    assert_arg_count_gte(args, 2, "get")?;
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
}

fn contains(args: &mut Vec<MalType>, _env: Option<Env>) -> MalResult {
    assert_arg_count_gte(args, 2, "contains")?;
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
}

fn keys(args: &mut Vec<MalType>, _env: Option<Env>) -> MalResult {
    assert_arg_count_gte(args, 1, "keys")?;
    let map = args.remove(0);
    if let MalType::HashMap(map, _) = map {
        let list = map.keys().map(|k| k.clone()).collect();
        Ok(MalType::list(list))
    } else {
        Err(MalError::WrongArguments(
            "First argument must be a hash-map".to_string(),
        ))
    }
}

fn vals(args: &mut Vec<MalType>, _env: Option<Env>) -> MalResult {
    assert_arg_count_gte(args, 1, "vals")?;
    let map = args.remove(0);
    if let MalType::HashMap(map, _) = map {
        let list = map.values().map(|k| k.clone()).collect();
        Ok(MalType::list(list))
    } else {
        Err(MalError::WrongArguments(
            "First argument must be a hash-map".to_string(),
        ))
    }
}

fn _is_sequential(val: &MalType) -> bool {
    match *val {
        MalType::List(_, _) | MalType::Vector(_, _) => true,
        _ => false,
    }
}

fn is_sequential(args: &mut Vec<MalType>, _env: Option<Env>) -> MalResult {
    assert_arg_count_gte(args, 1, "sequential?")?;
    let arg = args.remove(0);
    Ok(mal_bool(_is_sequential(&arg)))
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
    assert_arg_count_gte(args, 1, "meta")?;
    let arg = args.remove(0);
    match arg {
        MalType::Lambda { metadata, .. }
        | MalType::Function { metadata, .. }
        | MalType::List(_, metadata)
        | MalType::Vector(_, metadata)
        | MalType::HashMap(_, metadata) => Ok(*metadata),
        _ => Ok(MalType::Nil),
    }
}

fn with_meta(args: &mut Vec<MalType>, _env: Option<Env>) -> MalResult {
    assert_arg_count_gte(args, 2, "with-meta")?;
    let mut func = args.remove(0).clone();
    let new_metadata = args.remove(0);
    match func {
        MalType::Lambda {
            ref mut metadata, ..
        }
        | MalType::Function {
            ref mut metadata, ..
        }
        | MalType::List(_, ref mut metadata)
        | MalType::Vector(_, ref mut metadata)
        | MalType::HashMap(_, ref mut metadata) => {
            *metadata = Box::new(new_metadata.clone());
        }
        _ => {}
    };
    Ok(func)
}

fn is_string(args: &mut Vec<MalType>, _env: Option<Env>) -> MalResult {
    assert_arg_count_gte(args, 1, "string?")?;
    match args.remove(0) {
        MalType::String(_) => Ok(MalType::True),
        _ => Ok(MalType::False),
    }
}

fn is_number(args: &mut Vec<MalType>, _env: Option<Env>) -> MalResult {
    assert_arg_count_gte(args, 1, "number?")?;
    match args.remove(0) {
        MalType::Number(_) => Ok(MalType::True),
        _ => Ok(MalType::False),
    }
}

fn is_fn(args: &mut Vec<MalType>, _env: Option<Env>) -> MalResult {
    assert_arg_count_gte(args, 1, "fn?")?;
    match args.remove(0) {
        MalType::Function { .. } => Ok(MalType::True),
        MalType::Lambda { is_macro, .. } => Ok(mal_bool(!is_macro)),
        _ => Ok(MalType::False),
    }
}

fn is_macro(args: &mut Vec<MalType>, _env: Option<Env>) -> MalResult {
    assert_arg_count_gte(args, 1, "macro?")?;
    match args.remove(0) {
        MalType::Lambda { is_macro, .. } => Ok(mal_bool(is_macro)),
        _ => Ok(MalType::False),
    }
}

fn conj(args: &mut Vec<MalType>, _env: Option<Env>) -> MalResult {
    assert_arg_count_gte(args, 2, "conj")?;
    let list = args.remove(0);
    match list {
        MalType::List(mut vec, _) => {
            for new_item in args {
                vec.insert(0, new_item.clone());
            }
            Ok(MalType::list(vec))
        }
        MalType::Vector(mut vec, _) => {
            for new_item in args {
                vec.push(new_item.clone());
            }
            Ok(MalType::vector(vec))
        }
        _ => Err(MalError::WrongArguments(
            "Must pass a list or vector to conj".to_string(),
        )),
    }
}

fn seq(args: &mut Vec<MalType>, _env: Option<Env>) -> MalResult {
    assert_arg_count_gte(args, 1, "seq")?;
    let arg = args.remove(0);
    match arg {
        MalType::String(string) => {
            if string.len() == 0 {
                Ok(MalType::Nil)
            } else {
                Ok(MalType::list(
                    string
                        .chars()
                        .map(|c| MalType::String(c.to_string()))
                        .collect(),
                ))
            }
        }
        MalType::List(vec, _) | MalType::Vector(vec, _) => {
            if vec.len() == 0 {
                Ok(MalType::Nil)
            } else {
                Ok(MalType::list(vec))
            }
        }
        MalType::Nil => Ok(MalType::Nil),
        _ => Err(MalError::WrongArguments(
            "Must pass a string, list, or vector to seq".to_string(),
        )),
    }
}

fn gensym(_args: &mut Vec<MalType>, env: Option<Env>) -> MalResult {
    let env = env.expect("env must be passed to gensym");
    let mut auto_incr = env.get("*gensym-auto-incr*").unwrap();
    let number = match auto_incr {
        MalType::Atom(ref val) => match *val.borrow_mut() {
            MalType::Number(mut num) => num,
            _ => panic!("not possible"),
        },
        _ => panic!("not possible"),
    };
    let add_fn = MalType::function(Box::new(add), Some(env));
    auto_incr.swap(add_fn, &mut vec![MalType::Number(1)])?;
    let name = "gensym-".to_string() + &number.to_string();
    Ok(MalType::Symbol(name))
}

fn time_ms(_args: &mut Vec<MalType>, _env: Option<Env>) -> MalResult {
    let t = get_time();
    let ms = (t.sec * 1_000) as i64 + (t.nsec / 1_000_000) as i64;
    Ok(MalType::Number(ms))
}

fn eval(mut args: Vec<MalType>, env: &Env) -> MalResult {
    if let Ok(MalType::Function { func, .. }) = env.get("eval") {
        func(&mut args, Some(env.clone()))
    } else {
        panic!("eval not a function!");
    }
}

pub fn eval_func(func: MalType, mut args: &mut Vec<MalType>) -> MalResult {
    match func {
        MalType::Function { func, env, .. } => func(&mut args, env),
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
