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
    Ok(MalType::number(answer))
}

pub fn subtract(args: &mut Vec<MalType>, _env: Option<Env>) -> MalResult {
    assert_arg_count_gte(args, 1, "-")?;
    let mut iter = MalNumberIter { items: args };
    let mut answer = iter.next().unwrap()?;
    for num in iter {
        answer -= num?;
    }
    Ok(MalType::number(answer))
}

pub fn multiply(args: &mut Vec<MalType>, _env: Option<Env>) -> MalResult {
    assert_arg_count_gte(args, 1, "*")?;
    let mut iter = MalNumberIter { items: args };
    let mut answer = iter.next().unwrap()?;
    for num in iter {
        answer *= num?;
    }
    Ok(MalType::number(answer))
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
    Ok(MalType::number(answer))
}

fn _println(args: &mut Vec<MalType>, print_readably: bool, joiner: &str) -> MalResult {
    let results: Vec<String> = args.iter().map(|arg| pr_str(arg, print_readably)).collect();
    let out = results.join(joiner);
    println!("{}", out);
    Ok(MalType::nil())
}

fn println_fn(args: &mut Vec<MalType>, _env: Option<Env>) -> MalResult {
    _println(args, false, " ")
}

fn prn(args: &mut Vec<MalType>, _env: Option<Env>) -> MalResult {
    _println(args, true, " ")
}

fn _str_fn(args: &mut Vec<MalType>, print_readably: bool, joiner: &str) -> MalResult {
    let results: Vec<String> = args.iter().map(|arg| pr_str(arg, print_readably)).collect();
    Ok(MalType::string(results.join(joiner)))
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
        MalType::bool_true()
    } else {
        MalType::bool_false()
    }
}

fn is_list(args: &mut Vec<MalType>, _env: Option<Env>) -> MalResult {
    assert_arg_count_gte(args, 1, "list?")?;
    Ok(mal_bool(args[0].is_list()))
}

fn vector(args: &mut Vec<MalType>, _env: Option<Env>) -> MalResult {
    Ok(MalType::vector(args.clone()))
}

fn is_vector(args: &mut Vec<MalType>, _env: Option<Env>) -> MalResult {
    assert_arg_count_gte(args, 1, "vector?")?;
    Ok(mal_bool(args[0].is_vector()))
}

fn is_empty(args: &mut Vec<MalType>, _env: Option<Env>) -> MalResult {
    assert_arg_count_gte(args, 1, "empty?")?;
    let arg = args.remove(0);
    let vec = vec_result(&arg)?;
    Ok(mal_bool(vec.len() == 0))
}

fn count(args: &mut Vec<MalType>, _env: Option<Env>) -> MalResult {
    assert_arg_count_gte(args, 1, "count")?;
    let arg = &args[0];
    if let Some(vec) = arg.list_or_vector_val() {
        Ok(MalType::number(vec.len() as i64))
    } else if arg.is_nil() {
        Ok(MalType::number(0))
    } else {
        Err(MalError::WrongArguments(
            "Must pass a list or vector to count".to_string(),
        ))
    }
}

fn are_lists_equal(list1: &MalType, list2: &MalType) -> bool {
    if let Some(vec1) = list1.list_or_vector_val() {
        if let Some(vec2) = list2.list_or_vector_val() {
            if vec1.len() == vec2.len() {
                for (index, item1) in vec1.iter().enumerate() {
                    let item2 = &vec2[index];
                    if !is_equal_bool(item1, item2) {
                        return false;
                    }
                }
                return true;
            }
        }
    }
    false
}

fn are_hash_maps_equal(map1: &MalType, map2: &MalType) -> bool {
    if let Some(map1) = map1.hashmap_val() {
        if let Some(map2) = map2.hashmap_val() {
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
                return true;
            }
        }
    }
    false
}

