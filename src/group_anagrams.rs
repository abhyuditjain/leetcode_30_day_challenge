/*
49. Group Anagrams

Given an array of strings, group anagrams together.

Example:

Input: ["eat", "tea", "tan", "ate", "nat", "bat"],
Output:
[
["ate","eat","tea"],
["nat","tan"],
["bat"]
]
Note:

All inputs will be in lowercase.
The order of your output does not matter.
*/

use std::collections::HashMap;

pub struct Solution;

impl Solution {
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        let mut h = HashMap::<String, Vec<String>>::new();

        for s in strs {
            h.entry(Self::get_sorted(&s)).or_default().push(s);
        }

        h.into_iter().map(|(_, v)| v).collect()
    }

    fn get_sorted(s: &str) -> String {
        let mut chars: Vec<_> = s.chars().collect();
        chars.sort();
        chars.iter().collect()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::utils::test_tools::{assert_nested_equivalent, map_to_string};

    #[test]
    fn test() {
        let input = map_to_string(&["eat", "tea", "tan", "ate", "nat", "bat"]);
        let output = Solution::group_anagrams(input);
        let expected = vec![
            map_to_string(&["ate","eat","tea"]),
            map_to_string(&["nat","tan"]),
            map_to_string(&["bat"])
        ];
        assert_nested_equivalent(&output, &expected);
    }
}