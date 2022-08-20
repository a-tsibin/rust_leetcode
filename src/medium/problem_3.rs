pub struct Solution {}

/*
Given a string s, find the length of the longest substring without repeating characters.
 */
impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        use std::collections::HashMap;

        let mut set: HashMap<u8, i32> = HashMap::new();
        let mut start = 0;
        let mut max_len: i32 = 0;
        let mut curr_len = 0;
        for (i, c) in s.bytes().enumerate() {
            if set.contains_key(&c) {
                if curr_len > max_len {
                    max_len = curr_len;
                }
                let new_start = (set.get(&c).unwrap() + 1) as usize;
                if new_start > start {
                    start = new_start;
                }
                curr_len = (i - start) as i32;
            }
            set.insert(c, i as i32);
            curr_len += 1;
        }

        max_len
    }
}