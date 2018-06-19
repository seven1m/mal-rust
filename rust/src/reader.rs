extern crate regex;
use regex::Regex;

use types::*;

pub struct Reader {
    tokens: Vec<String>,
    position: usize,
}

impl Reader {
    pub fn peek(&self) -> Option<String> {
        if self.tokens.len() > self.position {
            Some(self.tokens[self.position].to_owned())
        } else {
            None
        }
    }

    pub fn next(&mut self) -> Option<String> {
        if let Some(token) = self.peek() {
            self.position += 1;
            Some(token)
        } else {
            None
        }
    }
}

pub fn read_str(code: &str) -> MalType {
    let tokens = tokenizer(code);
    let mut reader = Reader { tokens: tokens, position: 0 };
    read_form(&mut reader)
}

const TOKEN_MATCH: &str = r#"[\s,]*(~@|[\[\]{}()'`~^@]|"(?:\\.|[^\\"])*"|;.*|[^\s\[\]{}('"`,;)]*)"#;

fn tokenizer(code: &str) -> Vec<String> {
    let re = Regex::new(TOKEN_MATCH).unwrap();
    let mut tokens: Vec<String> = vec![];
    for token_match in re.captures_iter(code) {
        tokens.push(token_match[1].to_string());
    }
    tokens
}

fn read_form(reader: &mut Reader) -> MalType {
    let token = reader.peek().unwrap();
    if token == "(" {
        read_list(reader)
    } else {
        read_atom(reader)
    }
}

fn read_list(reader: &mut Reader) -> MalType {
    let start = reader.next().unwrap();
    if start != "(" { panic!("Expected start of list!") }
    let mut list: Vec<MalType> = Vec::new();
    loop {
        if let Some(token) = reader.peek() {
            if token == ")" { break; }
            let form = read_form(reader);
            list.push(form);
        } else {
            println!("Error: EOF while reading list");
            break;
        }

    }
    MalType::List(list)
}

const NUMBER_MATCH: &str = r#"^\-?[\d\.]+$"#;

fn read_atom(reader: &mut Reader) -> MalType {
    let token = reader.next().unwrap();
    let num_re = Regex::new(NUMBER_MATCH).unwrap();
    if num_re.is_match(&token) {
        MalType::Number(token.parse::<i64>().unwrap_or(0))
    } else {
        MalType::Symbol(token)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tokenizer() {
        let code = "(+ 2 (* 3 4))";
        let tokens = tokenizer(code);
        assert_eq!(
            tokens,
            vec![
                "(".to_string(),
                "+".to_string(),
                "2".to_string(),
                "(".to_string(),
                "*".to_string(),
                "3".to_string(),
                "4".to_string(),
                ")".to_string(),
                ")".to_string(),
            ]
        );
    }

    #[test]
    fn test_read_str() {
        let code = "(+ 2 (* 3 4))";
        let ast = read_str(code);
        assert_eq!(
            ast,
            MalType::List(vec![
                MalType::Symbol("+".to_string()), 
                MalType::Number(2),
                MalType::List(vec![
                    MalType::Symbol("*".to_string()),
                    MalType::Number(3),
                    MalType::Number(4)
                ])
            ])
        );
    }
}
