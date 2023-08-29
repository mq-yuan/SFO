// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

pub struct Solution {}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

impl Solution {
    pub fn reverse_print(head: Option<Box<ListNode>>) -> Vec<i32> {
        let mut ans = Vec::new();
        let mut res = Vec::new();
        let mut p = &head;
        while let Some(node) = p {
            ans.push(node.val);
            p = &node.next;
        }
        while let Some(v) = ans.pop() {
            res.push(v);
        }
        res
    }
}

fn vec_to_linked_list(nums: Vec<i32>) -> Option<Box<ListNode>> {
    let mut dummy = ListNode::new(0);
    let mut current = &mut dummy;

    for &num in nums.iter() {
        current.next = Some(Box::new(ListNode::new(num)));
        current = current.next.as_mut().unwrap();
    }

    dummy.next
}

fn main() {
    let input = vec![1, 3, 2];
    println!("{:?}", input);
    let head = vec_to_linked_list(input);
    let ans = Solution::reverse_print(head);

    println!("{:?}", ans);
}
