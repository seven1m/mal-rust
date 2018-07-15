extern crate mal_rust;

use mal_rust::env::Env;
use mal_rust::printer::pr_str;
use mal_rust::reader::read_str;
use mal_rust::readline::Readline;
use mal_rust::types::*;
use mal_rust::core::NS;

use std::collections::BTreeMap;

fn main() {
    let mut readline = Readline::new("user> ");
    let mut repl_env = top_repl_env();
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

fn top_repl_env() -> Env {
    let repl_env = Env::new(None);
    for (name, func) in NS.iter() {
        repl_env.set(
            name,
            MalType::function(Function {
                func: Box::new(*func),
                env: None,
            }),
        );
    }
    rep(
        "(def! not (fn* (a) (if a false true)))".to_string(),
        &repl_env,
    ).expect("could not define not");
    repl_env
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
                    } else if let Some(Lambda {
                        env, args, body, ..
                    }) = first.lambda_val()
                    {
                        call_lambda(env.clone(), args.clone(), body.clone(), vec)
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

fn call_lambda(
    outer_env: Env,
    binds: Vec<MalType>,
    mut body: Vec<MalType>,
    args: Vec<MalType>,
) -> MalResult {
    let env = Env::with_binds(Some(&outer_env), binds, args);
    let expr = body.remove(0);
    eval(expr, &env)
}

fn is_special_form(ast: &MalType) -> bool {
    if let Some(vec) = ast.list_val() {
        if let Some(sym) = vec[0].symbol_val() {
            match sym {
                "def!" | "let*" | "do" | "if" | "fn*" => return true,
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
                "do" => special_do(&mut vec, repl_env),
                "if" => special_if(&mut vec, repl_env),
                "fn*" => special_fn(&mut vec, repl_env),
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

fn special_do(list: &mut Vec<MalType>, repl_env: &Env) -> MalResult {
    let mut result = MalType::nil();
    while list.len() > 0 {
        result = eval(list.remove(0), repl_env)?;
    }
    Ok(result)
}

fn special_if(list: &mut Vec<MalType>, repl_env: &Env) -> MalResult {
    let condition = list[0].clone();
    let result = eval(condition, repl_env)?;
    if result.is_falsey() {
        if list.len() >= 3 {
            eval(list[2].clone(), repl_env)
        } else {
            Ok(MalType::nil())
        }
    } else {
        eval(list[1].clone(), repl_env)
    }
}

fn special_fn(list: &mut Vec<MalType>, repl_env: &Env) -> MalResult {
    let args = &list[0];
    if let Some(args) = args.list_or_vector_val() {
        let mut args = args.clone();
        let body = list[1].clone();
        Ok(MalType::lambda(Lambda {
            env: repl_env.clone(),
            args,
            body: vec![body],
            is_macro: false,
        }))
    } else {
        Err(MalError::WrongArguments(format!(
            "Expected a vector as the first argument to fn* but got: {:?}",
            args
        )))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_if() {
        let mut repl_env = top_repl_env();
        let result = rep("(if 1 2 3)".to_string(), &mut repl_env).unwrap();
        assert_eq!("2", result);
        let result = rep("(if false 2 3)".to_string(), &mut repl_env).unwrap();
        assert_eq!("3", result);
        let result = rep("(if nil 2 (+ 2 3))".to_string(), &mut repl_env).unwrap();
        assert_eq!("5", result);
        let result = rep("(if nil 2)".to_string(), &mut repl_env).unwrap();
        assert_eq!("nil", result);
    }

    #[test]
    fn test_fn() {
        let mut repl_env = top_repl_env();
        let result = rep("(fn* [a] a)".to_string(), &mut repl_env).unwrap();
        assert_eq!("#<function>", result);
        let result = rep("((fn* [a] a) 7)".to_string(), &mut repl_env).unwrap();
        assert_eq!("7", result);
        let result = rep("((fn* [a b] (+ a b)) 2 3)".to_string(), &mut repl_env).unwrap();
        assert_eq!("5", result);
        let result = rep(
            "((fn* [a & more] (count more)) 2 3 4)".to_string(),
            &mut repl_env,
        ).unwrap();
        assert_eq!("2", result);
        let result = rep(
            "((fn* (a & more) (count more)) 2)".to_string(),
            &mut repl_env,
        ).unwrap();
        assert_eq!("0", result);
    }

    #[test]
    fn test_do() {
        let mut repl_env = top_repl_env();
        let result = rep("(do 1 (def! x (+ 1 2)) (* 2 3))".to_string(), &mut repl_env).unwrap();
        assert_eq!("6", result);
        assert_eq!(MalType::number(3), repl_env.get("x").unwrap());
    }

    #[test]
    fn test_list_and_vec_equal() {
        let mut repl_env = top_repl_env();
        let result = rep(
            "(= [1 2 (list 3 4 [5 6])] (list 1 2 [3 4 (list 5 6)]))".to_string(),
            &mut repl_env,
        ).unwrap();
        assert_eq!("true", result);
    }
}
