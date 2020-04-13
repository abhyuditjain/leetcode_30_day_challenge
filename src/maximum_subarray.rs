/*
53. Maximum Subarray

Given an integer array nums, find the contiguous subarray (containing at least one number) which has the largest sum and return its sum.

Example:

Input: [-2,1,-3,4,-1,2,1,-5,4],
Output: 6
Explanation: [4,-1,2,1] has the largest sum = 6.

Follow up:
If you have figured out the O(n) solution, try coding another solution using the divide and conquer approach, which is more subtle.
*/

pub struct Solution;

impl Solution {
    pub fn max_sub_array(nums: Vec<i32>) -> i32 {
        let mut sum: i32 = nums[0];
        let mut max: i32 = nums[0];

        for &i in nums.iter().skip(1) {
            if sum < 0 {
                sum = i;
            } else {
                sum += i;
            }
            if sum > max {
                max = sum;
            }
        }
        max
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(6, Solution::max_sub_array(vec![-2, 1, -3, 4, -1, 2, 1, -5, 4]));
        assert_eq!(6, Solution::max_sub_array(vec![-1, 1, 2, 3, -2, 0]));
    }
}