mod problem_2;
mod problem_7;
mod problem_3;
mod problem_35;
mod problem_202;
mod problem_26;
mod problem_20;
mod problem_50;
mod problem_41;
mod problem_42;
mod problem_9;
mod problem_295;

#[cfg(test)]
mod tests {
    use super::*;
    use rand::distributions::{Alphanumeric, Standard};
    use rand::prelude::*;

    #[test]
    fn generate_data() {
        let mut rng = thread_rng();
        (0..10000).map(|_| {
            let h = rng.gen_range(0..100000);
            print!("{},", h);
            h
        }).collect::<Vec<i32>>();
    }

    #[test]
    fn test() {
        let x: f64 = 2.0;
        let res = problem_41::Solution::first_missing_positive(vec![1, 2, 0]);
        assert_eq!(res, 3)
    }
}
