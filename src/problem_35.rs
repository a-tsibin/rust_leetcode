pub struct Solution {}

/*
Given a sorted array of distinct integers and a target value, return the index if the target is found. If not, return the index where it would be if it were inserted in order.
You must write an algorithm with O(log n) runtime complexity.
 */
impl Solution {
    pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
        fn bin_search(nums: &[i32], from: usize, to: usize, target: i32) -> usize {
            if nums.len() == 1 {
                if &target > nums.get(0).unwrap() {
                    return from + 1;
                }
                return from;
            }
            let mut mid = nums.len() / 2;
            if nums.get(mid).unwrap() > &target {
                return bin_search(&nums[0..mid], from, mid, target);
            }
            return bin_search(&nums[mid..nums.len()], from + mid, to, target);
        }
        bin_search(nums.as_slice(), 0, nums.len(), target) as i32
    }
}