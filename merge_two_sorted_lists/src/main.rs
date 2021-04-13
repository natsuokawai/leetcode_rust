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

struct Solution;

impl Solution {
    pub fn merge_two_lists(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        match (l1, l2) {
            (None, None) => None,
            (Some(n), None) => Some(n),
            (None, Some(n)) => Some(n),
            (Some(n1), Some(n2)) => {
                if n1.val <= n2.val {
                    Some(Box::new(ListNode {
                        val: n1.val,
                        next: Self::merge_two_lists(n1.next, Some(n2)),
                    }))
                } else {
                    Some(Box::new(ListNode {
                        val: n2.val,
                        next: Self::merge_two_lists(n2.next, Some(n1)),
                    }))
                }
            }
        }
    }
}
