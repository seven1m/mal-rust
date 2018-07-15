extern crate mal_rust;

use mal_rust::env::Env;
use mal_rust::printer::pr_str;
use mal_rust::reader::read_str;
use mal_rust::readline::Readline;
use mal_rust::types::*;
use mal_rust::core::NS;

use std::collections::BTreeMap;
use std::env;
use std::process;

fn main() {
    let mut readline = Readline::new("user> ");
    let repl_env = top_repl_env();
    let args: Vec<_> = env::args().collect();
    if args.len() > 1 {
        let result = rep(
            "(load-file \"".to_string() + &args[1] + "\")",
            repl_env.clone(),
        );
        match result {
            Err(err) => {
                println!("{}", err);
                process::exit(1);
            }
            _ => process::exit(0),
        }
    }
    loop {
        match readline.get() {
            Some(line) => {
                if line.len() > 0 {
                    let result = rep(line, repl_env.clone());
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
            MalType::function(Box::new(*func), Some(repl_env.clone())),
        );
    }
    repl_env.set(
        "eval",
        MalType::function(Box::new(eval_fn), Some(repl_env.clone())),
    );
    let argv: Vec<_> = env::args().collect();
    repl_env.set(
        "*ARGV*",
        MalType::list(if argv.len() >= 3 {
            argv[2..]
                .iter()
                .map(|a| MalType::string(a.clone()))
                .collect()
        } else {
            vec![]
        }),
    );
    rep("(def! not (fn* (a) (if a false true)))", repl_env.clone()).expect("could not define not");
    rep(
        "(def! load-file (fn* (f) (eval (read-string (str \"(do \" (slurp f) \")\")))))",
        repl_env.clone(),
    ).expect("could not define load-file");
    repl_env
}

fn eval_fn(args: &mut Vec<MalType>, repl_env: Option<Env>) -> MalResult {
    eval(args.remove(0), repl_env.unwrap())
}

fn rep<S: Into<String>>(input: S, repl_env: Env) -> Result<String, MalError> {
    let out = read(input.into())?;
    let out = eval(out, repl_env)?;
    let out = print(out);
    Ok(out)
}

fn read(code: String) -> MalResult {
    read_str(&code)
}

fn eval(mut ast: MalType, mut repl_env: Env) -> MalResult {
    loop {
        if ast.is_list() {
            if list_len(&ast) == 0 {
                return Ok(ast);
            } else {
                let result = if is_special_form(&ast) {
                    process_special_form(&mut ast, repl_env.clone())?
                } else {
                    eval_list(ast, repl_env.clone())?
                };
                match result {
                    TailPosition::Return(ret) => return Ok(ret),
                    TailPosition::Call(new_ast, new_repl_env) => {
                        ast = new_ast;
                        if new_repl_env.is_some() {
                            repl_env = new_repl_env.unwrap();
                        }
                    }
                }
            }
        } else {
            return Ok(eval_ast(ast, repl_env.clone())?);
        }
    }
}

fn eval_list(ast: MalType, repl_env: Env) -> TailPositionResult {
    let new_ast = eval_ast(ast, repl_env)?;
    if let Some(vec) = new_ast.list_val() {
        if vec.len() > 0 {
            let mut vec = vec.clone();
            let first = vec.remove(0);
            if let Some(Function { env, func, .. }) = first.function_val() {
                func(&mut vec, env.clone()).map(|r| TailPosition::Return(r))
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

fn eval_ast(ast: MalType, repl_env: Env) -> MalResult {
    if let Some(symbol) = ast.symbol_val() {
        if let Ok(val) = repl_env.get(&symbol) {
            return Ok(val.clone());
        } else {
            return Err(MalError::SymbolUndefined(symbol.to_string()));
        }
    } else if let Some(vec) = ast.list_or_vector_val() {
        let results: Result<Vec<MalType>, MalError> = vec.into_iter()
            .map(|item| eval(item.clone(), repl_env.clone()))
            .collect();
        if ast.is_list() {
            return Ok(MalType::list(results?));
        } else {
            return Ok(MalType::vector(results?));
        }
    } else if let Some(map) = ast.hashmap_val() {
        let mut new_map = BTreeMap::new();
        for (key, val) in map {
            new_map.insert(key.clone(), eval(val.clone(), repl_env.clone())?);
        }
        let mut map = MalType::hashmap(new_map);
        return Ok(map);
    };
    Ok(ast)
}

fn print(ast: MalType) -> String {
    pr_str(&ast, true)
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
) -> TailPositionResult {
    let env = Env::with_binds(Some(&outer_env), binds, args);
    let expr = body.remove(0);
    Ok(TailPosition::Call(expr, Some(env)))
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

fn process_special_form(ast: &mut MalType, repl_env: Env) -> TailPositionResult {
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

fn special_def(vec: &mut Vec<MalType>, repl_env: Env) -> TailPositionResult {
    let name = vec.remove(0);
    if let Some(sym) = name.symbol_val() {
        let val = eval(vec.remove(0), repl_env.clone())?;
        repl_env.set(sym, val.clone());
        Ok(TailPosition::Return(val))
    } else {
        Err(MalError::WrongArguments(format!(
            "Expected a symbol as the first argument to def! but got: {:?}",
            name
        )))
    }
}

fn special_let(vec: &mut Vec<MalType>, repl_env: Env) -> TailPositionResult {
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
                let val = eval(bindings.remove(0), inner_repl_env.clone())?;
                inner_repl_env.set(name, val);
            } else {
                return Err(MalError::Parse("Expected symbol".to_string()));
            }
        }
        let rest = vec.remove(0);
        Ok(TailPosition::Call(rest, Some(inner_repl_env)))
    } else {
        Err(MalError::WrongArguments(format!(
            "Expected a vector or list as the first argument to let* but got: {:?}",
            bindings
        )))
    }
}

fn special_do(list: &mut Vec<MalType>, repl_env: Env) -> TailPositionResult {
    if list.len() > 0 {
        while list.len() >= 2 {
            eval(list.remove(0), repl_env.clone())?;
        }
        Ok(TailPosition::Call(list.remove(0), Some(repl_env)))
    } else {
        Ok(TailPosition::Return(MalType::nil()))
    }
}

fn special_if(list: &mut Vec<MalType>, repl_env: Env) -> TailPositionResult {
    let condition = list[0].clone();
    let result = eval(condition, repl_env)?;
    if result.is_falsey() {
        if list.len() >= 3 {
            Ok(TailPosition::Call(list[2].clone(), None))
        } else {
            Ok(TailPosition::Return(MalType::nil()))
        }
    } else {
        Ok(TailPosition::Call(list[1].clone(), None))
    }
}

fn special_fn(list: &mut Vec<MalType>, repl_env: Env) -> TailPositionResult {
    let args = &list[0];
    if let Some(args) = args.list_or_vector_val() {
        let mut args = args.clone();
        let body = list[1].clone();
        Ok(TailPosition::Return(MalType::lambda(
            repl_env.clone(),
            args,
            vec![body],
        )))
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
    fn test_atom() {
        let repl_env = top_repl_env();
        rep("(def! a (atom 1))", repl_env.clone()).unwrap();
        let a = &repl_env.get("a").unwrap();
        assert_eq!("(atom 1)", print(a.clone()));
        rep("(reset! a 2)", repl_env.clone()).unwrap();
        assert_eq!("(atom 2)", print(a.clone()));
        let result = rep("(deref a)", repl_env.clone()).unwrap();
        assert_eq!("2", result);
        assert_eq!("(atom 2)", print(a.clone()));
        rep("(swap! a + 2)", repl_env.clone()).unwrap();
        assert_eq!("(atom 4)", print(a.clone()));
    }

    #[test]
    fn test_load_file() {
        let repl_env = top_repl_env();
        let result = rep("(load-file \"../tests/incB.mal\")", repl_env.clone()).unwrap();
        assert_eq!("\"incB.mal return string\"", result);
    }
}
