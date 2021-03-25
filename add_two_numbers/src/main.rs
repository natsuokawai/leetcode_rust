fn main() {
    println!("Hello, world!");
}

// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
  pub val: i32,
  pub next: Option<Box<ListNode>>
}

impl ListNode {
  #[inline]
  fn new(val: i32) -> Self {
    ListNode {
      next: None,
      val
    }
  }
}

impl Iterator for ListNode {
    type Item = Box::<ListNode>;

    // next() is the only required method
    fn next(&mut self) -> Option<Self::Item> {
				self.next.clone()
    }
}

struct Solution;

impl Solution {
    pub fn add_two_numbers(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
				let mut sum = Solution::list_to_num(&l1) + Solution::list_to_num(&l2);

        Some(Box::new(ListNode::new(1)))
    }

		fn list_to_num(list: &Option<Box<ListNode>>) -> i32 {
        list.iter().rev().enumerate().map(|(i, node)| node.val * 10i32.pow(i as u32)).sum()
    }
}
