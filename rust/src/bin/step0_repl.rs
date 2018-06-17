use std::io::{stdin, stdout};
use std::io::Write;
use std::process;

fn main() {
    loop {
        print!("user> ");
        stdout().flush().unwrap();
        let mut input=String::new();
        stdin().read_line(&mut input).unwrap();
        if input.bytes().len() == 0 {
            process::exit(0);
        }
        input.pop(); // remove newline
        println!("{}", rep(input));
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
