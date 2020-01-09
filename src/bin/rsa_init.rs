extern crate cryptography_algo;

use cryptography_algo::gcd;
use cryptography_algo::ext_euclid;
use rand::Rng;

fn main() {
    let p: i128 = 88668866989;
    let q: i128 = 13917341761;
    let N: i128 = p * q;
    let r = (p - 1) * (q - 1);

    println!("p: {} ,q: {} N: {}, r: {}", p, q, N, r);

    let mut rng = rand::thread_rng();

    let mut e;
    loop {
        e = rng.gen::<u16>() as i128;
        if gcd::gcd(e, p) == 1 && gcd::gcd(e, q) == 1 {
            break;
        }
    }

    let d = ext_euclid::inv(e, r);

    println!("e: {}, d: {}", e, d);

    println!("public key is(N, e): ({}, {})", N, e);
    println!("private key is(N, d): ({}, {})", N, d);
}
