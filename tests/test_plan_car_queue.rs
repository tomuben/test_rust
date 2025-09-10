#[path = "../src/plan_car_queue/mod.rs"] mod plan_car_queue;


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lin_first() {
        let cars: Vec<i32> = vec![5, 10, 15, 20];
        let mechanics: Vec<i32> = vec![1, 2];
        
        let res = plan_car_queue::linear::alg::plan(cars, mechanics);

        assert_eq!(res, 20);
    }

    #[test]
    fn test_lin_second() {
        let cars: Vec<i32> = vec![1, 1, 1, 1];
        let mechanics: Vec<i32> = vec![3];

        let _res = plan_car_queue::linear::alg::plan(cars, mechanics);
        const _EXPECTED: i32 = 0*3 + 1*3 + (1*3+3) + (1*3+6);
        assert_eq!(_res, _EXPECTED);
    }


    #[test]
    fn test_lin_third() {
        let cars: Vec<i32> = vec![15, 5, 10, 20];
        let mechanics: Vec<i32> = vec![3];

        let _res = plan_car_queue::linear::alg::plan(cars, mechanics);
        const _EXPECTED: i32 = 150;
        assert_eq!(_res, _EXPECTED);
    }

    #[test]
    fn test_exp_one() {
        let cars: Vec<i32> = vec![15, 10, 5, 20];
        let mechanics: Vec<i32> = vec![1, 2];

        let res = plan_car_queue::exp::plan(&cars, &mechanics);
        assert_eq!(res, 20);
    }

    #[test]
    fn test_exp_two() {
        let cars: Vec<i32> = vec![1, 1, 1, 1];
        let mechanics: Vec<i32> = vec![3];

        let res = plan_car_queue::exp::plan(&cars, &mechanics);
        const _EXPECTED: i32 = 0*3 + 1*3 + (1*3+3) + (1*3+6);
        assert_eq!(res, _EXPECTED);
    }

    #[test]
    fn test_exp_third() {
        let cars: Vec<i32> = vec![15, 5, 10, 20];
        let mechanics: Vec<i32> = vec![3];

        let _res = plan_car_queue::exp::plan(&cars, &mechanics);
        const _EXPECTED: i32 = 150;
        assert_eq!(_res, _EXPECTED);
    }

    #[test]
    fn test_exp_custom() {
        let cars: Vec<i32> = vec![5, 5, 5, 10, 10];
        let mechanics: Vec<i32> = vec![10, 1];

        let _res = plan_car_queue::exp::plan(&cars, &mechanics);
        const _EXPECTED: i32 = 30; // Mech[weight=1]: 0*1 + 5*1 + (5+5)*1 + (5+5+5)*1 , Mech[weight=10] = 0*10
        assert_eq!(_res, _EXPECTED);
    }


    #[test]
    fn test_lin_custom() {
        let cars: Vec<i32> = vec![5, 5, 5, 10, 10];
        let mechanics: Vec<i32> = vec![10, 1];

        let _res = plan_car_queue::linear::alg::plan(cars, mechanics);
        const _EXPECTED: i32 = 30; // Mech[weight=1]: 0*1 + 5*1 + (5+5)*1 + (5+5+5)*1 , Mech[weight=10] = 0*10
        assert_eq!(_res, _EXPECTED);
    }
}