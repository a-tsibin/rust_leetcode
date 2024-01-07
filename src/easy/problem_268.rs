pub struct Solution {}

/*
Given an array nums containing n distinct numbers in the range [0, n], return the only number in the range that is missing from the array.
 */
impl Solution {
    pub fn missing_number(nums: Vec<i32>) -> i32 {
        let sum = nums.iter().sum::<i32>();
        let expected_sum = (1 + nums.len()) as i32 * nums.len() as i32 / 2;
        expected_sum - sum
    }
}