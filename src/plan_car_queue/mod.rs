pub mod linear;
pub mod exp;

pub struct Mechanic {
    pub weight: i32,
    acc_wait_time: i32,
    current_wait_time: i32,
    pub tot_wait_time: i32,
}

impl Mechanic {
    pub fn new(weight: i32) -> Mechanic {
        Mechanic { weight, acc_wait_time: 0, tot_wait_time: 0, current_wait_time: 0 }
    }

    pub fn assign_new_car(& mut self, weight_car: i32) {
        self.tot_wait_time += self.acc_wait_time;
        self.current_wait_time = weight_car * self.weight;
        self.acc_wait_time += self.current_wait_time;
    }
}