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

fn vec_to_link_list(num: Vec<i32>) -> Option<Box<ListNode>> {
    let mut head = ListNode::new(0);
    let mut current = &mut head;
    for &num in num.iter() {
        current.next = Some(Box::new(ListNode::new(num)));
        current = current.next.as_mut().unwrap();
    }
    head.next
}

impl Solution {
    pub fn reverse_print(head: Option<Box<ListNode>>) -> Vec<i32> {
        let mut res: Vec<i32> = Vec::new();
        Solution::icur(head, &mut res);
        res
    }

    pub fn icur(head: Option<Box<ListNode>>, res: &mut Vec<i32>) {
        match head {
            Some(node) => {
                Solution::icur(node.next, res);
                res.push(node.val);
            }
            None => {}
        }
    }
}

fn main() {
    let input = vec![1, 3, 2];
    println!("{:?}", input);
    let input = vec_to_link_list(input);
    let ans = Solution::reverse_print(input);
    println!("{:?}", ans);
}
