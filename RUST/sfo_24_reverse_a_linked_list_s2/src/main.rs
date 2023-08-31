// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}
//
impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

pub struct Solution {}
pub struct Utils {}

impl Solution {
    pub fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        Solution::recur(head)
    }
}

impl Utils {
    pub fn vec_to_link_list(num: Vec<i32>) -> Option<Box<ListNode>> {
        let mut head = ListNode::new(0);
        let mut current = &mut head;
        for &num in num.iter() {
            current.next = Some(Box::new(ListNode::new(num)));
            current = current.next.as_mut().unwrap();
        }
        head.next
    }

    pub fn print_link_list(head: &Option<Box<ListNode>>) {
        let mut current = head;
        while let Some(node) = current {
            print!("{}->", node.val);
            current = &node.next;
        }
        println!("NULL");
    }
}

fn main() {
    let input = vec![1, 2, 3, 4, 5];
    let input = Utils::vec_to_link_list(input);
    Utils::print_link_list(&input);
    let ans = Solution::reverse_list(input);
    Utils::print_link_list(&ans);
}
