extern crate rustc_serialize;
extern crate openssl;

use std::env;
use rustc_serialize::hex::ToHex;
use openssl::crypto::hash::{hash, Type};

fn main() {
    let input = env::args().nth(1);
    match input {
        Some(x) => {
            let result = mine_prefix(x, "000000");
            println!("{}", result);
        }
        None => println!("No input provided. Provide a single argument with the instructions"),
    }
}

fn mine_prefix(input: String, prefix: &str) -> u64 {
    let mut cnt = 0u64;
    loop {
        let test_str = input.clone() + &cnt.to_string();
        let res = hash_md5(test_str);
        if res.starts_with(prefix) {
            return cnt;
        }

        cnt = cnt + 1;
    }
}


fn hash_md5(input: String) -> String {
    let input_hex = input.as_bytes();
    let res_bytes = hash(Type::MD5, &input_hex);
    res_bytes.to_hex()
}
