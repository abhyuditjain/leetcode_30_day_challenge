/*
Counting Elements

Given an integer array arr, count element x such that x + 1 is also in arr.

If there're duplicates in arr, count them separately.

Example 1:
Input: arr = [1,2,3]
Output: 2
Explanation: 1 and 2 are counted cause 2 and 3 are in arr.

Example 2:
Input: arr = [1,1,3,3,5,5,7,7]
Output: 0
Explanation: No numbers are counted, cause there's no 2, 4, 6, or 8 in arr.

Example 3:
Input: arr = [1,3,2,3,5,0]
Output: 3
Explanation: 0, 1 and 2 are counted cause 1, 2 and 3 are in arr.

Example 4:
Input: arr = [1,1,2,2]
Output: 2
Explanation: Two 1s are counted cause 2 is in arr.


Constraints:
1 <= arr.length <= 1000
0 <= arr[i] <= 1000
*/

use std::collections::HashSet;

pub struct Solution;

impl Solution {
    pub fn count_elements(arr: Vec<i32>) -> i32 {
        let set: HashSet<i32> = arr.iter().cloned().collect();
        arr.iter().fold(0, |count, &x| if set.contains(&(x + 1)) { count + 1 } else { count })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(2, Solution::count_elements(vec![1, 2, 3]));
        assert_eq!(0, Solution::count_elements(vec![1, 1, 3, 3, 5, 5, 7, 7]));
        assert_eq!(3, Solution::count_elements(vec![1, 3, 2, 3, 5, 0]));
        assert_eq!(2, Solution::count_elements(vec![1, 1, 2, 2]));
    }
}