use std::io::prelude::*;
use std::{
    fs::File,
    io::{self, LineWriter},
};

#[derive(Default, Debug)]
struct Game {
    d: usize,
    i: usize,
    s: usize,
    v: usize,
    f: usize,
    solution: Vec<Vec<String>>,
}

struct Street {
    l: usize,
    start: usize,
    end: usize,
    name: String,
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
        instance.d = first_line[0].parse().unwrap();
        instance.i = first_line[1].parse().unwrap();
        instance.s = first_line[2].parse().unwrap();
        instance.v = first_line[3].parse().unwrap();
        instance.f = first_line[4].parse().unwrap();
        instance
    }

    pub fn write_solution(&self, path: String) {
        let mut file = LineWriter::new(File::create(path).unwrap());
        for row in &self.solution {
            file.write_all(row.join(" ").as_bytes()).unwrap();
            file.write_all(b"\n").unwrap();
        }
        file.flush().unwrap();
    }
}

fn main() {
    let game = Game::init();
    println!("{:?}", &game);
    let output_path = "a.out";
    game.write_solution(String::from(output_path));
    println!("\nSolution written at: {}", output_path);
}
