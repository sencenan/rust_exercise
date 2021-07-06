use core::str::Chars;

pub fn longest_common_prefix(ss: Vec<String>) -> String {
    let mut prefix = String::from("");
    let mut iters: Vec<Chars> = ss.iter().map(|it| it.chars()).collect();

    'outer: loop {
        let mut cur: Option<char> = None;

        for iter in iters.iter_mut() {
            match iter.next() {
                None => break 'outer,
                Some(c) => match cur {
                    None => cur = Some(c),
                    Some(x) => {
                        if x != c {
                            break 'outer;
                        }
                    }
                },
            }
        }

        if let Some(x) = cur {
            prefix.push(x)
        }
    }

    prefix
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn longest_common_prefix_test() {
        let ss = vec!["flight", "fly", "flower"]
            .iter()
            .map(|x| x.to_string())
            .collect();

        assert_eq!(longest_common_prefix(ss), "fl");
    }
}
