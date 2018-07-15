use types::*;
use std::collections::BTreeMap;
use regex::Regex;

pub fn pr_str(value: &MalType, print_readably: bool) -> String {
    if value.is_nil() {
        "nil".to_string()
    } else if value.is_true() {
        "true".to_string()
    } else if value.is_false() {
        "false".to_string()
    } else if let Some(number) = value.number_val() {
        number.to_string()
    } else if let Some(symbol) = value.symbol_val() {
        symbol.to_string()
    } else if let Some(keyword) = value.keyword_val() {
        ":".to_string() + keyword
    } else if let Some(string) = value.string_val() {
        if print_readably {
            unescape_single_quotes(&format!("{:?}", string))
        } else {
            string.to_owned()
        }
    } else if let Some(list) = value.list_val() {
        pr_list(list, '(', ')', print_readably)
    } else if let Some(vector) = value.vector_val() {
        pr_list(vector, '[', ']', print_readably)
    } else if let Some(map) = value.hashmap_val() {
        pr_map(map, print_readably)
    } else if value.is_function() {
        "#<function>".to_string()
    } else if value.is_lambda() {
        "#<function>".to_string()
    } else if let Some(atom) = value.atom_val() {
        format!("(atom {})", pr_str(&(*atom.borrow()), print_readably)).to_string()
    } else {
        panic!("Unknown type")
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
