use std::{collections::HashMap, io::prelude::*};
use std::{
    fs::File,
    io::{self, LineWriter},
};
use std::env;

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
    input: Vec<Street>,
    output: Vec<Street>,
    load: usize,
}

#[derive(Default, Debug)]
struct Street {
    l: usize,
    start: usize,
    end: usize,
    name: String,
    load: usize,
}

impl Clone for Street {
    fn clone(&self) -> Self {
        Street {
            l: self.l,
            start: self.start,
            end: self.end,
            name: self.name.clone(),
            load: self.load,
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
            instance.streets.insert(name, street.clone());
            instance.intersections[end].input.push(street);
        }
        for _ in 0..instance.v {
            let car = Car::from_line(&(line_iter.next().unwrap().unwrap()));
            for (i, s) in car.route.iter().enumerate() {
                if i < car.route.len() - 1 {
                    let street = instance.streets.get(s).unwrap();
                    instance.intersections[street.end].load += 1;
                    for is in instance.intersections[street.end].input.iter_mut() {
                        if is.name.eq(&street.name) {
                            is.load += 1;
                            break;
                        }
                    }
                }
            }
            instance.cars.push(car);
        }
        instance
    }

    pub fn calculate_greedy_solution(&mut self) {
        let intersections: Vec<(usize, &Intersection)> = self.intersections.iter().enumerate().filter(|x| x.1.load != 0).collect();
        self.solution.push(vec![intersections.len().to_string()]);
        for (i, intersection) in intersections.iter() {
            self.solution.push(vec![i.to_string()]);
            let streets: Vec<&Street> = intersection.input.iter().filter(|x| x.load != 0).collect();
            self.solution.push(vec![streets.len().to_string()]);
            for street in streets.iter() {
                let mut weight = street.load;
                if weight > self.d {
                    weight = self.d;
                }
                //println!("{}", street.load * 10);
                self.solution.push(vec![street.name.clone(), weight.to_string()]);
            }
        }
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
    let output_name = env::args().next().unwrap_or_else(|| "result.txt".to_string());
    let mut game = Game::init();
    game.calculate_greedy_solution();
    game.write_solution(String::from(&output_name));
    println!("\nSolution written at: {}", &output_name);
}
