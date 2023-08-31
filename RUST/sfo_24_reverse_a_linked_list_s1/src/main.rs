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

impl Solution {
    // Do not change origion link list head
    // pub fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    //     let mut res = ListNode::new(0);
    //     let mut current = &head;
    //     while let Some(node) = current {
    //         current = &node.next;
    //         let mut new_node = node.clone();
    //         new_node.next = res.next.take();
    //         res.next = Some(new_node);
    //     }
    //     res.next
    // }

    pub fn reverse_list(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut res = None;
        while let Some(mut node) = head {
            head = node.next.take();
            node.next = res;
            res = Some(node);
        }
        res
    }
}

pub struct Utils {}

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
