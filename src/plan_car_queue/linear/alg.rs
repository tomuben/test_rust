use super::super::Mechanic;

pub fn plan(cars: Vec<i32>, mechanics: Vec<i32>) -> i32 {
    let mut sorted_cars = cars.clone();
    sorted_cars.sort();
    let mut sorted_mechanics = mechanics.clone();
    sorted_mechanics.sort_by(|a, b| b.cmp(a));

    let mut mechs: Vec<Mechanic> = sorted_mechanics.iter().map(|id| Mechanic::new(*id)).collect();

    'outer: while sorted_cars.len() > 0 {
        for mechanic in mechs.iter_mut() {
            mechanic.assign_new_car(sorted_cars.remove(0));
            if sorted_cars.is_empty() {
                break 'outer
            }
        }
    }
    let sum = mechs.iter().map(|m: &Mechanic| m.tot_wait_time).sum();
    sum
}