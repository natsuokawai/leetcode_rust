fn main() {
    assert_eq!(
        Solution::merge_two_lists(
            Some(Box::new(ListNode { val: 1, next: None })),
            Some(Box::new(ListNode { val: 2, next: None }))
        ),
        Some(Box::new(ListNode {
            val: 1,
            next: Some(Box::new(ListNode { val: 2, next: None })),
        }))
    );

    assert_eq!(
        Solution::merge_two_lists(None, Some(Box::new(ListNode { val: 0, next: None }))),
        Some(Box::new(ListNode { val: 0, next: None }))
    );

    assert_eq!(Solution::merge_two_lists(None, None), None);

    let l1 = Some(Box::new(ListNode {
        val: 2,
        next: Some(Box::new(ListNode { val: 4, next: None })),
    }));
    let l2 = Some(Box::new(ListNode {
        val: 1,
        next: Some(Box::new(ListNode { val: 3, next: None })),
    }));
    let expected = Some(Box::new(ListNode {
        val: 1,
        next: Some(Box::new(ListNode {
            val: 2,
            next: Some(Box::new(ListNode {
                val: 3,
                next: Some(Box::new(ListNode { val: 4, next: None })),
            })),
        })),
    }));
    assert_eq!(Solution::merge_two_lists(l1, l2), expected);
}

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

struct Solution;

impl Solution {
    pub fn merge_two_lists(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        None
    }
}
