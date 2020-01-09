fn ext_euclid(x: i128, y: i128) -> (i128, i128, i128) {
    let (mut old_r, mut r) = (x, y);
    let (mut old_s, mut s) = (1, 0);
    let (mut old_t, mut t) = (0, 1);

    if y == 0 {
        return (1, 0, x);
    } else {
        while r != 0 {
            let q = old_r / r;

            let t_r = r;
            r = old_r - q * r;
            old_r = t_r;

            let t_s = s;
            s = old_s - q * s;
            old_s = t_s;

            let t_t = t;
            t = old_t - q * t;
            old_t = t_t;
        }
        return (old_s, old_t, old_r);
    }
}

pub fn inv(a: i128, p: i128) -> i128 {
    let (r, _, _) = ext_euclid(a, p);
    return ((r % p) + p) % p
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ext_euclid_works() {
        assert_eq!(inv(5, 11), 9);
        assert_eq!(inv(7, 13), 2);
        assert_eq!(inv(10, 31), 28);
        assert_eq!(inv(12, 29), 17);
    }
}

