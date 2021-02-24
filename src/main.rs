use std::io::prelude::*;
use std::{
    fs::File,
    io::{self, LineWriter},
};

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

    pub fn write_solution(&self, path: String) {
        let mut file = LineWriter::new(File::create(path).unwrap());
        for row in &self.s {
            let parsed_row: Vec<String> = row.iter().map(|x| x.to_string()).collect();
            file.write_all(parsed_row.join(" ").as_bytes()).unwrap();
            file.write_all(b"\n").unwrap();
        }
        file.flush().unwrap();
    }
}

fn main() {
    let mut game = Game::init();
    println!("{:?}", &game);
    game.calculate_dummy_solution();
    let output_path = "a_example.out";
    game.write_solution(String::from(output_path));
    println!("\nSolution written at: {}", output_path);
}
