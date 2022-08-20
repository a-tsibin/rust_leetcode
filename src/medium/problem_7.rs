pub struct Solution {}

/*
Given a signed 32-bit integer x, return x with its digits reversed. If reversing x causes the value
to go outside the signed 32-bit integer range [-231, 231 - 1], then return 0.
 */
impl Solution {
    pub fn reverse(x: i32) -> i32 {
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
    }
}