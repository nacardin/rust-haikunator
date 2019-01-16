extern crate haikunator;

use haikunator::Haikunator;

fn main() {
    let mut haikunator = Haikunator::default();
    haikunator.token_hex = true;
    println!("{}", haikunator.haikunate()); // => "misty-boat-bd01"
}