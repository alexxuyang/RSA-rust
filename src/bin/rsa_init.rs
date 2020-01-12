extern crate cryptography_algo;

use cryptography_algo::gcd;
use cryptography_algo::ext_euclid;

use rand::Rng;
use std::env;

fn main() {
    let p;
    let q; 

    let mut rng = rand::thread_rng();

    if env::args().len() == 3 {

        p = i64::from_str_radix(&env::args().nth(1).unwrap(), 10).unwrap();
        q = i64::from_str_radix(&env::args().nth(2).unwrap(), 10).unwrap();

        if !prime_tools::is_u64_prime(p as u64) {
            panic!("p is not a prime number!");
        }

        if !prime_tools::is_u64_prime(q as u64) {
            panic!("q is not a prime number!");
        }
    } else {
        p = 2134324421;
        q = 1990843139;
    }

    let N: i64 = p * q;
    let r = (p - 1) * (q - 1);

    println!("p: {} ,q: {} N: {}, r: {}", p, q, N, r);


    let mut e;
    loop {
        e = rng.gen::<u16>() as i64;
        if gcd::gcd(e, r)  == 1 {
            break;
        }
    }

    let d = ext_euclid::inv(e, r);

    println!("e: {}, d: {}", e, d);

    println!("public key is(N, e): ({}, {})", N, e);
    println!("private key is(N, d): ({}, {})", N, d);
}
