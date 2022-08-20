pub struct Solution {}

/*
Implement pow(x, n), which calculates x raised to the power n (i.e., xn).
 */
impl Solution {
    pub fn my_pow(x: f64, n: i32) -> f64 {
        fn fast_pow(y: f64, x: f64, n: i64) -> f64 {
            match n {
                0 => y,
                n if n < 0 => fast_pow(y, 1.0 / x, -n),
                n if n % 2 == 0 => fast_pow(y, x * x, n >> 1),
                _ => fast_pow(y * x, x * x, (n - 1) >> 1),
            }
        }
        fast_pow(1.0, x, n as i64)
    }
}