use std::collections::HashMap;
use std::cmp;

fn main() {
    assert_eq!(
        Solution::length_of_longest_substring(String::from("pwwkew")),
        3
    )
}

struct Solution;

impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let mut map = HashMap::new();
        let mut max_len = 0;
        let mut current = 0;
        let mut before = -1;

        for c in s.chars() {
            // if map already contains 'c', returns None
            // update 'before' as base pointer
            if let Some(last) = map.insert(c, current) {
                before = cmp::max(before, last);
            } 
            max_len = cmp::max(max_len, current - before);
            current += 1;
        }

        max_len
    }
}
