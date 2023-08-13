use std::collections::VecDeque;

pub struct Solution {}

/*
A certain bug's home is on the x-axis at position x. Help them get there from position 0.

The bug jumps according to the following rules:

It can jump exactly a positions forward (to the right).
It can jump exactly b positions backward (to the left).
It cannot jump backward twice in a row.
It cannot jump to any forbidden positions.
The bug may jump forward beyond its home, but it cannot jump to positions numbered with negative integers.

Given an array of integers forbidden, where forbidden[i] means that the bug cannot jump to the position forbidden[i], and integers a, b, and x, return the minimum number of jumps needed for the bug to reach its home. If there is no possible sequence of jumps that lands the bug on position x, return -1.
*/

impl Solution {
    pub fn minimum_jumps(forbidden: Vec<i32>, a: i32, b: i32, x: i32) -> i32 {
        fn bfs(
            forbidden: Vec<i32>,
            a: i32,
            b: i32,
            x: i32,
            visited: Vec<i32>,
            unvisited: VecDeque<(i32, bool)>,
        ) -> i32 {
            if unvisited.is_empty() {
                return -1
            };

            let (pos, prev_back) = *unvisited.front().unwrap();
            if pos == x {
                return visited.len() as i32
            };

            if pos > x + a + b {
                return -1
            };

            let mut visited = visited.clone();
            visited.push(pos);
            let mut unvisited = unvisited.into_iter().skip(1).collect::<VecDeque<_>>();

            let right = pos + a;
            if !forbidden.contains(&right) && !visited.contains(&right) {
                unvisited.push_back((right, false));
            }
            let right_res = bfs(
                forbidden.clone(),
                a,
                b,
                x,
                visited.clone(),
                unvisited.clone(),
            );

            let left = pos - b;
            if !prev_back && left > 0 && !forbidden.contains(&left) && !visited.contains(&left) {
                unvisited.push_back((left, true));
            }
            let left_res = bfs(forbidden, a, b, x, visited, unvisited);
            match (right_res, left_res) {
                (r, l) if r == -1 => l,
                (r, l) if l == -1 => r,
                (r, l) => r.min(l),
            }
        }

        let visited: Vec<i32> = vec![];
        let unvisited: VecDeque<(i32, bool)> = VecDeque::from(vec![(0, false)]);

        bfs(forbidden, a, b, x, visited, unvisited)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_1() {
        let res = Solution::minimum_jumps(vec![14, 4, 18, 1, 15], 3, 15, 9);
        assert_eq!(res, 3)
    }

    #[test]
    fn case_2() {
        let res = Solution::minimum_jumps(vec![8, 3, 16, 6, 12, 20], 15, 13, 11);
        assert_eq!(res, -1)
    }

    #[test]
    fn case_3() {
        let res = Solution::minimum_jumps(vec![1, 6, 2, 14, 5, 17, 4], 16, 9, 7);
        assert_eq!(res, 2)
    }
}
