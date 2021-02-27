use std::{cell::RefCell, rc::Rc, collections::HashMap, io::prelude::*};
use std::{
    fs::File,
    io::{self, LineWriter},
};
use std::env;

type LocalRef<T> = Rc<RefCell<T>>;

#[derive(Default, Debug)]
struct Game {
    d: usize,
    i: usize,
    s: usize,
    v: usize,
    f: usize,
    streets: HashMap<String, LocalRef<Street>>,
    cars: Vec<Car>,
    intersections: Vec<Intersection>,
    solution: Vec<Vec<String>>,
}

#[derive(Default, Debug)]
struct Intersection {
    input: Vec<LocalRef<Street>>,
    load: usize,
}

#[derive(Default, Debug, Clone)]
struct Street {
    l: usize,
    start: usize,
    end: usize,
    name: String,
    load: usize,
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
            let name = street.name.clone();
            let r_street: LocalRef<Street> = Rc::new(RefCell::new(street));
            instance.streets.insert(name, Rc::clone(&r_street));
            instance.intersections[r_street.borrow().end].input.push(Rc::clone(&r_street));
        }
        for _ in 0..instance.v {
            let car = Car::from_line(&(line_iter.next().unwrap().unwrap()));
            for (i, s) in car.route.iter().enumerate() {
                if i < car.route.len() - 1 {
                    let street_ref = instance.streets.get(s).unwrap();
                    let mut street = street_ref.borrow_mut();
                    instance.intersections[street.end].load += 1;
                    street.load += 1;
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
            let streets: Vec<LocalRef<Street>> = intersection.input.iter().filter(|x| x.borrow().load != 0).map(|x| Rc::clone(x)).collect();
            self.solution.push(vec![streets.len().to_string()]);
            for street_ref in streets.iter() {
                let mut weight = street_ref.borrow().load;
                if weight > self.d {
                    weight = self.d;
                }
                //println!("{}", street.load * 10);
                self.solution.push(vec![street_ref.borrow().name.clone(), weight.to_string()]);
            }
        }
    }

    pub fn write_solution(&self, path: &String) {
        let mut file = LineWriter::new(File::create(path).unwrap());
        for row in &self.solution {
            file.write_all(row.join(" ").as_bytes()).unwrap();
            file.write_all(b"\n").unwrap();
        }
        file.flush().unwrap();
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let output_name = match args.get(1) {
        Some(x) => x.clone(),
        None => "result.txt".to_string()
    };
    
    println!("{}", output_name);
    let mut game = Game::init();
    game.calculate_greedy_solution();
    game.write_solution(&output_name);
    println!("\nSolution written at: {}", &output_name);
}
