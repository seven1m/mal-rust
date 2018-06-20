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

pub fn read_str(code: &str) -> MalResult {
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

fn read_form(reader: &mut Reader) -> MalResult {
    let token = reader.peek().unwrap();
    if token.len() == 0 {
        return Err(MalError::Parse("unexpected EOF".to_string()));
    }
    match token.chars().next().unwrap() {
        '(' => read_list(reader),
        '"' => read_string(reader),
        _   => read_atom(reader),
    }
}

fn read_string(reader: &mut Reader) -> MalResult {
    let token = reader.next().unwrap();
    let mut chars = token.chars();
    if chars.next().unwrap() != '"' { return Err(MalError::Parse("Expected start of a string!".to_string())) }
    let mut str = String::new();
    loop {
        match chars.next() {
            Some('"')  => break,
            Some('\\') => str.push(unescape_char(chars.next())?),
            Some(c)    => str.push(c),
            None       => { return Err(MalError::Parse("Unexpected end of string!".to_string())) }
        }
    }
    Ok(MalType::String(str))
}

fn unescape_char(char: Option<char>) -> Result<char, MalError> {
    match char {
        Some('n') => Ok('\n'),
        Some(c) => Ok(c),
        None => Err(MalError::Parse("Unexpected end of string!".to_string())),
    }
}

fn read_list(reader: &mut Reader) -> MalResult {
    let start = reader.next().unwrap();
    if start != "(" { panic!("Expected start of list!") }
    let mut list: Vec<MalType> = Vec::new();
    loop {
        if let Some(token) = reader.peek() {
            if token == ")" { break; }
            let form = read_form(reader)?;
            list.push(form);
        } else {
            return Err(MalError::Parse("EOF while reading list".to_string()));
        }

    }
    Ok(MalType::List(list))
}

const NUMBER_MATCH: &str = r#"^\-?[\d\.]+$"#;

fn read_atom(reader: &mut Reader) -> MalResult {
    let token = reader.next().unwrap();
    let num_re = Regex::new(NUMBER_MATCH).unwrap();
    let value = if num_re.is_match(&token) {
        MalType::Number(token.parse::<i64>().unwrap_or(0))
    } else {
        match token.as_ref() {
            "nil"   => MalType::Nil,
            "true"  => MalType::True,
            "false" => MalType::False,
            _       => MalType::Symbol(token),
        }
    };
    Ok(value)
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
        let code = "(nil true false \"string\" (+ 2 (* 3 4)))";
        let ast = read_str(code).unwrap();
        assert_eq!(
            ast,
            MalType::List(vec![
                MalType::Nil,
                MalType::True,
                MalType::False,
                MalType::String("string".to_string()),
                MalType::List(vec![
                    MalType::Symbol("+".to_string()),
                    MalType::Number(2),
                    MalType::List(vec![
                        MalType::Symbol("*".to_string()),
                        MalType::Number(3),
                        MalType::Number(4)
                    ])
                ])
            ])
        );
    }

    #[test]
    fn test_unclosed_string() {
        let code = "\"abc";
        let err = read_str(code).unwrap_err();
        assert_eq!(
            err,
            MalError::Parse("unexpected EOF".to_string())
        );
    }
}
