extern crate mal_rust;

use mal_rust::readline::Readline;

fn main() {
    let mut readline = Readline::new("user> ");
    loop {
        match readline.get() {
            Some(line) => {
                if line.len() > 0 {
                    println!("{}", rep(line))
                }
            }
            None => break,
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

fn read(arg: String) -> String {
    arg
}

fn eval(arg: String) -> String {
    arg
}

fn print(arg: String) -> String {
    arg
}
