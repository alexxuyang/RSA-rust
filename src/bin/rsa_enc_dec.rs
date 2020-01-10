extern crate cryptography_algo;

use cryptography_algo::power_mod;

use rand::Rng;
use std::env;

fn main() {
    if env::args().len() < 3 {
        println!("program should be start with: rsa_enc_dec base power modula");
        return;
    }

    let base = i64::from_str_radix(&env::args().nth(1).unwrap(), 10).unwrap();
    let power = i64::from_str_radix(&env::args().nth(2).unwrap(), 10).unwrap();
    let modula = i64::from_str_radix(&env::args().nth(3).unwrap(), 10).unwrap();

    println!("base: {}, power: {}, modula: {}", base, power, modula);

    println!("result: {}", power_mod::power_mod(base, power, modula));
}
