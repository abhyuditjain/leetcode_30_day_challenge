/*
Diameter of Binary Tree
Solution
Given a binary tree, you need to compute the length of the diameter of the tree. The diameter of a binary tree is the length of the longest path between any two nodes in a tree. This path may or may not pass through the root.

Example:
Given a binary tree
          1
         / \
        2   3
       / \
      4   5
Return 3, which is the length of the path [4,2,1,3] or [5,2,1,3].

Note: The length of path between two nodes is represented by the number of edges between them.
*/

use std::rc::Rc;
use std::cell::RefCell;
use std::cmp::max;
use crate::utils::binary_tree::TreeNode;

pub struct Solution;

impl Solution {
    pub fn diameter_of_binary_tree(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        if root.is_some() {
            let (_max_depth, diameter) = Self::max_depth_diameter(&root);
            diameter
        } else {
            0
        }
    }

    fn max_depth_diameter(root: &Option<Rc<RefCell<TreeNode>>>) -> (i32, i32) {
        if let Some(node) = root {
            let node = node.borrow();

            let (left_depth, left_diameter) = Self::max_depth_diameter(&node.left);
            let (right_depth, right_diameter) = Self::max_depth_diameter(&node.right);

            let max_depth = 1 + max(left_depth, right_depth);
            let max_diameter = [left_diameter, right_diameter, left_depth + right_depth]
                .iter()
                .max()
                .unwrap()
                .clone();

            (max_depth, max_diameter)
        } else {
            (0, 0)
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        let root = Rc::new(RefCell::new(TreeNode::new(1)));
        let left = Rc::new(RefCell::new(TreeNode::new(2)));
        let right = Rc::new(RefCell::new(TreeNode::new(3)));
        left.borrow_mut().left = Some(Rc::new(RefCell::new(TreeNode::new(4))));
        left.borrow_mut().right = Some(Rc::new(RefCell::new(TreeNode::new(5))));

        root.borrow_mut().left = Some(left);
        root.borrow_mut().right = Some(right);

        assert_eq!(Solution::diameter_of_binary_tree(Some(root)), 3);
    }
}