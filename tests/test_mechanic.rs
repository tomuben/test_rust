#[path = "../src/plan_car_queue/mod.rs"] mod plan_car_queue;



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        let mut mech = plan_car_queue::Mechanic::new(100);

        mech.assign_new_car(3);
        assert_eq!(mech.tot_wait_time, 0);
        mech.assign_new_car(5);
        assert_eq!(mech.tot_wait_time, 300);
    }
}