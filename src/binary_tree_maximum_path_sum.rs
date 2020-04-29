/*
Binary Tree Maximum Path Sum

Given a non-empty binary tree, find the maximum path sum.

For this problem, a path is defined as any sequence of nodes from some starting node to any node in the tree along the parent-child connections.
The path must contain at least one node and does not need to go through the root.

Example 1:
Input: [1,2,3]

       1
      / \
     2   3

Output: 6

Example 2:
Input: [-10,9,20,null,null,15,7]

   -10
   / \
  9  20
    /  \
   15   7

Output: 42
*/

pub struct Solution;

use std::rc::Rc;
use std::cell::RefCell;
use crate::utils::binary_tree::TreeNode;

impl Solution {
    pub fn max_path_sum(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut sum = std::i32::MIN;
        Self::max_path_sum_helper(root, &mut sum);
        sum
    }

    fn max_path_sum_helper(node: Option<Rc<RefCell<TreeNode>>>, sum: &mut i32) -> i32 {
        if node.is_none() {
            return 0;
        }

        let left = std::cmp::max(0, Self::max_path_sum_helper(node.as_ref().unwrap().borrow().left.clone(), sum));
        let right = std::cmp::max(0, Self::max_path_sum_helper(node.as_ref().unwrap().borrow().right.clone(), sum));

        *sum = std::cmp::max(*sum, left + right + node.as_ref().unwrap().borrow().val.clone());

        std::cmp::max(left, right) + node.as_ref().unwrap().borrow().val
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1() {
        let root = Rc::new(RefCell::new(TreeNode::new(1)));
        root.borrow_mut().left = Some(Rc::new(RefCell::new(TreeNode::new(2))));
        root.borrow_mut().right = Some(Rc::new(RefCell::new(TreeNode::new(3))));

        assert_eq!(Solution::max_path_sum(Some(root)), 6);
    }

    #[test]
    fn test_2() {
        let root = Rc::new(RefCell::new(TreeNode::new(-10)));
        root.borrow_mut().left = Some(Rc::new(RefCell::new(TreeNode::new(9))));
        root.borrow_mut().right = Some(Rc::new(RefCell::new(TreeNode::new(20))));
        root.borrow_mut().right.as_ref().unwrap().borrow_mut().left = Some(Rc::new(RefCell::new(TreeNode::new(15))));
        root.borrow_mut().right.as_ref().unwrap().borrow_mut().right = Some(Rc::new(RefCell::new(TreeNode::new(7))));


        assert_eq!(Solution::max_path_sum(Some(root)), 42);
    }
}