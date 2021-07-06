pub fn roman_to_int(s: String) -> i32 {
    let (res, _) = s.chars().rev().fold((0i32, 0i32), |(acc, prev), x| {
        let v: i32 = match x {
            'I' => 1,
            'V' => 5,
            'X' => 10,
            'L' => 50,
            'C' => 100,
            'D' => 500,
            'M' => 1000,
            _ => 0,
        };

        if v < prev {
            (acc - v, v)
        } else {
            (acc + v, v)
        }
    });

    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn roman_to_int_test() {
        assert_eq!(roman_to_int("MCMXCIV".to_string()), 1994);
    }
}
