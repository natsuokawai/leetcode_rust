use std::collections::HashMap;

fn main() {
    assert_eq!(Solution::two_sum(vec![2, 7, 11, 15], 9), vec![0, 1]);
}

struct Solution;

impl Solution {
    fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut num_hash: HashMap<i32, i32> = HashMap::new();

        for (i, num) in nums.iter().enumerate() {
            match num_hash.get(num) {
                Some(n) => return vec![*n, i as i32],
                None => num_hash.insert(target - num, i as i32),
            };
        }

        vec![]
    }
}
