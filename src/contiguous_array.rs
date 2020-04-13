/*
Contiguous Array

Given a binary array, find the maximum length of a contiguous subarray with equal number of 0 and 1.

Example 1:
Input: [0,1]
Output: 2
Explanation: [0, 1] is the longest contiguous subarray with equal number of 0 and 1.

Example 2:
Input: [0,1,0]
Output: 2
Explanation: [0, 1] (or [1, 0]) is a longest contiguous subarray with equal number of 0 and 1.

Note: The length of the given binary array will not exceed 50,000.
*/

use std::collections::HashMap;

pub struct Solution;

impl Solution {
    pub fn find_max_length(nums: Vec<i32>) -> i32 {
        let mut sum_to_index = HashMap::<i32, i32>::new();
        sum_to_index.insert(0, -1);

        let mut sum = 0;
        let mut max = 0;

        for (i, &x) in nums.iter().enumerate() {
            sum = match x {
                0 => sum - 1,
                _ => sum + 1,
            };
            if sum_to_index.contains_key(&sum) {
                max = std::cmp::max(max, i as i32 - *sum_to_index.get(&sum).unwrap());
            } else {
                sum_to_index.insert(sum, i as i32);
            }

            sum_to_index.entry(sum);
        }

        max
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(Solution::find_max_length(vec![0, 1]), 2);
        assert_eq!(Solution::find_max_length(vec![0, 1, 0]), 2);
        assert_eq!(Solution::find_max_length(vec![0]), 0);
    }
}