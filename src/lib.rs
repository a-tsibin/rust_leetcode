mod medium;
mod hard;
mod easy;

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
