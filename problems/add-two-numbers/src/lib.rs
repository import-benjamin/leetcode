// Runtime: 0ms, 2.4MB
// You are given two non-empty linked lists representing two non-negative integers. The digits are stored in reverse order, and each of their nodes contains a single digit. Add the two numbers and return the sum as a linked list.
// You may assume the two numbers do not contain any leading zero, except the number 0 itself.
extern crate leetcode_common;
use leetcode_common::ListNode;

pub fn add_two_numbers(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let (mut p1, mut p2) = (l1.as_deref(), l2.as_deref()); // Pointers to traverse the linked lists
    let mut dummy = ListNode { val: 0, next: None }; // Dummy node to simplify list construction
    let mut tail = &mut dummy; // Pointer to the last node in the result list
    let mut carry = 0; // Stores carry-over value

    while p1.is_some() || p2.is_some() || carry > 0 {
        let sum = p1.map_or(0, |n| n.val) + p2.map_or(0, |n| n.val) + carry; // Compute sum
        carry = sum / 10; // Determine new carry value
        tail.next = Some(Box::new(ListNode { val: sum % 10, next: None })); // Store digit in new node
        tail = tail.next.as_mut().unwrap(); // Move tail forward
        p1 = p1.and_then(|n| n.next.as_deref()); // Advance pointer in first list
        p2 = p2.and_then(|n| n.next.as_deref()); // Advance pointer in second list
    }

    dummy.next // Return the sum list, skipping dummy node
}
