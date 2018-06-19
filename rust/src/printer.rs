use types::*;

pub fn pr_str(value: &MalType) -> String {
    match value {
        &MalType::List(ref list) => {
            let mut str = String::new();
            str.push_str("(");
            let atoms: Vec<String> = list.iter().map(|atom| pr_str(atom)).collect();
            str.push_str(&atoms.join(" "));
            str.push_str(")");
            str
        }
        &MalType::Number(ref number) => number.to_string(),
        &MalType::Symbol(ref symbol) => symbol.to_owned(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use reader::read_str;

    #[test]
    fn test_pr_str() {
        let code = "(+ 2 (* 3 4))";
        let ast = read_str(code);
        assert_eq!(
            pr_str(&ast),
            code
        );
    }
}
