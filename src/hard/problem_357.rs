pub struct Solution {}

/*
Given an integer n, return the count of all numbers with unique digits, x, where 0 <= x < 10n.
*/

impl Solution {
    pub fn count_numbers_with_unique_digits(n: i32) -> i32 {
        match n {
            0 => 1,
            1 => 10,
            _ => Self::count_numbers_with_unique_digits(n - 1) + 9 * ((11 - n)..=9).product::<i32>(),
        }
    }
}
