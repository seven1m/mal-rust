extern crate mal_rust;

use mal_rust::core;
use mal_rust::printer::pr_str;
use mal_rust::reader::read_str;
use mal_rust::readline::Readline;
use mal_rust::types::*;

use std::collections::HashMap;
use std::collections::BTreeMap;

fn main() {
    let mut readline = Readline::new("user> ");
    loop {
        match readline.get() {
            Some(line) => {
                if line.len() > 0 {
                    let result = rep(line);
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

type ReplEnv = HashMap<String, MalType>;

fn rep(input: String) -> Result<String, MalError> {
    let mut repl_env: ReplEnv = HashMap::new();
    repl_env.insert(
        "+".to_string(),
        MalType::function(Function {
            func: Box::new(core::add),
            env: None,
        }),
    );
    repl_env.insert(
        "-".to_string(),
        MalType::function(Function {
            func: Box::new(core::subtract),
            env: None,
        }),
    );
    repl_env.insert(
        "*".to_string(),
        MalType::function(Function {
            func: Box::new(core::multiply),
            env: None,
        }),
    );
    repl_env.insert(
        "/".to_string(),
        MalType::function(Function {
            func: Box::new(core::divide),
            env: None,
        }),
    );
    let out = read(input)?;
    let out = eval(out, &repl_env)?;
    let out = print(out);
    Ok(out)
}

fn read(code: String) -> MalResult {
    read_str(&code)
}

fn eval(ast: MalType, repl_env: &ReplEnv) -> MalResult {
    if ast.is_list() {
        if list_len(&ast) == 0 {
            Ok(ast)
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

fn eval_ast(ast: MalType, repl_env: &ReplEnv) -> MalResult {
    if let Some(symbol) = ast.symbol_val() {
        if let Some(val) = repl_env.get(symbol) {
            return Ok(val.to_owned());
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
