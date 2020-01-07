pub fn gcd(x: u64, y: u64) -> u64 {
    let remainder = x % y;

    if remainder == 0 {
        return y;
    } else {
        return gcd(y, remainder);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn gcd_works() {
        assert_eq!(gcd(2, 4), 2);
        assert_eq!(gcd(6, 27), 3);
        assert_eq!(gcd(4, 2), 2);
        assert_eq!(gcd(27, 6), 3);
    }
}
