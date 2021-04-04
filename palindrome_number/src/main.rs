fn main() {
    assert_eq!(Solution::is_palindrome(121), true);
    assert_eq!(Solution::is_palindrome(-121), false);
    assert_eq!(Solution::is_palindrome(10), false);
    assert_eq!(Solution::is_palindrome(-101), false);
}

struct Solution;

impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        if x < 0 {
            return false;
        }
        if x < 10 {
            return true;
        }

        let mut arr = Vec::new();
        let mut num = x;
        while num != 0 {
            arr.push(num % 10);
            num /= 10;
        }

        for (i, n) in arr.iter().rev().enumerate() {
            if &arr[i] != n {
                return false;
            }
        }
        true
    }
}
