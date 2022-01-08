mod problem_2;
mod problem_7;
mod problem_3;
mod problem_35;
mod problem_202;
mod problem_26;
mod problem_20;
mod problem_50;
mod problem_41;

#[cfg(test)]
mod tests {
    use super::*;
    use rand::distributions::{Alphanumeric, Standard};
    use rand::prelude::*;

    #[test]
    fn generate_data() {
        for _ in 0..100 {
            let rand_float: f64 = thread_rng().gen_range(-100.0..100.0);
            let rand_int: i32 = thread_rng().gen_range(-3..3);
            println!("{}", rand_float);
            println!("{}", rand_int);
        }
    }

    #[test]
    fn test() {
        let x: f64 = 2.0;
        let res = problem_41::Solution::first_missing_positive(vec![1, 2, 0]);
        assert_eq!(res, 3)
    }
}
