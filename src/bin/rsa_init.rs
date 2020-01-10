extern crate cryptography_algo;

use cryptography_algo::gcd;
use cryptography_algo::ext_euclid;
use rand::Rng;

fn main() {
    // let p: i64 = 666889;
    // let q: i64 = 981287;
    let p: i64 = 16977949338478092359;
    let q: i64 = 981287;
    let N: i64 = p * q;
    let r = (p - 1) * (q - 1);

    println!("p: {} ,q: {} N: {}, r: {}", p, q, N, r);

    let mut rng = rand::thread_rng();

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
