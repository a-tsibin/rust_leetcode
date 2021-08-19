use std::collections::{HashMap, HashSet};

struct Solution {}

impl Solution {
    /*pub fn reverse(x: i32) -> i32 {
        let mut str = x.to_string();
        let mut negative = false;
        if str.starts_with("-") {
            negative = true;
            str.remove(0);
        }
        let res = match str.chars().rev().collect::<String>().parse() {
            Ok(n) =>
                if negative {
                    0 - n
                } else {
                    n
                },
            _ => 0
        };
        res
    }*/

    /*pub fn add_two_numbers(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        match (l1, l2) {
            (None, None) => None,
            (Some(n), None) | (None, Some(n)) => Some(n),
            (Some(n1), Some(n2)) => {
                let sum = n1.val + n2.val;
                if sum < 10 {
                    Some(Box::new(ListNode {
                        val: sum,
                        next: Solution::add_two_numbers(n1.next, n2.next)
                    }))
                } else {
                    let prev = Some(Box::new(ListNode::new(1)));
                    Some(Box::new(ListNode {
                        val: sum - 10,
                        next: Solution::add_two_numbers(Solution::add_two_numbers(prev, n1.next), n2.next)
                    }))
                }
            }
        }
    }*/

    /*pub fn length_of_longest_substring(s: String) -> i32 {
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
    }*/

    /*pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
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
    }*/

    /*fn nb_dig(n: i32, d: i32) -> i32 {
        let target = &d.to_string().chars().next().unwrap();
        let mut str = String::new();
        for i in 0..n {
            str.push_str(&(i * i).to_string());
        }
        str.chars().filter(|c| c == target).count() as i32
    }*/

    /*fn positive_sum(slice: &[i32]) -> i32 {
        slice.iter().filter(|x| **x > 0).sum()
    }*/

    /*pub fn is_happy(n: i32) -> bool {
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
    }*/

    /*pub fn convert(s: String, num_rows: i32) -> String {
        let row_size = s.len() / (num_rows + (num_rows - 2));
        let zig_zag = vec![vec![char; row_size as usize]; num_rows as usize];
        let mut i = 0;
        let mut j = 0;
        let t = s.map(|c| {
            if j >= row_size {
                i += 1;
                j = 0;
            };
            if i % row_size != 0 {};
        }).collect();
        return "".into_string();
    }*/

    /*pub fn is_match(s: String, p: String) -> bool {
        use regex::Regex;

        let re = Regex::new();
    }*/

    pub fn find_replace_string(s: String, indices: Vec<i32>, sources: Vec<String>, targets: Vec<String>) -> String {

    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::prelude::*;

    #[test]
    fn generate_data() {
        let mut lst: Vec<i32> = Vec::new();
        let mut rng = rand::thread_rng();
        for i in 0..50 {
            let f: i32 = rng.gen_range(1..std::i32::MAX);
            println!("{}", f);
        }
    }
}
