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
    let repl_env = top_repl_env();
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
        repl_env.set(name, MalType::function(Box::new(*func), None));
    }
    rep("(def! not (fn* (a) (if a false true)))", repl_env.clone()).expect("could not define not");
    repl_env
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
        if let MalType::List(_, _) = ast {
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
    if let MalType::List(mut vec, _) = new_ast {
        if vec.len() > 0 {
            let first = vec.remove(0);
            match first {
                MalType::Function { func, .. } => {
                    func(&mut vec, None).map(|r| TailPosition::Return(r))
                }
                MalType::Lambda {
                    env, args, body, ..
                } => call_lambda(env, args, body, vec),
                _ => Err(MalError::NotAFunction(first)),
            }
        } else {
            panic!("Eval'd list is empty!")
        }
    } else {
        panic!("Eval'd list is no longer a list!")
    }
}

fn eval_ast(ast: MalType, repl_env: Env) -> MalResult {
    match ast {
        MalType::Symbol(symbol) => {
            if let Ok(val) = repl_env.get(&symbol) {
                Ok(val.to_owned())
            } else {
                Err(MalError::SymbolUndefined(symbol.to_string()))
            }
        }
        MalType::List(vec, _) => {
            let results: Result<Vec<MalType>, MalError> = vec.into_iter()
                .map(|item| eval(item, repl_env.clone()))
                .collect();
            Ok(MalType::list(results?))
        }
        MalType::Vector(vec, _) => {
            let results: Result<Vec<MalType>, MalError> = vec.into_iter()
                .map(|item| eval(item, repl_env.clone()))
                .collect();
            Ok(MalType::vector(results?))
        }
        MalType::HashMap(map, metadata) => {
            let mut new_map = BTreeMap::new();
            for (key, val) in map {
                new_map.insert(key, eval(val, repl_env.clone())?);
            }
            Ok(MalType::HashMap(new_map, metadata))
        }
        _ => Ok(ast),
    }
}

fn print(ast: MalType) -> String {
    pr_str(&ast, true)
}

fn list_len(list: &MalType) -> usize {
    if let &MalType::List(ref vec, _) = list {
        vec.len()
    } else {
        panic!("Not a list!")
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
    if let &MalType::List(ref vec, _) = ast {
        if let &MalType::Symbol(ref sym) = &vec[0] {
            match sym.as_ref() {
                "def!" | "let*" | "do" | "if" | "fn*" => return true,
                _ => {}
            }
        }
    }
    false
}

fn process_special_form(ast: &mut MalType, repl_env: Env) -> TailPositionResult {
    if let &mut MalType::List(ref mut vec, _) = ast {
        if let MalType::Symbol(special) = vec.remove(0) {
            return match special.as_ref() {
                "def!" => special_def(vec, repl_env),
                "let*" => special_let(vec, repl_env),
                "do" => special_do(vec, repl_env),
                "if" => special_if(vec, repl_env),
                "fn*" => special_fn(vec, repl_env),
                _ => panic!(format!("Unhandled special form: {}", &special)),
            };
        }
    }
    panic!("Expected a List for a special form!")
}

fn special_def(vec: &mut Vec<MalType>, repl_env: Env) -> TailPositionResult {
    let name = vec.remove(0);
    if let MalType::Symbol(ref sym) = name {
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
    match bindings {
        MalType::Vector(mut bindings, _) | MalType::List(mut bindings, _) => {
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
                    let val = eval(bindings.remove(0), inner_repl_env.clone())?;
                    inner_repl_env.set(&name, val);
                } else {
                    return Err(MalError::Parse("Expected symbol".to_string()));
                }
            }
            let rest = vec.remove(0);
            Ok(TailPosition::Call(rest, Some(inner_repl_env)))
            //return eval(rest, &mut inner_repl_env).map(|r| TailPosition::Return(r));
        }
        _ => Err(MalError::WrongArguments(format!(
            "Expected a vector or list as the first argument to let* but got: {:?}",
            bindings
        ))),
    }
}

fn special_do(list: &mut Vec<MalType>, repl_env: Env) -> TailPositionResult {
    if list.len() > 0 {
        while list.len() >= 2 {
            eval(list.remove(0), repl_env.clone())?;
        }
        Ok(TailPosition::Call(list.remove(0), Some(repl_env)))
    } else {
        Ok(TailPosition::Return(MalType::Nil))
    }
}

fn special_if(list: &mut Vec<MalType>, repl_env: Env) -> TailPositionResult {
    let condition = list.remove(0);
    match eval(condition, repl_env)? {
        MalType::False | MalType::Nil => {
            if list.len() >= 2 {
                Ok(TailPosition::Call(list.remove(1), None))
            } else {
                Ok(TailPosition::Return(MalType::Nil))
            }
        }
        _ => Ok(TailPosition::Call(list.remove(0), None)),
    }
}

fn special_fn(list: &mut Vec<MalType>, repl_env: Env) -> TailPositionResult {
    let args = list.remove(0);
    match args {
        MalType::List(args, _) | MalType::Vector(args, _) => {
            let body = list.remove(0);
            Ok(TailPosition::Return(MalType::lambda(
                repl_env.clone(),
                args,
                vec![body],
            )))
        }
        _ => Err(MalError::WrongArguments(format!(
            "Expected a vector as the first argument to fn* but got: {:?}",
            args
        ))),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tco() {
        let repl_env = top_repl_env();
        rep(
            "(def! f
              (fn* [a i]
                (if (= i 0)
                    a
                    (f (+ a 1) (- i 1)))))",
            repl_env.clone(),
        ).unwrap();
        let result = rep("(f 1 1000)", repl_env).unwrap();
        assert_eq!("1001", result);
    }
}
