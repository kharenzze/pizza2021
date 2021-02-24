use std::io;
use std::io::prelude::*;

#[derive(Default, Debug)]
struct Game {
    m: usize,
    t: (usize, usize, usize),
    i: Vec<Vec<String>>,
    s: Vec<Vec<i32>>,
}

impl Game {
    pub fn init() -> Self {
        let mut instance = Self::default();
        let stdin = io::stdin();
        let mut line_iter = stdin.lock().lines();
        let first_line: Vec<String> = line_iter
            .next()
            .unwrap()
            .unwrap()
            .split(' ')
            .map(|x| String::from(x))
            .collect();
        instance.m = first_line[0].parse().unwrap();
        instance.t = (
            first_line[1].parse().unwrap(),
            first_line[2].parse().unwrap(),
            first_line[3].parse().unwrap(),
        );
        instance.i = vec![];
        for line in line_iter {
            let current_line: Vec<String> =
                line.unwrap().split(' ').map(|x| String::from(x)).collect();
            instance.i.push(current_line[1..].to_vec());
        }
        instance
    }

    pub fn calculate_dummy_solution(&mut self) {
        self.s = vec![vec![2], vec![2, 1, 4], vec![3, 0, 2, 3]];
    }
}

fn main() {
    let mut game = Game::init();
    println!("{:?}", &game);
    game.calculate_dummy_solution();
}
