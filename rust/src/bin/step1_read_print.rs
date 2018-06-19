extern crate mal_rust;

use mal_rust::readline::Readline;
use mal_rust::reader::read_str;
use mal_rust::printer::pr_str;
use mal_rust::types::*;

fn main() {
    let mut readline = Readline::new();
    loop {
        match readline.get() {
            Some(line) => {
                if line.len() > 0 {
                    println!("{}", rep(line))
                }
            }
            None => break
        }
    }
    readline.save_history();
}

fn rep(input: String) -> String {
    let out = read(input);
    let out = eval(out);
    let out = print(out);
    out
}

fn read(arg: String) -> MalType {
    read_str(&arg)
}

fn eval(arg: MalType) -> MalType {
    arg
}

fn print(arg: MalType) -> String {
    pr_str(&arg)
}
