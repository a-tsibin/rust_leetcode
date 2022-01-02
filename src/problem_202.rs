pub struct Solution {}

/*
Write an algorithm to determine if a number n is happy.

A happy number is a number defined by the following process:

Starting with any positive integer, replace the number by the sum of the squares of its digits.
Repeat the process until the number equals 1 (where it will stay), or it loops endlessly in a cycle which does not include 1.
Those numbers for which this process ends in 1 are happy.
Return true if n is a happy number, and false if not.
 */
impl Solution {
    pub fn is_happy(n: i32) -> bool {
        use std::collections::HashSet;

        let mut set: HashSet<u32> = HashSet::new();

        fn check_number(n: u32, set: &mut HashSet<u32>) -> bool {
            if set.contains(&n) {
                return false
            }
            set.insert(n);
            let sum = n.to_string().chars().map(|c| c.to_digit(10))
                .map(|x| x.unwrap())
                .fold(0, |acc: u32, x| acc + x * x);
            if sum == 1 {
                return true
            }
            check_number(sum, set)
        }
        check_number(n as u32, &mut set)
    }
}