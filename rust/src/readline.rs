use std::io::{stdin, stdout, Write};
use std::env;
use linefeed::{DefaultTerminal, Interface, ReadResult};

pub struct Readline {
    reader: Interface<DefaultTerminal>,
    use_readline: bool,
}

const HISTORY_FILE: &str = ".mal-history";
 
impl Readline {
    pub fn new() -> Readline {
        let reader = Interface::new("mal").unwrap();
        reader.set_prompt("user> ");
        reader.load_history(HISTORY_FILE).unwrap_or(());
        Readline {
            reader: reader,
            use_readline: use_readline(),
        }
    }

    pub fn get(&mut self) -> Option<String> {
        if self.use_readline {
            real_readline(&mut self.reader)
        } else {
            dumb_readline()
        }
    }

    pub fn save_history(&self) {
        self.reader.save_history(HISTORY_FILE).unwrap_or(());
    }
}

fn use_readline() -> bool {
    env::var("READLINE").unwrap_or("true".to_string()) == "true"
}

fn real_readline(reader: &mut Interface<DefaultTerminal>) -> Option<String> {
    match reader.read_line() {
        Ok(read_result) => {
            match read_result {
                ReadResult::Input(line) => {
                    if line.len() > 0 {
                        reader.add_history(line.clone());
                    }
                    Some(line)
                }
                _ => None
            }
        }
        Err(err) => {
            println!("Error: {:?}", err);
            None
        }
    }
}

fn dumb_readline() -> Option<String> {
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
