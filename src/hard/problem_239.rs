use std::collections::{BinaryHeap, VecDeque};

pub struct Solution {}

/*
You are given an array of integers nums, there is a sliding window of size k which is moving from
the very left of the array to the very right. You can only see the k numbers in the window.
Each time the sliding window moves right by one position.
Return the max sliding window.
 */
impl Solution {
    pub fn max_sliding_window(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let k = k as usize;
        let mut queue: VecDeque<usize> = VecDeque::new();
        let mut result = Vec::new();
        for i in 0..nums.len() {
            while let Some(&idx) = queue.front() {
                if idx + k <= i {
                    queue.pop_front();
                } else {
                    break;
                }
            }
            while let Some(&idx) = queue.back() {
                if nums[idx] <= nums[i] {
                    queue.pop_back();
                } else {
                    break;
                }
            }
            queue.push_back(i);
            if i >= k - 1 {
                if let Some(&idx) = queue.front() {
                    result.push(nums[idx]);
                }
            }
        }
        result
    }
}