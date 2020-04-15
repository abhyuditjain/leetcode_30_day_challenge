/*
238. Product of Array Except Self

Given an array nums of n integers where n > 1,  return an array output such that output[i] is equal to the product of all the elements of nums except nums[i].

Example:
Input:  [1,2,3,4]
Output: [24,12,8,6]
Constraint: It's guaranteed that the product of the elements of any prefix or suffix of the array (including the whole array) fits in a 32 bit integer.

Note: Please solve it without division and in O(n).

Follow up:
Could you solve it with constant space complexity? (The output array does not count as extra space for the purpose of space complexity analysis.)
*/

pub struct Solution;

impl Solution {
    pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
        let mut output = vec![1; nums.len()];

        let mut left = 1;
        let mut right = 1;

        for i in 1..nums.len() {
            left *= nums[i - 1];
            output[i] *= left;

            right *= nums[nums.len() - i];
            output[nums.len() - i - 1] *= right;
        }

        output
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(Solution::product_except_self(vec![1, 2, 3, 4]), vec![24, 12, 8, 6]);
    }
}