extern crate regex;
use regex::Regex;

use types::*;

use std::collections::BTreeMap;

macro_rules! consume_and_assert_eq {
    ( $reader:expr, $expected:expr ) => {
        {
            let token = $reader.next().expect(
                &format!("Expected {:?} but got None!", &$expected)
            );
            if token != $expected {
                panic!(format!("Expected {:?} but got {:?}", &$expected, &token));
            }
        }
    };
}

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
    let mut reader = Reader {
        tokens: tokens,
        position: 0,
    };
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
    let mut chars = token.chars();
    match chars.next().unwrap() {
        ';' => {
            reader.next();
            Err(MalError::BlankLine)
        }
        '(' => read_list(reader),
        '[' => read_vector(reader),
        '{' => read_hash_map(reader),
        '"' => read_string(reader),
        ':' => read_keyword(reader),
        '\'' => read_quote(reader, "quote"),
        '~' => {
            if let Some('@') = chars.next() {
                read_quote(reader, "splice-unquote")
            } else {
                read_quote(reader, "unquote")
            }
        }
        '`' => read_quote(reader, "quasiquote"),
        '@' => read_quote(reader, "deref"),
        '^' => read_with_meta(reader),
        _ => read_atom(reader),
    }
}

fn read_string(reader: &mut Reader) -> MalResult {
    let token = reader.next().unwrap();
    let mut chars = token.chars();
    if chars.next().unwrap() != '"' {
        panic!("Expected start of a string!")
    }
    let mut str = String::new();
    loop {
        match chars.next() {
            Some('"') => break,
            Some('\\') => str.push(unescape_char(chars.next())?),
            Some(c) => str.push(c),
            None => return Err(MalError::Parse("Unexpected end of string!".to_string())),
        }
    }
    Ok(MalType::String(str))
}

fn read_keyword(reader: &mut Reader) -> MalResult {
    let token = reader.next().unwrap();
    Ok(MalType::Keyword(token[1..].to_string()))
}

fn read_quote(reader: &mut Reader, expanded: &str) -> MalResult {
    reader.next().unwrap();
    let value = read_form(reader).unwrap();
    let list = MalType::List(vec![MalType::Symbol(expanded.to_string()), value]);
    Ok(list)
}

fn read_with_meta(reader: &mut Reader) -> MalResult {
    consume_and_assert_eq!(reader, "^");
    let metadata = read_form(reader)?;
    let value = read_form(reader)?;
    let list = MalType::List(vec![
        MalType::Symbol("with-meta".to_string()),
        value,
        metadata,
    ]);
    Ok(list)
}

fn unescape_char(char: Option<char>) -> Result<char, MalError> {
    match char {
        Some('n') => Ok('\n'),
        Some(c) => Ok(c),
        None => Err(MalError::Parse("Unexpected end of string!".to_string())),
    }
}

fn read_list(reader: &mut Reader) -> MalResult {
    consume_and_assert_eq!(reader, "(");
    let list = read_list_inner(reader, ")")?;
    Ok(MalType::List(list))
}

fn read_vector(reader: &mut Reader) -> MalResult {
    consume_and_assert_eq!(reader, "[");
    let list = read_list_inner(reader, "]")?;
    Ok(MalType::Vector(list))
}

fn read_hash_map(reader: &mut Reader) -> MalResult {
    consume_and_assert_eq!(reader, "{");
    let list = read_list_inner(reader, "}")?;
    if list.len() % 2 != 0 {
        return Err(MalError::Parse("Odd number of hash-map items!".to_string()));
    }
    let mut map = BTreeMap::new();
    let mut list_iter = list.into_iter();
    loop {
        if let Some(key) = list_iter.next() {
            let val = list_iter.next().unwrap();
            map.insert(key, val);
        } else {
            break;
        }
    }
    Ok(MalType::HashMap(map))
}

fn read_list_inner(reader: &mut Reader, close: &str) -> Result<Vec<MalType>, MalError> {
    let mut list: Vec<MalType> = Vec::new();
    loop {
        if let Some(token) = reader.peek() {
            if token == close {
                reader.next();
                break;
            }
            match read_form(reader) {
                Err(MalError::BlankLine) => {}
                Err(other) => return Err(other),
                Ok(form) => list.push(form),
            }
        } else {
            return Err(MalError::Parse("EOF while reading list".to_string()));
        }
    }
    Ok(list)
}

