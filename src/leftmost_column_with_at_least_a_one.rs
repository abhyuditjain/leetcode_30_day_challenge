/*
Leftmost Column with at Least a One

A binary matrix means that all elements are 0 or 1. For each individual row of the matrix, this row is sorted in non-decreasing order.
Given a row-sorted binary matrix binaryMatrix, return leftmost column index(0-indexed) with at least a 1 in it. If such index doesn't exist, return -1.

You can't access the Binary Matrix directly.  You may only access the matrix using a BinaryMatrix interface:
    BinaryMatrix.get(x, y) returns the element of the matrix at index (x, y) (0-indexed).
    BinaryMatrix.dimensions() returns a list of 2 elements [m, n], which means the matrix is m * n.

Submissions making more than 1000 calls to BinaryMatrix.get will be judged Wrong Answer.  Also, any solutions that attempt to circumvent the judge will result in disqualification.
For custom testing purposes you're given the binary matrix mat as input in the following four examples. You will not have access the binary matrix directly.

Example 1:
Input: mat = [[0,0],[1,1]]
Output: 0

Example 2:
Input: mat = [[0,0],[0,1]]
Output: 1

Example 3:
Input: mat = [[0,0],[0,0]]
Output: -1

Example 4:
Input: mat = [[0,0,0,1],[0,0,1,1],[0,1,1,1]]
Output: 1

Constraints:
m == mat.length
n == mat[i].length
1 <= m, n <= 100
mat[i][j] is either 0 or 1.
mat[i] is sorted in a non-decreasing way.
*/

use std::convert::TryInto;

pub struct Solution;

impl Solution {
    pub fn left_most_column_with_one(matrix: BinaryMatrix) -> i32 {
        let dimensions = matrix.dimensions();

        let mut i: usize = 0;
        let mut j: i32 = (dimensions[1] - 1) as i32;

        let mut ans = -1;
        while i < dimensions[0] && j >= 0 {
            if matrix.get(i, j.try_into().unwrap()) == 0 {
                i += 1;
            } else {
                ans = j;
                j -= 1;
            }
        }
        ans
    }
}

pub struct BinaryMatrix {
    v: Vec<Vec<usize>>,
}

impl BinaryMatrix {
    fn new(v: Vec<Vec<usize>>) -> Self {
        BinaryMatrix { v }
    }

    fn get(&self, x: usize, y: usize) -> usize {
        self.v[x][y]
    }
    fn dimensions(&self) -> Vec<usize> {
        vec![self.v.len(), self.v[0].len()]
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_binary_matrix_new() {
        let bm = BinaryMatrix::new(vec![vec![0, 1], vec![0, 1]]);
        assert_eq!(bm.v, vec![vec![0, 1], vec![0, 1]]);
    }

    #[test]
    fn test_binary_matrix_get() {
        let bm = BinaryMatrix::new(vec![vec![0, 1], vec![1, 0]]);
        assert_eq!(bm.get(0, 1), 1);
        assert_eq!(bm.get(1, 1), 0);
    }

    #[test]
    fn test_binary_matrix_dimensions() {
        let bm = BinaryMatrix::new(vec![vec![0, 1], vec![1, 0]]);
        assert_eq!(bm.dimensions(), vec![2, 2]);
    }

    #[test]
    fn test() {
        let bm = BinaryMatrix::new(vec![vec![0, 0], vec![1, 1]]);
        assert_eq!(Solution::left_most_column_with_one(bm), 0);

        let bm = BinaryMatrix::new(vec![vec![0, 0], vec![0, 1]]);
        assert_eq!(Solution::left_most_column_with_one(bm), 1);

        let bm = BinaryMatrix::new(vec![vec![0, 0], vec![0, 0]]);
        assert_eq!(Solution::left_most_column_with_one(bm), -1);

        let bm = BinaryMatrix::new(vec![vec![0, 0, 0, 0], vec![0, 0, 1, 1], vec![0, 1, 1, 1]]);
        assert_eq!(Solution::left_most_column_with_one(bm), 1);
    }
}