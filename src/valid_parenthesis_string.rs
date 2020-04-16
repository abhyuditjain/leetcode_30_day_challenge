/*
678. Valid Parenthesis String

Given a string containing only three types of characters: '(', ')' and '*', write a function to check whether this string is valid.

We define the validity of a string by these rules:
1. Any left parenthesis '(' must have a corresponding right parenthesis ')'.
2. Any right parenthesis ')' must have a corresponding left parenthesis '('.
3. Left parenthesis '(' must go before the corresponding right parenthesis ')'.
4. '*' could be treated as a single right parenthesis ')' or a single left parenthesis '(' or an empty string.
5. An empty string is also valid.

Example 1:
Input: "()"
Output: True

Example 2:
Input: "(*)"
Output: True

Example 3:
Input: "(*))"
Output: True

Note:
The string size will be in the range [1, 100].
*/

pub struct Solution;

impl Solution {
    pub fn check_valid_string(s: String) -> bool {
        let mut parentheses = vec![];
        let mut stars = vec![];

        for (i, c) in s.chars().enumerate() {
            match c {
                '(' => parentheses.push(i),
                '*' => stars.push(i),
                _ => {
                    if parentheses.is_empty() && stars.is_empty() {
                        return false;
                    }
                    if !parentheses.is_empty() {
                        parentheses.pop();
                    } else {
                        stars.pop();
                    }
                }
            }
        }

        while !parentheses.is_empty() && !stars.is_empty() {
            if parentheses.pop().unwrap() > stars.pop().unwrap() {
                return false;
            }
        }
        return parentheses.is_empty();
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(Solution::check_valid_string(String::from("()")), true);
        assert_eq!(Solution::check_valid_string(String::from("(*)")), true);
        assert_eq!(Solution::check_valid_string(String::from("(*))")), true);
        assert_eq!(Solution::check_valid_string(String::from("())")), false);
    }
}