const NUMBER_MATCH: &str = r#"^\-?[\d\.]+$"#;

fn read_atom(reader: &mut Reader) -> MalResult {
    let token = reader.next().unwrap();
    let num_re = Regex::new(NUMBER_MATCH).unwrap();
    let value = if num_re.is_match(&token) {
        MalType::Number(token.parse::<i64>().unwrap_or(0))
    } else {
        match token.as_ref() {
            "nil" => MalType::Nil,
            "true" => MalType::True,
            "false" => MalType::False,
            _ => MalType::Symbol(token),
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
        let code = "(nil true false :foo \"string\" (+ 2 (* 3 4)))";
        let ast = read_str(code).unwrap();
        assert_eq!(
            ast,
            MalType::List(vec![
                MalType::Nil,
                MalType::True,
                MalType::False,
                MalType::Keyword("foo".to_string()),
                MalType::String("string".to_string()),
                MalType::List(vec![
                    MalType::Symbol("+".to_string()),
                    MalType::Number(2),
                    MalType::List(vec![
                        MalType::Symbol("*".to_string()),
                        MalType::Number(3),
                        MalType::Number(4),
                    ]),
                ]),
            ])
        );
    }

    #[test]
    fn test_read_vector() {
        let code = "[1 :foo nil]";
        let ast = read_str(code).unwrap();
        assert_eq!(
            ast,
            MalType::Vector(vec![
                MalType::Number(1),
                MalType::Keyword("foo".to_string()),
                MalType::Nil,
            ])
        );
    }

    #[test]
    fn test_hash_map() {
        let code = "{:foo 1 \"bar\" [2 3]}";
        let ast = read_str(code).unwrap();
        let mut map = BTreeMap::new();
        map.insert(MalType::Keyword("foo".to_string()), MalType::Number(1));
        map.insert(
            MalType::String("bar".to_string()),
            MalType::Vector(vec![MalType::Number(2), MalType::Number(3)]),
        );
        assert_eq!(ast, MalType::HashMap(map));
    }

    #[test]
    fn test_unclosed_string() {
        let code = "\"abc";
        let err = read_str(code).unwrap_err();
        assert_eq!(err, MalError::Parse("unexpected EOF".to_string()));
    }

    #[test]
    fn test_quote() {
        let code = "('foo ~bar `baz ~@fuz @buz)";
        let ast = read_str(code).unwrap();
        assert_eq!(
            ast,
            MalType::List(vec![
                MalType::List(vec![
                    MalType::Symbol("quote".to_string()),
                    MalType::Symbol("foo".to_string()),
                ]),
                MalType::List(vec![
                    MalType::Symbol("unquote".to_string()),
                    MalType::Symbol("bar".to_string()),
                ]),
                MalType::List(vec![
                    MalType::Symbol("quasiquote".to_string()),
                    MalType::Symbol("baz".to_string()),
                ]),
                MalType::List(vec![
                    MalType::Symbol("splice-unquote".to_string()),
                    MalType::Symbol("fuz".to_string()),
                ]),
                MalType::List(vec![
                    MalType::Symbol("deref".to_string()),
                    MalType::Symbol("buz".to_string()),
                ]),
            ])
        );
    }

    #[test]
    fn test_with_meta() {
        let code = "^{\"a\" 1} [1 2 3]";
        let ast = read_str(code).unwrap();
        let mut map = BTreeMap::new();
        map.insert(MalType::String("a".to_string()), MalType::Number(1));
        assert_eq!(
            ast,
            MalType::List(vec![
                MalType::Symbol("with-meta".to_string()),
                MalType::Vector(vec![
                    MalType::Number(1),
                    MalType::Number(2),
                    MalType::Number(3),
                ]),
                MalType::HashMap(map),
            ])
        );
    }

    #[test]
    fn test_comment() {
        let code = "; comment";
        let err = read_str(code).unwrap_err();
        assert_eq!(err, MalError::BlankLine);
        let code = "[1] ; comment";
        let ast = read_str(code).unwrap();
        assert_eq!(ast, MalType::Vector(vec![MalType::Number(1)]));
        let code = "\"str\" ; comment";
        let ast = read_str(code).unwrap();
        assert_eq!(ast, MalType::String("str".to_string()));
    }
}
