use rustyline::{Editor, error::ReadlineError};
use std::io::{stdin, stdout, Write};
use std::env;

pub struct Readline {
    rl: Editor<()>,
    use_readline: bool,
}

const HISTORY_FILE: &str = ".mal-history";
 
impl Readline {
    pub fn new() -> Readline {
        let mut rl = Editor::<()>::new();
        rl.load_history(HISTORY_FILE).unwrap_or(());
        Readline {
            rl: rl,
            use_readline: use_readline(),
        }
    }

    pub fn get(&mut self) -> Option<String> {
        if self.use_readline {
            real_readline(&mut self.rl)
        } else {
            dumb_readline()
        }
    }

    pub fn save_history(&self) {
        self.rl.save_history(HISTORY_FILE).unwrap();
    }
}

fn use_readline() -> bool {
    env::var("READLINE").unwrap_or("true".to_string()) == "true"
}

fn real_readline(rl: &mut Editor<()>) -> Option<String> {
    let readline = rl.readline("user> ");
    match readline {
        Ok(line) => {
            if line.len() > 0 {
                rl.add_history_entry(&line);
            }
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
