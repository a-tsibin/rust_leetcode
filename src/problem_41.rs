pub struct Solution {}

/*
Given an unsorted integer array nums, return the smallest missing positive integer.

You must implement an algorithm that runs in O(n) time and uses constant extra space.
 */
impl Solution {
    pub fn first_missing_positive(nums: Vec<i32>) -> i32 {
        let mut checks = vec![false; nums.len()];
        for i in &nums {
            let num = *i as usize;
            if num > 0 && num <= nums.len() {
                checks[num - 1] = true;
            }
        }
        checks.into_iter().position(|x| !x).unwrap_or(nums.len()) as i32 + 1
    }
}