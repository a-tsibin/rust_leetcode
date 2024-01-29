pub struct Solution {}

/*
An ugly number is a positive integer whose prime factors are limited to 2, 3, and 5.

Given an integer n, return true if n is an ugly number.
 */
impl Solution {
    pub fn is_ugly(n: i32) -> bool {
        if n < 1 {
            return false
        }
        if n < 7 {
            return true
        }

        let mut temp = n;
        while temp % 2 == 0 {
            temp = temp / 2;
        }
        while temp % 3 == 0 {
            temp = temp / 3;
        }
        while temp % 5 == 0 {
            temp = temp / 5;
        }

        return temp == 1
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn check() {
        assert_eq!(Solution::is_ugly(1), true);
        assert_eq!(Solution::is_ugly(6), true);
        assert_eq!(Solution::is_ugly(14), false);
        assert_eq!(Solution::is_ugly(1332185066), false);
    }
}