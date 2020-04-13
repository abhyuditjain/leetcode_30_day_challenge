/*
202. Happy Number

Write an algorithm to determine if a number n is "happy".

A happy number is a number defined by the following process: Starting with any positive integer, replace the number by the sum of the squares of its digits, and repeat the process until the number equals 1 (where it will stay), or it loops endlessly in a cycle which does not include 1. Those numbers for which this process ends in 1 are happy numbers.

Return True if n is a happy number, and False if not.

Example:
Input: 19
Output: true
Explanation:
1^2 + 9^2 = 82
8^2 + 2^2 = 68
6^2 + 8^2 = 100
1^2 + 0^2 + 0^2 = 1
*/

pub struct Solution;

impl Solution {
    pub fn is_happy(n: i32) -> bool {
        if n <= 0 {
            return false;
        }
        let mut slow = n;
        let mut fast = Solution::square_sum(n);
        while fast != 1 && slow != fast {
            slow = Solution::square_sum(slow);
            fast = Solution::square_sum(fast);
            fast = Solution::square_sum(fast);
        }

        fast == 1
    }

    fn square_sum(mut n: i32) -> i32 {
        let mut sum: i32 = 0;
        while n > 0 {
            let digit = n % 10;
            sum += digit * digit;
            n /= 10;
        }
        sum
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(true, Solution::is_happy(19));
        assert_eq!(false, Solution::is_happy(20));
    }
}