fn is_equal_bool(val1: &MalType, val2: &MalType) -> bool {
    if val1.is_list_or_vector() && val2.is_list_or_vector() {
        are_lists_equal(&val1, &val2)
    } else if val1.is_hashmap() && val2.is_hashmap() {
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
        let n1 = num_result(&args.remove(0))?;
        let n2 = num_result(&args.remove(0))?;
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
    if let Some(code) = args.remove(0).string_val() {
        read_str(code)
    } else {
        Err(MalError::WrongArguments(
            "Must pass a string to read_string".to_string(),
        ))
    }
}

fn slurp(args: &mut Vec<MalType>, _env: Option<Env>) -> MalResult {
    assert_arg_count_gte(args, 1, "slurp")?;
    if let Some(path) = args.remove(0).string_val() {
        let mut file = File::open(path)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        Ok(MalType::string(contents))
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
    Ok(mal_bool(args.remove(0).is_atom()))
}

fn deref(args: &mut Vec<MalType>, _env: Option<Env>) -> MalResult {
    assert_arg_count_gte(args, 1, "deref")?;
    if let Some(val) = args.remove(0).atom_val() {
        Ok(val.borrow().clone())
    } else {
        Err(MalError::WrongArguments(
            "Must pass an atom to deref".to_string(),
        ))
    }
}

fn reset(args: &mut Vec<MalType>, _env: Option<Env>) -> MalResult {
    assert_arg_count_gte(args, 2, "reset!")?;
    let atom = args.remove(0);
    let new_val = args.remove(0);
    if let Some(ref mut val) = atom.atom_val() {
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
    let mut vec = vec_result(&list)?;
    vec.insert(0, item);
    Ok(MalType::list(vec))
}

fn concat(args: &mut Vec<MalType>, _env: Option<Env>) -> MalResult {
    let mut result = vec![];
    while args.len() > 0 {
        let vec = vec_result(&args.remove(0))?;
        for item in vec {
            result.push(item);
        }
    }
    Ok(MalType::list(result))
}

fn nth(args: &mut Vec<MalType>, _env: Option<Env>) -> MalResult {
    assert_arg_count_gte(args, 2, "nth")?;
    let list = args.remove(0);
    let index = num_result(&args.remove(0))? as usize;
    let vec = vec_result(&list)?;
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
    if let Some(vec) = list.list_or_vector_val() {
        if vec.len() > 0 {
            Ok(vec[0].clone())
        } else {
            Ok(MalType::nil())
        }
    } else if list.is_nil() {
        Ok(MalType::nil())
    } else {
        Err(MalError::WrongArguments(
            format!("Expected a list passed to first but got: {:?}", list).to_string(),
        ))
    }
}

fn rest(args: &mut Vec<MalType>, _env: Option<Env>) -> MalResult {
    assert_arg_count_gte(args, 1, "rest")?;
    let list = args.remove(0);
    if let Some(vec) = list.list_or_vector_val() {
        if vec.len() > 0 {
            Ok(MalType::list(vec[1..].to_owned()))
        } else {
            Ok(MalType::list(vec![]))
        }
    } else if list.is_nil() {
        Ok(MalType::list(vec![]))
    } else {
        Err(MalError::WrongArguments(
            format!("Expected a list passed to rest but got: {:?}", list).to_string(),
        ))
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
    for item in vec_result(&list)? {
        args.push(item);
    }
    eval_func(func, args)
}

fn map(args: &mut Vec<MalType>, _env: Option<Env>) -> MalResult {
    assert_arg_count_gte(args, 2, "map")?;
    let lambda = args.remove(0);
    let list = args.remove(0);
    let mut result_list = vec![];
    for item in vec_result(&list)? {
        let mut args = vec![item];
        let result = eval_func(lambda.clone(), &mut args)?;
        result_list.push(result);
    }
    Ok(MalType::list(result_list))
}

fn is_nil(args: &mut Vec<MalType>, _env: Option<Env>) -> MalResult {
    assert_arg_count_gte(args, 1, "nil?")?;
    Ok(mal_bool(args[0].is_nil()))
}

fn is_true(args: &mut Vec<MalType>, _env: Option<Env>) -> MalResult {
    assert_arg_count_gte(args, 1, "true?")?;
    Ok(mal_bool(args[0].is_true()))
}

fn is_false(args: &mut Vec<MalType>, _env: Option<Env>) -> MalResult {
    assert_arg_count_gte(args, 1, "false?")?;
    Ok(mal_bool(args[0].is_false()))
}

fn symbol(args: &mut Vec<MalType>, _env: Option<Env>) -> MalResult {
    assert_arg_count_gte(args, 1, "symbol")?;
    if let Some(name) = args[0].string_val() {
        Ok(MalType::symbol(name.to_owned()))
    } else {
        Err(MalError::WrongArguments(
            "Must pass a string to symbol".to_string(),
        ))
    }
}

fn is_symbol(args: &mut Vec<MalType>, _env: Option<Env>) -> MalResult {
    assert_arg_count_gte(args, 1, "symbol?")?;
    Ok(mal_bool(args[0].is_symbol()))
}

fn keyword(args: &mut Vec<MalType>, _env: Option<Env>) -> MalResult {
    assert_arg_count_gte(args, 1, "keyword")?;
    if let Some(name) = args[0].string_val() {
        Ok(MalType::keyword(name.to_owned()))
    } else if args[0].is_keyword() {
        Ok(args[0].clone())
    } else {
        Err(MalError::WrongArguments(
            "Must pass a string to keyword".to_string(),
        ))
    }
}

fn is_keyword(args: &mut Vec<MalType>, _env: Option<Env>) -> MalResult {
    assert_arg_count_gte(args, 1, "keyword?")?;
    Ok(mal_bool(args[0].is_keyword()))
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
    Ok(mal_bool(args[0].is_hashmap()))
}

fn assoc(args: &mut Vec<MalType>, _env: Option<Env>) -> MalResult {
    if args.len() % 2 == 1 {
        if let Some(map) = args[0].hashmap_val() {
            let mut map = map.clone();
            let mut list_iter = args.iter().skip(1);
            loop {
                if let Some(key) = list_iter.next() {
                    let val = list_iter.next().unwrap();
                    map.insert(key.clone(), val.clone());
                } else {
                    break;
                }
            }
            let map = MalType::hashmap_with_meta(
                map,
                args[0]
                    .get_metadata()
                    .expect("Expected hashmap to return metadata")
                    .clone(),
            );
            Ok(map)
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
    if let Some(map) = args[0].hashmap_val() {
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
        let map = MalType::hashmap_with_meta(
            map,
            args[0]
                .get_metadata()
                .expect("Expected hashmap to return metadata")
                .clone(),
        );
        Ok(map)
    } else {
        Err(MalError::WrongArguments(
            "First argument must be a hash-map".to_string(),
        ))
    }
}

fn get(args: &mut Vec<MalType>, _env: Option<Env>) -> MalResult {
    assert_arg_count_gte(args, 2, "get")?;
    if let Some(map) = args[0].hashmap_val() {
        let key = &args[1];
        match map.get(key) {
            Some(val) => Ok(val.clone()),
            None => Ok(MalType::nil()),
        }
    } else if args[0].is_nil() {
        Ok(MalType::nil())
    } else {
        Err(MalError::WrongArguments(
            "First argument must be a hash-map".to_string(),
        ))
    }
}

fn contains(args: &mut Vec<MalType>, _env: Option<Env>) -> MalResult {
    assert_arg_count_gte(args, 2, "contains")?;
    if let Some(map) = args[0].hashmap_val() {
        let key = &args[1];
        Ok(mal_bool(map.contains_key(key)))
    } else {
        Err(MalError::WrongArguments(
            "First argument must be a hash-map".to_string(),
        ))
    }
}

fn keys(args: &mut Vec<MalType>, _env: Option<Env>) -> MalResult {
    assert_arg_count_gte(args, 1, "keys")?;
    if let Some(map) = args[0].hashmap_val() {
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
    if let Some(map) = args[0].hashmap_val() {
        let list = map.values().map(|k| k.clone()).collect();
        Ok(MalType::list(list))
    } else {
        Err(MalError::WrongArguments(
            "First argument must be a hash-map".to_string(),
        ))
    }
}

fn is_sequential(args: &mut Vec<MalType>, _env: Option<Env>) -> MalResult {
    assert_arg_count_gte(args, 1, "sequential?")?;
    Ok(mal_bool(args[0].is_list_or_vector()))
}

fn readline(args: &mut Vec<MalType>, _env: Option<Env>) -> MalResult {
    let prompt = if args.len() >= 1 {
        if let Some(string) = args[0].string_val() {
            string
        } else {
            return Err(MalError::WrongArguments(
                "Must pass a string to readline".to_string(),
            ));
        }
    } else {
        ">"
    };
    let mut readline = Readline::new(prompt);
    match readline.get() {
        Some(line) => Ok(MalType::string(line)),
        None => Ok(MalType::nil()),
    }
}

fn meta(args: &mut Vec<MalType>, _env: Option<Env>) -> MalResult {
    assert_arg_count_gte(args, 1, "meta")?;
    if let Some(metadata) = args[0].get_metadata() {
        Ok(metadata.to_owned())
    } else {
        Ok(MalType::nil())
    }
}

fn with_meta(args: &mut Vec<MalType>, _env: Option<Env>) -> MalResult {
    assert_arg_count_gte(args, 2, "with-meta")?;
    let mut func = args[0].clone();
    Ok(func.clone_with_meta(args[1].clone()))
}

fn is_string(args: &mut Vec<MalType>, _env: Option<Env>) -> MalResult {
    assert_arg_count_gte(args, 1, "string?")?;
    Ok(mal_bool(args[0].is_string()))
}

fn is_number(args: &mut Vec<MalType>, _env: Option<Env>) -> MalResult {
    assert_arg_count_gte(args, 1, "number?")?;
    Ok(mal_bool(args[0].is_number()))
}

fn is_fn(args: &mut Vec<MalType>, _env: Option<Env>) -> MalResult {
    assert_arg_count_gte(args, 1, "fn?")?;
    if args[0].is_function() {
        Ok(MalType::bool_true())
    } else if let Some(Lambda { is_macro, .. }) = args[0].lambda_val() {
        Ok(mal_bool(!is_macro))
    } else {
        Ok(MalType::bool_false())
    }
}

fn is_macro(args: &mut Vec<MalType>, _env: Option<Env>) -> MalResult {
    assert_arg_count_gte(args, 1, "macro?")?;
    if let Some(Lambda { is_macro, .. }) = args[0].lambda_val() {
        Ok(mal_bool(*is_macro))
    } else {
        Ok(MalType::bool_false())
    }
}

fn conj(args: &mut Vec<MalType>, _env: Option<Env>) -> MalResult {
    assert_arg_count_gte(args, 2, "conj")?;
    if let Some(vec) = args[0].list_val() {
        let mut vec = vec.to_owned();
        for new_item in args.iter().skip(1) {
            vec.insert(0, new_item.clone());
        }
        Ok(MalType::list(vec))
    } else if let Some(vec) = args[0].vector_val() {
        let mut vec = vec.to_owned();
        for new_item in args.iter().skip(1) {
            vec.push(new_item.clone());
        }
        Ok(MalType::vector(vec))
    } else {
        Err(MalError::WrongArguments(
            "Must pass a list or vector to conj".to_string(),
        ))
    }
}

fn seq(args: &mut Vec<MalType>, _env: Option<Env>) -> MalResult {
    assert_arg_count_gte(args, 1, "seq")?;
    if let Some(string) = args[0].string_val() {
        if string.len() == 0 {
            Ok(MalType::nil())
        } else {
            Ok(MalType::list(
                string
                    .chars()
                    .map(|c| MalType::string(c.to_string()))
                    .collect(),
            ))
        }
    } else if let Some(vec) = args[0].list_or_vector_val() {
        if vec.len() == 0 {
            Ok(MalType::nil())
        } else {
            Ok(MalType::list(vec.clone()))
        }
    } else if args[0].is_nil() {
        Ok(MalType::nil())
    } else {
        Err(MalError::WrongArguments(
            "Must pass a string, list, or vector to seq".to_string(),
        ))
    }
}

fn gensym(_args: &mut Vec<MalType>, env: Option<Env>) -> MalResult {
    let env = env.expect("env must be passed to gensym");
    let mut auto_incr = env.get("*gensym-auto-incr*").unwrap();
    let number = if let Some(auto_incr_val) = auto_incr.atom_val() {
        if let Some(num) = (*auto_incr_val.borrow_mut()).number_val() {
            num
        } else {
            panic!("not possible")
        }
    } else {
        panic!("not possible")
    };
    let add_fn = MalType::function(Function {
        func: Box::new(add),
        env: Some(env),
    });
    auto_incr.swap(add_fn, &mut vec![MalType::number(1)])?;
    let name = "gensym-".to_string() + &number.to_string();
    Ok(MalType::symbol(name))
}

fn time_ms(_args: &mut Vec<MalType>, _env: Option<Env>) -> MalResult {
    let t = get_time();
    let ms = (t.sec * 1_000) as i64 + (t.nsec / 1_000_000) as i64;
    Ok(MalType::number(ms))
}

fn eval(mut args: Vec<MalType>, env: &Env) -> MalResult {
    if let Some(Function { func, .. }) = env.get("eval")
        .expect("eval not a function!")
        .function_val()
    {
        func(&mut args, Some(env.clone()))
    } else {
        panic!("eval not a function!");
    }
}

pub fn eval_func(func: MalType, mut args: &mut Vec<MalType>) -> MalResult {
    if let Some(Function { env, func, .. }) = func.function_val() {
        return func(&mut args, env.clone());
    } else if let Some(Lambda {
        env,
        args: binds,
        body,
        ..
    }) = func.lambda_val()
    {
        let inner_env = Env::with_binds(Some(&env), binds.clone(), args.clone());
        return eval(body.clone(), &inner_env);
    }
    Err(MalError::NotAFunction(func))
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
        if let Some(num) = item.number_val() {
            Some(Ok(num))
        } else {
            Some(Err(MalError::NotANumber))
        }
    }
}
