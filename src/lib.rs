mod medium;
mod hard;
mod easy;

use crate::hard::problem_239;

#[cfg(test)]
mod tests {
    use super::*;
    use rand::distributions::{Alphanumeric, Standard};
    use rand::prelude::*;

    #[test]
    fn generate_data() {
        let mut rng = thread_rng();
        (0..100000).map(|_| {
            let h = rng.gen_range(-10000..10000);
            print!("{},", h);
            h
        }).collect::<Vec<i32>>();
    }

    #[test]
    fn test() {
        let res = problem_239::Solution::max_sliding_window(vec![1, 3, -1, -3, 5, 3, 6, 7], 3);
        assert_eq!(res, vec![3, 3, 5, 5, 6, 7])
    }
}
