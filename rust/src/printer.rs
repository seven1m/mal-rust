use types::*;
use std::collections::BTreeMap;
use regex::Regex;

pub fn pr_str(value: &MalType, print_readably: bool) -> String {
    match *value {
        MalType::Nil => "nil".to_string(),
        MalType::True => "true".to_string(),
        MalType::False => "false".to_string(),
        MalType::Number(ref number) => number.to_string(),
        MalType::Symbol(ref symbol) => symbol.to_string(),
        MalType::Keyword(ref keyword) => ":".to_string() + keyword,
        MalType::String(ref string) => {
            if print_readably {
                unescape_single_quotes(&format!("{:?}", string))
            } else {
                string.to_owned()
            }
        }
        MalType::List(ref list, _) => pr_list(list, '(', ')', print_readably),
        MalType::Vector(ref list, _) => pr_list(list, '[', ']', print_readably),
        MalType::HashMap(ref map, _) => pr_map(map, print_readably),
        MalType::Function { .. } => "#<function>".to_string(),
        MalType::Lambda { .. } => "#<function>".to_string(),
        MalType::Atom(ref val) => {
            format!("(atom {})", pr_str(&(*val.borrow()), print_readably)).to_string()
        }
    }
}

fn pr_list(list: &Vec<MalType>, open: char, close: char, print_readably: bool) -> String {
    let mut str = String::new();
    str.push(open);
    let atoms: Vec<String> = list.iter()
        .map(|atom| pr_str(atom, print_readably))
        .collect();
    str.push_str(&atoms.join(" "));
    str.push(close);
    str
}

fn pr_map(map: &BTreeMap<MalType, MalType>, print_readably: bool) -> String {
    let mut str = String::new();
    str.push('{');
    let pairs: Vec<String> = map.iter()
        .map(|(key, val)| pr_str(key, print_readably) + " " + &pr_str(val, print_readably))
        .collect();
    str.push_str(&pairs.join(" "));
    str.push('}');
    str
}

const ESCAPED_SINGLE_QUOTE: &str = r#"\\'"#;

/// Rust likes to escape single quotes -- not sure why.
/// But it breaks our tests.
fn unescape_single_quotes(string: &str) -> String {
    let re = Regex::new(ESCAPED_SINGLE_QUOTE).unwrap();
    re.replace_all(string, "'").into_owned()
}

#[cfg(test)]
mod tests {
    use super::*;
    use reader::read_str;

    #[test]
    fn test_pr_str() {
        let code = "(+ 2 (* 3 4))";
        let ast = read_str(code).unwrap();
        assert_eq!(pr_str(&ast, false), code);
    }
}
