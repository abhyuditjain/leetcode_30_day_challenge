/*
200. Number of Islands

Given a 2d grid map of '1's (land) and '0's (water), count the number of islands.
An island is surrounded by water and is formed by connecting adjacent lands horizontally or vertically.
You may assume all four edges of the grid are all surrounded by water.

Example 1:
Input:
11110
11010
11000
00000
Output: 1

Example 2:
Input:
11000
11000
00100
00011
Output: 3
*/

pub struct Solution;

impl Solution {
    pub fn num_islands(mut grid: Vec<Vec<char>>) -> i32 {
        let mut res = 0;

        for i in 0..grid.len() {
            for j in 0..grid.get(0).unwrap().len() {
                if grid.get(i).unwrap().get(j).unwrap() == &'1' {
                    Self::dfs(&mut grid, i, j);
                    res += 1;
                }
            }
        }
        res
    }

    fn dfs(grid: &mut Vec<Vec<char>>, i: usize, j: usize) {
        if i == grid.len() || j == grid[0].len() || grid[i][j] == '0' {
            return;
        }
        grid[i][j] = '0';

        if i != 0 {
            Self::dfs(grid, i - 1, j);
        }
        if j != 0 {
            Self::dfs(grid, i, j - 1);
        }
        Self::dfs(grid, i + 1, j);
        Self::dfs(grid, i, j + 1);
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(Solution::num_islands(vec![
            vec!['1', '1', '1', '1', '0'],
            vec!['1', '1', '0', '1', '0'],
            vec!['1', '1', '0', '0', '0'],
            vec!['0', '0', '0', '0', '0'],
        ]), 1);

        assert_eq!(Solution::num_islands(vec![
            vec!['1', '1', '0', '0', '0'],
            vec!['1', '1', '0', '0', '0'],
            vec!['0', '0', '1', '0', '0'],
            vec!['0', '0', '0', '1', '1'],
        ]), 3);
    }
}