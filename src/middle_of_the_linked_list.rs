/*
876. Middle of the Linked List

Given a non-empty, singly linked list with head node head, return a middle node of linked list.

If there are two middle nodes, return the second middle node.

Example 1:
Input: [1,2,3,4,5]
Output: Node 3 from this list (Serialization: [3,4,5])
The returned node has value 3.  (The judge's serialization of this node is [3,4,5]).
Note that we returned a ListNode object ans, such that:
ans.val = 3, ans.next.val = 4, ans.next.next.val = 5, and ans.next.next.next = NULL.

Example 2:
Input: [1,2,3,4,5,6]
Output: Node 4 from this list (Serialization: [4,5,6])
Since the list has two middle nodes with values 3 and 4, we return the second one.


Note:
The number of nodes in the given list will be between 1 and 100.
*/

use crate::utils::linked_list::ListNode;

pub struct Solution;

impl Solution {
    pub fn middle_node(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut slow = &head;
        let mut fast = &head;

        while fast.is_some() && fast.as_ref().unwrap().next.is_some() {
            slow = &slow.as_ref().unwrap().next;
            fast = &fast.as_ref().unwrap().next.as_ref().unwrap().next;
        }
        slow.clone()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        let mut node1 = Box::new(ListNode::new(1));
        let mut node2 = Box::new(ListNode::new(2));
        let mut middle = Box::new(ListNode::new(3));
        let mut node4 = Box::new(ListNode::new(4));
        let mut node5 = Box::new(ListNode::new(5));
        let node6 = Box::new(ListNode::new(6));

        node4.next = Some(node5.clone());
        middle.next = Some(node4.clone());
        node2.next = Some(middle.clone());
        node1.next = Some(node2.clone());

        assert_eq!(Solution::middle_node(Some(node1.clone())), Some(middle.clone()));

        node5.next = Some(node6.clone());
        node4.next = Some(node5.clone());
        middle.next = Some(node4.clone());
        node2.next = Some(middle.clone());
        node1.next = Some(node2.clone());

        assert_eq!(Solution::middle_node(Some(node1.clone())), Some(node4.clone()));
    }
}