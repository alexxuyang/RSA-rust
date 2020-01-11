extern crate cryptography_algo;

use cryptography_algo::power_mod;

use rand::Rng;
use std::env;

#[test]
fn test_all() {
    let p: i64 = 2134324421;
    let q: i64 = 1990843139;
    let N = p * q;
    let r = (p - 1) * (q - 1);
    let e: i64 = 1343;
    let d: i64 = 1920481616808978247;

    let m: i64 = 50412164937805327;

    let c = power_mod::power_mod(m, e, N);
    let m2 = power_mod::power_mod(c, d, N);

    assert_eq!(m, m2);
}
