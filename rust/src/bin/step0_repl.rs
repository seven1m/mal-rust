extern crate rustyline;

use std::io::{stdin, stdout, Write};
use std::env;
use rustyline::{Editor, error::ReadlineError};

const HISTORY_FILE: &str = ".mal-history";
 
fn main() {
    let mut rl = Editor::<()>::new();
    rl.load_history(HISTORY_FILE).unwrap_or(());
    let use_readline = use_readline();
    loop {
        let line = if use_readline {
            readline_rep(&mut rl)
        } else {
            dumb_readline_rep()
        };
        match line {
            Some(line) => {
                if line.len() > 0 {
                    rl.add_history_entry(&line);
                    println!("{}", rep(line))
                }
            }
            None => break
        }
    }
    rl.save_history(HISTORY_FILE).unwrap();
}

fn use_readline() -> bool {
    env::var("READLINE").unwrap_or("true".to_string()) == "true"
}

fn readline_rep(rl: &mut Editor<()>) -> Option<String> {
    let readline = rl.readline("user> ");
    match readline {
        Ok(line) => {
            Some(line)
        },
        Err(ReadlineError::Interrupted) | Err(ReadlineError::Eof) => {
            None
        },
        Err(err) => {
            println!("Error: {:?}", err);
            None
        }
    }
}

fn dumb_readline_rep() -> Option<String> {
    print!("user> ");
    stdout().flush().unwrap();
    let mut line=String::new();
    stdin().read_line(&mut line).unwrap();
    if line.bytes().len() > 0 {
        line.pop(); // remove newline
        Some(line)
    } else {
        None
    }
}

fn rep(input: String) -> String {
    let out = read(input);
    let out = eval(out);
    let out = print(out);
    out
}

fn read(arg: String) -> String {
    arg
}

fn eval(arg: String) -> String {
    arg
}

fn print(arg: String) -> String {
    arg
}
