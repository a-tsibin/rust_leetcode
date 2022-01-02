pub struct Solution {}

/*
Given a string s containing just the characters '(', ')', '{', '}', '[' and ']', determine if the input string is valid.

An input string is valid if:

Open brackets must be closed by the same type of brackets.
Open brackets must be closed in the correct order.
 */
impl Solution {
    pub fn is_valid(s: String) -> bool {
        use std::collections::HashMap;
        let brackets_match: HashMap<char, char> = [
            (')', '('),
            (']', '['),
            ('}', '{')
        ].iter().cloned().collect();
        let mut stack: Vec<char> = Vec::new();
        for c in s.chars() {
            if c == ')' || c == '}' || c == ']' {
                if brackets_match.get(&c).map(|e| *e != stack.pop().unwrap_or('a')).unwrap_or(true) {
                    return false;
                }
            } else {
                stack.push(c)
            }
        }
        stack.len() == 0
    }
}