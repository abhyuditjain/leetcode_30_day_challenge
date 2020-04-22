/*
Construct Binary Search Tree from Preorder Traversal

Return the root node of a binary search tree that matches the given preorder traversal.
(Recall that a binary search tree is a binary tree where for every node, any descendant of node.left has a value < node.val, and any descendant of node.right has a value > node.val.  Also recall that a preorder traversal displays the value of the node first, then traverses node.left, then traverses node.right.)

Example 1:
Input: [8,5,1,7,10,12]
Output: [8,5,10,1,7,null,12]

Note:
1 <= preorder.length <= 100
The values of preorder are distinct.
*/

use std::cell::RefCell;
use std::rc::Rc;

use crate::utils::binary_tree::TreeNode;

pub struct Solution;

impl Solution {
    pub fn bst_from_preorder(preorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        Self::bst_helper(&preorder)
    }

    fn bst_helper(preorder: &[i32]) -> Option<Rc<RefCell<TreeNode>>> {
        if let Some(first) = preorder.first() {
            let node = Some(Rc::new(RefCell::new(TreeNode::new(*first))));
            let i = (1..preorder.len()).find(|&i| preorder[i] > preorder[0]).unwrap_or_else(|| preorder.len());
            node.as_ref().unwrap().borrow_mut().left = Self::bst_helper(&preorder[1..i]);
            node.as_ref().unwrap().borrow_mut().right = Self::bst_helper(&preorder[i..]);
            node
        } else {
            None
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        let root = Some(Rc::new(RefCell::new(TreeNode::new(8))));
        root.as_ref().unwrap().borrow_mut().left = Some(Rc::new(RefCell::new(TreeNode::new(5))));
        root.as_ref().unwrap().borrow_mut().right = Some(Rc::new(RefCell::new(TreeNode::new(10))));
        root.as_ref().unwrap().borrow_mut().left.as_ref().unwrap().borrow_mut().left = Some(Rc::new(RefCell::new(TreeNode::new(1))));
        root.as_ref().unwrap().borrow_mut().left.as_ref().unwrap().borrow_mut().right = Some(Rc::new(RefCell::new(TreeNode::new(7))));
        root.as_ref().unwrap().borrow_mut().right.as_ref().unwrap().borrow_mut().right = Some(Rc::new(RefCell::new(TreeNode::new(12))));

        assert_eq!(Solution::bst_from_preorder(vec![8, 5, 1, 7, 10, 12]), root);
    }
}