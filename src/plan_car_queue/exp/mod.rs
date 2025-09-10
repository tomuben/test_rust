use std::collections::HashMap;
use super::Mechanic;
use itertools::{repeat_n, Itertools};

pub struct MechanicCar {
    pub mechanic_idx: usize,
    mechanic_weight: i32,
    car: i32,
}


fn calc_sum(combinations: Vec<MechanicCar>) -> i32 {
    let mut mechanics_map:HashMap<usize, Mechanic> = HashMap::new();
    for combination in combinations {
        mechanics_map.entry(combination.mechanic_idx).or_insert(Mechanic::new(combination.mechanic_weight)).assign_new_car(combination.car);
    }
    let mut sum = 0;
    for (mech_idx, mech) in & mechanics_map {
        sum += mech.tot_wait_time;
    }
    sum
}

fn do_calc(cars: &Vec<&i32>, mechanics: &Vec<i32>, combinations: Vec<&usize>) -> i32 {
    let mechanic_car = combinations.iter().enumerate().map(|(idx, combination)| {MechanicCar{mechanic_idx: **combination, mechanic_weight: mechanics[**combination], car: *cars[idx]}}).collect();
    calc_sum(mechanic_car)
}

fn do_combinations(cars: &Vec<&i32>, mechanics: &Vec<i32>) -> i32 {
    let mechanics_indices = mechanics.iter().enumerate().map(|(idx, _)| idx).collect_vec();
    let perms = repeat_n(mechanics_indices.iter(), cars.len()).multi_cartesian_product();
    perms.into_iter().map(|x| do_calc(cars, mechanics, x)).min().unwrap()
}

pub fn plan(cars: &Vec<i32>, mechanics: &Vec<i32>) -> i32 {
    let perms = cars.iter().permutations(cars.len());
    perms.into_iter().map(|x| do_combinations(&x, mechanics)).min().unwrap()
}
