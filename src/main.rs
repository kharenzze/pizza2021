use std::io;
use std::io::prelude::*;

fn main() {
    let stdin = io::stdin();
    let mut i = 0_usize;
    for line in stdin.lock().lines() {
        println!("{}", line.unwrap());
    }
}
