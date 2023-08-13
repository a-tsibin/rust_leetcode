mod easy;
mod hard;
mod medium;

use crate::hard::problem_239;

#[cfg(test)]
mod tests {
    use rand::{
        distributions::{
            Alphanumeric,
            Standard,
        },
        prelude::*,
    };

    use super::*;
    use crate::medium::problem_1654;

    #[test]
    fn generate_data() {
        let mut rng = thread_rng();
        (0..100000)
            .map(|_| {
                let h = rng.gen_range(-10000..10000);
                print!("{},", h);
                h
            })
            .collect::<Vec<i32>>();
    }

    #[test]
    fn test() {
        let res = problem_1654::Solution::minimum_jumps(vec![8, 3, 16, 6, 12, 20], 15, 13, 11);
        assert_eq!(res, -1)
    }
}
