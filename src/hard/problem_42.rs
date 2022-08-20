pub struct Solution {}

/*
Given n non-negative integers representing an elevation map where the width of each bar is 1, compute how much water it can trap after raining.
*/
impl Solution {
    pub fn trap(height: Vec<i32>) -> i32 {
        let mut total_volume = 0;
        if !height.is_empty() {
            let mut left = 0;
            let mut right = height.len() - 1;
            let (mut left_max, mut right_max) = (0, 0);
            while left < right {
                if height[left] < height[right] {
                    if height[left] >= left_max {
                        left_max = height[left]
                    } else {
                        total_volume += left_max - height[left]
                    }
                    left += 1
                } else {
                    if height[right] >= right_max {
                        right_max = height[right]
                    } else {
                        total_volume += right_max - height[right]
                    }
                    right -= 1
                }
            };
        }
        total_volume
    }
}