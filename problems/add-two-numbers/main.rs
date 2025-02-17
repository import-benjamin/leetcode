fn extract_number(n: Option<Box<ListNode>>) -> i128 {
    match n {
        Some(boxed_listnode) => {
            match boxed_listnode.next {
                Some(next_boxed_listnode) => i128::from(boxed_listnode.val) + extract_number(Some(next_boxed_listnode)) * 10,
                None => i128::from(boxed_listnode.val)
            }
        }
        None => 0
    }
}

fn decompose(mut num: i128) -> Vec<u8> {
    if num == 0 {
        return vec![0];
    }
    let mut digits = Vec::new();
    while num > 0 {
        digits.push((num % 10) as u8);
        num /= 10;
    }
    digits
}

fn extract_listnode(n: i128) -> Option<Box<ListNode>> {
    let mut r = decompose(n);
    let t = ListNode::new(r.pop().unwrap() as i32);
    r.reverse();
    let f = r.iter().fold(t, |s, x| ListNode {val: i32::from(*x), next: Some(Box::new(s))});
    Some(Box::new(f))
}

impl Solution {
    pub fn add_two_numbers(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let n1 = extract_number(l1);
        let n2 = extract_number(l2);
        println!("{:?} + {:?} = {:?}", n1, n2, n1+n2);
        extract_listnode(n1 + n2)
    }
}
