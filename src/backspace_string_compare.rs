/*
Backspace String Compare
Solution
Given two strings S and T, return if they are equal when both are typed into empty text editors. # means a backspace character.

Example 1:
Input: S = "ab#c", T = "ad#c"
Output: true
Explanation: Both S and T become "ac".

Example 2:
Input: S = "ab##", T = "c#d#"
Output: true
Explanation: Both S and T become "".

Example 3:
Input: S = "a##c", T = "#a#c"
Output: true
Explanation: Both S and T become "c".
Example 4:

Input: S = "a#c", T = "b"
Output: false
Explanation: S becomes "c" while T becomes "b".

Note:
1 <= S.length <= 200
1 <= T.length <= 200
S and T only contain lowercase letters and '#' characters.

Follow up:
Can you solve it in O(N) time and O(1) space?
*/

pub struct Solution;

impl Solution {
    pub fn backspace_compare(s: String, t: String) -> bool {
        let mut s_i = s.chars().rev();
        let mut t_i = t.chars().rev();

        loop {
            let s_char = Self::get_next(&mut s_i);
            let t_char = Self::get_next(&mut t_i);
            if s_char == None && t_char == None {
                break true
            }
            if s_char != t_char {
                break false
            }
        }
    }

    fn get_next(iter: &mut impl Iterator<Item=char>) -> Option<char> {
        let mut skip = 0;
        loop {
            let cur = iter.next();
            match cur {
                Some('#') => skip+= 1,
                None => break None,
                Some(x) => {
                    if skip == 0 {
                        break Some(x);
                    }
                    skip -= 1;
                }
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(true, Solution::backspace_compare(String::from("ab#c"), String::from("ad#c")));
        assert_eq!(true, Solution::backspace_compare(String::from("ab##"), String::from("c#d#")));
        assert_eq!(true, Solution::backspace_compare(String::from("a##c"), String::from("#a#c")));
        assert_eq!(false, Solution::backspace_compare(String::from("a#c"), String::from("b")));
    }
}