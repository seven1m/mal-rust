use linefeed::{DefaultTerminal, Interface, ReadResult};

pub struct Readline {
    reader: Interface<DefaultTerminal>,
}

const HISTORY_FILE: &str = ".mal-history";

impl Readline {
    pub fn new(prompt: &str) -> Readline {
        let reader = Interface::new("mal").unwrap();
        reader.set_prompt(prompt).expect("could not set prompt");
        reader.load_history(HISTORY_FILE).unwrap_or(());
        Readline { reader: reader }
    }

    pub fn get(&mut self) -> Option<String> {
        readline(&mut self.reader)
    }

    pub fn save_history(&self) {
        self.reader.save_history(HISTORY_FILE).unwrap_or(());
    }
}

fn readline(reader: &mut Interface<DefaultTerminal>) -> Option<String> {
    match reader.read_line() {
        Ok(read_result) => match read_result {
            ReadResult::Input(line) => {
                if line.len() > 0 {
                    reader.add_history(line.clone());
                }
                Some(line)
            }
            _ => None,
        },
        Err(err) => {
            println!("Error: {:?}", err);
            None
        }
    }
}
