extern crate crypto;

use crypto::digest::{self, Digest};
use crypto::sha3::Sha3;

fn main() {
    let mut hasher = Sha3::sha3_256();
    hasher.input_str("hello wordl");
    let result = hasher.result_str();
    println!("result = {}", result);
}
