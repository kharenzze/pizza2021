use std::{collections::HashMap, io::prelude::*};
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
    streets: HashMap<String, Street>,
    cars: Vec<Car>,
    intersections: Vec<Intersection>,
    solution: Vec<Vec<String>>,
}

#[derive(Default, Debug)]
struct Intersection {
    input: Vec<Box<Street>>,
    output: Vec<Box<Street>>,
}

#[derive(Default, Debug)]
struct Street {
    l: usize,
    start: usize,
    end: usize,
    name: String,
}

impl Copy for Street { 

}

impl Clone for Street {
    fn clone(&self) -> Self {
        Street {
            l: self.l,
            start: self.start,
            end: self.end,
            name: self.name.clone(),
        }
    }
 }

impl Street {
    fn from_line(line: &String) -> Street {
        let elements: Vec<String> = line.split(' ').map(|x| String::from(x)).collect();
        let mut street = Street::default();
        street.start =  elements[0].parse().unwrap();
        street.end =  elements[1].parse().unwrap();
        street.name =  elements[2].clone();
        street.l =  elements[3].parse().unwrap();
        street
    }
}

#[derive(Default, Debug)]
struct Car {
    route: Vec<String>,
}

impl Car {
    fn from_line(line: &String) -> Car {
        let elements: Vec<String> = line.split(' ').map(|x| String::from(x)).collect();
        let mut car = Car::default();
        car.route = elements[1..].to_vec();
        car
    }
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
        instance.streets = HashMap::with_capacity(instance.s);
        instance.cars = Vec::with_capacity(instance.v);
        //instance.intersections = vec![Intersection::default(); instance.i];
        instance.intersections = Vec::with_capacity(instance.i);
        for _ in 0..instance.i {
            instance.intersections.push(Intersection::default());
        }
        for _ in 0..instance.s {
            let street = Street::from_line(&(line_iter.next().unwrap().unwrap()));
            let end = street.end;
            let name = street.name.clone();
            instance.streets.insert(name, street);
            instance.intersections[end].input.push(street);
        }
        for _ in 0..instance.v {
            let car = Car::from_line(&(line_iter.next().unwrap().unwrap()));
            instance.cars.push(car);
        }
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
