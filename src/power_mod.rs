pub fn power_mod(base: i64, mut power: i64, N: i64) -> i64 {
    let mut bits = Vec::new();

    while power != 0 {
        match power & 1 {
           1 => bits.push(true),
           0 => bits.push(false),
           _ => {}
        }
        power = power >> 1;
    }

    let mut result: i64 = 1;
    while let Some(bit) = bits.pop() {
        result = mod_multiply(result, result, N);
        if bit {
            result = mod_multiply(result, base, N);
        }
    }

    result
}

pub fn mod_multiply(a: i64, b:i64, N: i64) -> i64 {
    ((a as i128 * b as i128) % (N as i128)) as i64
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mod_multiply_works() {
        assert_eq!(mod_multiply(10, 20, 17), 13);
        assert_eq!(mod_multiply(6, 27, 5), 2);
        assert_eq!(mod_multiply(4, 2, 19), 8);
        assert_eq!(mod_multiply(27, 6, 13), 6);
    }

    #[test]
    fn power_mod_works() {
        assert_eq!(power_mod(3, 4, 5), 1);
        assert_eq!(power_mod(6, 10, 5), 1);
        assert_eq!(power_mod(5, 13, 19), 17);
        assert_eq!(power_mod(27, 6, 51), 9);
        assert_eq!(power_mod(45, 13, 19), 7);
        assert_eq!(power_mod(1234567, 100, 199), 29);
        assert_eq!(power_mod(66887799, 1000000, 1001), 22);
        assert_eq!(power_mod(1357924680, 999999, 666889), 355775);
        assert_eq!(power_mod(113355778866, 99999, 981287), 797582);
        assert_eq!(power_mod(113355778866, 999999, 5050404053), 31958690);
    }
}
