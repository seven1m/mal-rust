extern crate mal_rust;

use mal_rust::core;
use mal_rust::env::Env;
use mal_rust::printer::pr_str;
use mal_rust::reader::read_str;
use mal_rust::readline::Readline;
use mal_rust::types::*;

use std::collections::BTreeMap;

fn main() {
    let mut readline = Readline::new("user> ");
    let mut repl_env = Env::new(None);
    repl_env.set("+", MalType::function(Box::new(core::add), None));
    repl_env.set("-", MalType::function(Box::new(core::subtract), None));
    repl_env.set("*", MalType::function(Box::new(core::multiply), None));
    repl_env.set("/", MalType::function(Box::new(core::divide), None));
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

fn rep(input: String, repl_env: &Env) -> Result<String, MalError> {
    let out = read(input)?;
    let out = eval(out, repl_env)?;
    let out = print(out);
    Ok(out)
}

fn read(code: String) -> MalResult {
    read_str(&code)
}

fn eval(mut ast: MalType, repl_env: &Env) -> MalResult {
    if ast.is_list() {
        if list_len(&ast) == 0 {
            Ok(ast)
        } else if is_special_form(&ast) {
            process_special_form(&mut ast, repl_env)
        } else {
            let new_ast = eval_ast(ast, repl_env)?;
            if let Some(vec) = new_ast.list_val() {
                if vec.len() > 0 {
                    let mut vec = vec.clone();
                    let first = vec.remove(0);
                    if let Some(Function { func, .. }) = first.function_val() {
                        func(&mut vec, None)
                    } else {
                        Err(MalError::NotAFunction(first.clone()))
                    }
                } else {
                    panic!("Eval'd list is empty!")
                }
            } else {
                panic!("Eval'd list is no longer a list!")
            }
        }
    } else {
        Ok(eval_ast(ast, repl_env)?)
    }
}

fn print(ast: MalType) -> String {
    pr_str(&ast, true)
}

fn eval_ast(ast: MalType, repl_env: &Env) -> MalResult {
    if let Some(symbol) = ast.symbol_val() {
        if let Ok(val) = repl_env.get(&symbol) {
            return Ok(val.clone());
        } else {
            return Err(MalError::SymbolUndefined(symbol.to_string()));
        }
    } else if let Some(vec) = ast.list_or_vector_val() {
        let results: Result<Vec<MalType>, MalError> = vec.into_iter()
            .map(|item| eval(item.clone(), repl_env))
            .collect();
        if ast.is_list() {
            return Ok(MalType::list(results?));
        } else {
            return Ok(MalType::vector(results?));
        }
    } else if let Some(map) = ast.hashmap_val() {
        let mut new_map = BTreeMap::new();
        for (key, val) in map {
            new_map.insert(key.clone(), eval(val.clone(), repl_env)?);
        }
        let mut map = MalType::hashmap(new_map);
        return Ok(map);
    };
    Ok(ast)
}

fn list_len(list: &MalType) -> usize {
    if let Some(vec) = list.list_or_vector_val() {
        vec.len()
    } else {
        panic!("Expected a list but got: {:?}", list)
    }
}

fn is_special_form(ast: &MalType) -> bool {
    if let Some(vec) = ast.list_val() {
        if let Some(sym) = vec[0].symbol_val() {
            match sym {
                "def!" | "let*" => return true,
                _ => {}
            }
        }
    }
    false
}

fn process_special_form(ast: &mut MalType, repl_env: &Env) -> MalResult {
    if let Some(vec) = ast.list_val() {
        let mut vec = vec.clone();
        if let Some(special) = vec.remove(0).symbol_val() {
            return match special {
                "def!" => special_def(&mut vec, repl_env),
                "let*" => special_let(&mut vec, repl_env),
                _ => panic!(format!("Unhandled special form: {}", special)),
            };
        }
    }
    panic!("Expected a List for a special form!")
}

fn special_def(vec: &mut Vec<MalType>, repl_env: &Env) -> MalResult {
    let name = vec.remove(0);
    if let Some(sym) = name.symbol_val() {
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

fn special_let(vec: &mut Vec<MalType>, repl_env: &Env) -> MalResult {
    let inner_repl_env = Env::new(Some(&repl_env));
    let bindings = vec.remove(0);
    if let Some(bindings) = bindings.list_or_vector_val() {
        if bindings.len() % 2 != 0 {
            return Err(MalError::Parse(
                "Odd number of let* binding values!".to_string(),
            ));
        }
        let mut bindings = bindings.clone();
        loop {
            if bindings.len() == 0 {
                break;
            }
            if let Some(name) = bindings.remove(0).symbol_val() {
                let val = eval(bindings.remove(0), &inner_repl_env)?;
                inner_repl_env.set(name, val);
            } else {
                return Err(MalError::Parse("Expected symbol".to_string()));
            }
        }
        let rest = vec.remove(0);
        eval(rest, &inner_repl_env)
    } else {
        Err(MalError::WrongArguments(format!(
            "Expected a vector or list as the first argument to let* but got: {:?}",
            bindings
        )))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_def() {
        let mut repl_env = Env::new(None);
        rep("(def! x 1)".to_string(), &mut repl_env).unwrap();
        let result = rep("x".to_string(), &mut repl_env).unwrap();
        assert_eq!("1", format!("{}", result));
    }

    #[test]
    fn test_let() {
        let mut repl_env = Env::new(None);
        let result = rep("(let* [x 1 y 2 z x] [x y z])".to_string(), &mut repl_env).unwrap();
        assert_eq!("[1 2 1]", format!("{}", result));
    }
}
