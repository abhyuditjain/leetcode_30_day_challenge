/*
Given the root of a binary tree, the level of its root is 1, the level of its children is 2, and so on.

Return the smallest level X such that the sum of all the values of nodes at level X is maximal.

Example 1:
Input: [1,7,0,7,-8,null,null]
Output: 2
Explanation:
Level 1 sum = 1.
Level 2 sum = 7 + 0 = 7.
Level 3 sum = 7 + -8 = -1.
So we return the level with the maximum sum which is level 2.

Note:
The number of nodes in the given tree is between 1 and 10^4.
-10^5 <= node.val <= 10^5
*/

use std::rc::Rc;
use std::cell::RefCell;
use crate::utils::binary_tree::TreeNode;

pub struct Solution;

impl Solution {
    pub fn max_level_sum(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut level = 1;
        let mut level_max = std::i32::MIN;
        let mut level_max_sum = std::i32::MIN;
        let mut stack = vec![root];

        while !stack.is_empty() {
            let mut next_level_stack = vec![];
            let mut current_level_sum = 0;

            for node in stack {
                match node {
                    None => {}
                    Some(v) => {
                        let node = Rc::try_unwrap(v).unwrap().into_inner();
                        current_level_sum += node.val;
                        next_level_stack.push(node.left);
                        next_level_stack.push(node.right);
                    }
                }
            }

            if current_level_sum > level_max_sum {
                level_max_sum = current_level_sum;
                level_max = level;
            }
            level += 1;
            stack = next_level_stack;
        }

        level_max
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        let mut root = Some(Rc::new(RefCell::new(TreeNode::new(1))));
        root.as_ref().unwrap().borrow_mut().left = Some(Rc::new(RefCell::new(TreeNode::new(7))));
        root.as_ref().unwrap().borrow_mut().right = Some(Rc::new(RefCell::new(TreeNode::new(0))));
        root.as_ref().unwrap().borrow_mut().left.as_ref().unwrap().borrow_mut().left = Some(Rc::new(RefCell::new(TreeNode::new(7))));
        root.as_ref().unwrap().borrow_mut().right.as_ref().unwrap().borrow_mut().right = Some(Rc::new(RefCell::new(TreeNode::new(-8))));

        assert_eq!(Solution::max_level_sum(root), 2);
    }
}