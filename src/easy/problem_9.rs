pub struct Solution {}

/*
Given an integer x, return true if x is palindrome integer.
An integer is a palindrome when it reads the same backward as forward.
For example, 121 is a palindrome while 123 is not.
 */
impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        let str = x.to_string();
        return str == str.chars().rev().collect::<String>()
    }
}