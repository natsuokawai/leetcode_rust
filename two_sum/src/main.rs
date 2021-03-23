fn main() {
	Solution::two_sum(vec![2, 7, 11, 15], 9);
}

struct Solution;

impl Solution {
    fn two_sum(nums: Vec::<i32>, target: i32) -> Vec::<i32> {
        let mut result = Vec::new();

        for (i, num_i) in nums.iter().enumerate() {
            for (j, num_j) in nums[i+1..].iter().enumerate() {
                if num_i + num_j == target {
                    result = vec![i as i32, (i + j + 1) as i32];
                }
            }
        }
        
        result
    }
}

