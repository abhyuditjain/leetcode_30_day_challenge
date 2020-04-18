/*
64. Minimum Path Sum

Given a m x n grid filled with non-negative numbers, find a path from top left to bottom right which minimizes the sum of all numbers along its path.

Note: You can only move either down or right at any point in time.

Example:
Input:
[
  [1,3,1],
  [1,5,1],
  [4,2,1]
]
Output: 7
Explanation: Because the path 1→3→1→1→1 minimizes the sum.
*/

pub struct Solution;

impl Solution {
    pub fn min_path_sum(mut grid: Vec<Vec<i32>>) -> i32 {
        use std::cmp::min;

        if grid.is_empty() || grid[0].is_empty() {
            return 0;
        }

        let rows = grid.len();
        let cols = grid[0].len();

        for i in 0..rows {
            for j in 0..cols {
                if i == 0 && j == 0 {
                    continue;
                }
                if i == 0 {
                    grid[i][j] += grid[i][j - 1];
                } else if j == 0 {
                    grid[i][j] += grid[i - 1][j];
                } else {
                    grid[i][j] += min(grid[i - 1][j], grid[i][j - 1]);
                }
            }
        }
        grid[rows - 1][cols - 1]
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(Solution::min_path_sum(vec![
            vec![1, 3, 1],
            vec![1, 5, 1],
            vec![4, 2, 1],
        ]), 7);
    }
}