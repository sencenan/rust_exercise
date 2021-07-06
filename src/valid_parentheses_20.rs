pub fn is_valid(s: String) -> bool {
    fn get_closer(c: char) -> char {
        match c {
            '(' => ')',
            '[' => ']',
            '{' => '}',
            _ => ' ',
        }
    }

    let mut stack: Vec<char> = Vec::new();

    for c in s.chars() {
        match c {
            '(' | '[' | '{' => stack.push(c),
            _ => match stack.last() {
                None => return false,
                Some(last) => {
                    if get_closer(*last) != c {
                        return false;
                    } else {
                        stack.pop();
                    }
                }
            },
        }
    }

    stack.is_empty()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_valid_test() {
        assert_eq!(is_valid("".to_string()), true);
        assert_eq!(is_valid("[]".to_string()), true);
        assert_eq!(is_valid("{}".to_string()), true);
        assert_eq!(is_valid("(())".to_string()), true);
        assert_eq!(is_valid("{(()[]{()})}".to_string()), true);
        assert_eq!(is_valid("([)]".to_string()), false);
    }
}
