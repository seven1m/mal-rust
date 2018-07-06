extern crate mal_rust;

use mal_rust::core;
use mal_rust::env;
use mal_rust::printer::pr_str;
use mal_rust::reader::read_str;
use mal_rust::readline::Readline;
use mal_rust::types::*;

use std::collections::BTreeMap;

fn main() {
    let mut readline = Readline::new("user> ");
    let mut repl_env = env::Env::new(None);
    repl_env.set("+", MalType::Function(Box::new(core::add), None));
    repl_env.set("-", MalType::Function(Box::new(core::subtract), None));
    repl_env.set("*", MalType::Function(Box::new(core::multiply), None));
    repl_env.set("/", MalType::Function(Box::new(core::divide), None));
    loop {
        match readline.get() {
            Some(line) => {
                if line.len() > 0 {
                    let result = rep(line, &mut repl_env);
                    match result {
                        Ok(str) => println!("{}", str),
                        Err(MalError::BlankLine) => {}
                        Err(err) => println!("{}", err),
                    }
                }
            }
            None => break,
        }
    }
    readline.save_history();
}

fn rep(input: String, repl_env: &env::Env) -> Result<String, MalError> {
    let out = read(input)?;
    let out = eval(out, repl_env)?;
    let out = print(out);
    Ok(out)
}

fn read(code: String) -> MalResult {
    read_str(&code)
}

fn eval(mut ast: MalType, repl_env: &env::Env) -> MalResult {
    match ast {
        MalType::List(_) => {
            if list_len(&ast) == 0 {
                Ok(ast)
            } else if is_special_form(&ast) {
                process_special_form(&mut ast, repl_env)
            } else {
                let new_ast = eval_ast(ast, repl_env)?;
                if let MalType::List(mut vec) = new_ast {
                    if vec.len() > 0 {
                        let first = vec.remove(0);
                        if let MalType::Function(func, _) = first {
                            func(&mut vec, None)
                        } else {
                            Err(MalError::NotAFunction(first))
                        }
                    } else {
                        panic!("Eval'd list is empty!")
                    }
                } else {
                    panic!("Eval'd list is no longer a list!")
                }
            }
        }
        _ => Ok(eval_ast(ast, repl_env)?),
    }
}

fn print(ast: MalType) -> String {
    pr_str(&ast, true)
}

fn eval_ast(ast: MalType, repl_env: &env::Env) -> MalResult {
    match ast {
        MalType::Symbol(symbol) => {
            if let Ok(val) = repl_env.get(&symbol) {
                Ok(val.to_owned())
            } else {
                Err(MalError::SymbolUndefined(symbol.to_string()))
            }
        }
        MalType::List(vec) => {
            let results: Result<Vec<MalType>, MalError> =
                vec.into_iter().map(|item| eval(item, repl_env)).collect();
            Ok(MalType::List(results?))
        }
        MalType::Vector(vec) => {
            let results: Result<Vec<MalType>, MalError> =
                vec.into_iter().map(|item| eval(item, repl_env)).collect();
            Ok(MalType::Vector(results?))
        }
        MalType::HashMap(map, metadata) => {
            let mut new_map = BTreeMap::new();
            for (key, val) in map {
                new_map.insert(key, eval(val, repl_env)?);
            }
            Ok(MalType::HashMap(new_map, metadata))
        }
        _ => Ok(ast),
    }
}

fn list_len(list: &MalType) -> usize {
    if let &MalType::List(ref vec) = list {
        vec.len()
    } else {
        panic!("Not a list!")
    }
}

fn is_special_form(ast: &MalType) -> bool {
    if let &MalType::List(ref vec) = ast {
        if let &MalType::Symbol(ref sym) = &vec[0] {
            if sym == "def!" || sym == "let*" {
                return true;
            }
        }
    }
    false
}

fn process_special_form(ast: &mut MalType, repl_env: &env::Env) -> MalResult {
    if let &mut MalType::List(ref mut vec) = ast {
        if let MalType::Symbol(special) = vec.remove(0) {
            return match special.as_ref() {
                "def!" => process_special_def(vec, repl_env),
                "let*" => process_special_let(vec, repl_env),
                _ => panic!(format!("Unhandled special form: {}", &special)),
            };
        }
    }
    panic!("Expected a List for a special form!")
}

fn process_special_def(vec: &mut Vec<MalType>, repl_env: &env::Env) -> MalResult {
    let name = vec.remove(0);
    if let MalType::Symbol(ref sym) = name {
        let val = eval(vec.remove(0), repl_env)?;
        repl_env.set(sym, val.clone());
        Ok(val)
    } else {
        Err(MalError::WrongArguments(format!(
            "Expected a symbol as the first argument to def! but got: {:?}",
            name
        )))
    }
}

fn process_special_let(vec: &mut Vec<MalType>, repl_env: &env::Env) -> MalResult {
    let mut inner_repl_env = env::Env::new(Some(&repl_env));
    let bindings = vec.remove(0);
    match bindings {
        MalType::Vector(mut bindings) | MalType::List(mut bindings) => {
            if bindings.len() % 2 != 0 {
                return Err(MalError::Parse(
                    "Odd number of let* binding values!".to_string(),
                ));
            }
            loop {
                if bindings.len() == 0 {
                    break;
                }
                if let MalType::Symbol(name) = bindings.remove(0) {
                    let val = eval(bindings.remove(0), &mut inner_repl_env)?;
                    inner_repl_env.set(&name, val);
                } else {
                    return Err(MalError::Parse("Expected symbol".to_string()));
                }
            }
            let rest = vec.remove(0);
            return eval(rest, &mut inner_repl_env);
        }
        _ => {
            return Err(MalError::WrongArguments(format!(
                "Expected a vector or list as the first argument to let* but got: {:?}",
                bindings
            )));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_def() {
        let mut repl_env = env::Env::new(None);
        rep("(def! x 1)".to_string(), &mut repl_env).unwrap();
        let result = rep("x".to_string(), &mut repl_env).unwrap();
        assert_eq!("1", format!("{}", result));
    }

    #[test]
    fn test_let() {
        let mut repl_env = env::Env::new(None);
        let result = rep("(let* [x 1 y 2 z x] [x y z])".to_string(), &mut repl_env).unwrap();
        assert_eq!("[1 2 1]", format!("{}", result));
    }
}
