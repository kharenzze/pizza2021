use std::io;
use std::io::prelude::*;

#[derive(Default)]
struct Game {
    m: usize,
    t: (usize, usize, usize),
}

impl Game {
    pub fn init() {
        let mut instance = Self::default();

        let stdin = io::stdin();
        let mut line_iter = stdin.lock().lines();
        let x = line_iter.next().unwrap().unwrap();
        let first_line: Vec<&str> = x.split(' ').collect();
        instance.m = (*first_line[0]).parse().unwrap();
        instance.t = (
            (*first_line[1]).parse().unwrap(),
            (*first_line[2]).parse().unwrap(),
            (*first_line[3]).parse().unwrap()
        );
        instance
    }
}

fn main() {
    let game = Game::init();
}
