use types::*;

pub fn pr_str(value: &MalType, print_readably: bool) -> String {
    match value {
        &MalType::Nil => "nil".to_string(),
        &MalType::True => "true".to_string(),
        &MalType::False => "false".to_string(),
        &MalType::Number(ref number) => number.to_string(),
        &MalType::Symbol(ref symbol) => symbol.to_owned(),
        &MalType::String(ref string) => {
            if print_readably {
                format!("{:?}", string.to_owned())
            } else {
                string.to_owned()
            }
        }
        &MalType::List(ref list) => {
            let mut str = String::new();
            str.push_str("(");
            let atoms: Vec<String> = list.iter().map(|atom| pr_str(atom, print_readably)).collect();
            str.push_str(&atoms.join(" "));
            str.push_str(")");
            str
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use reader::read_str;

    #[test]
    fn test_pr_str() {
        let code = "(+ 2 (* 3 4))";
        let ast = read_str(code).unwrap();
        assert_eq!(
            pr_str(&ast, false),
            code
        );
    }
}
