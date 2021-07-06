use std::convert::TryFrom;

pub fn reverse(x: i32) -> i32 {
    let mut r: i64 = 0;
    let mut v: i64 = x.abs().into();

    let sign = match x.is_negative() {
        true => -1i64,
        _ => 1i64,
    };

    while v != 0 {
        r = r * 10 + v % 10;
        v /= 10;
    }

    i32::try_from(sign * r).unwrap_or(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reverse_test() {
        assert_eq!(reverse(-4), -4);
        assert_eq!(reverse(123), 321);
    }
}
