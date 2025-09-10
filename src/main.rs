mod plan_car_queue;
use std::io::{self, BufRead};

fn get_counts(l: String) -> (usize, usize) {
    let vector_of_ints: Vec<usize> = l
        .split_whitespace()
        .filter_map(|s| s.parse().ok())
        .collect();
    if vector_of_ints.len() != 2 {
        panic!("Expected 2 values but got {}", vector_of_ints.len());
    }
    (vector_of_ints[0], vector_of_ints[1])
}

fn parse_line_to_ints(l: String) -> Vec<i32> {
    let vector_of_ints: Vec<i32> = l
        .split_whitespace()
        .filter_map(|s| s.parse().ok())
        .collect();
    vector_of_ints
}


fn main() {
    let stdin = io::stdin();
    let mut iterator = stdin.lock().lines();
    let line1 = iterator.next().unwrap().unwrap();
    let line2 = iterator.next().unwrap().unwrap();
    let line3 = iterator.next().unwrap().unwrap();
    let (n_cars, n_mechanics) = get_counts(line1);
    let cars = parse_line_to_ints(line2);
    let mechanics = parse_line_to_ints(line3);
    if n_cars != cars.len() {
        panic!("Expected number of cars but got {}", n_cars);
    }
    if n_mechanics != mechanics.len() {
        panic!("Expected number of mechanics but got {}", n_mechanics);
    }

    let _res = plan_car_queue::linear::alg::plan(cars, mechanics);
    println!("{:?}", _res);
}
