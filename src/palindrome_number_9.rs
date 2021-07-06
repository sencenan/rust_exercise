pub fn is_palindrome(mut x: i32) -> bool {
    if x < 0 {
        false
    } else {
        let mut len = (x as f32).log(10.0).floor() as i32;

        while len >= 1 {
            let m = 10_i32.pow(len as u32);
            let lead = x / m;
            let tail = x % 10;

            if lead != tail {
                return false;
            }

            x -= lead * m;
            x /= 10;
            len -= 2;
        }

        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_palindrome_test() {
        assert_eq!(is_palindrome(-4), false);
        assert_eq!(is_palindrome(123), false);
        assert_eq!(is_palindrome(121), true);
        assert_eq!(is_palindrome(1221), true);
        assert_eq!(is_palindrome(10001), true);
    }
}
