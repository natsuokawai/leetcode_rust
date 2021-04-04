fn main() {
    assert_eq!(Solution::is_palindrome(121), true);
    assert_eq!(Solution::is_palindrome(-121), false);
    assert_eq!(Solution::is_palindrome(10), false);
    assert_eq!(Solution::is_palindrome(-101), false);
}

struct Solution;

impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        if x < 0 || x % 10 == 0 {
            return false;
        }

        let mut num = x;
        let mut reverse = 0;
        while num != 0 {
            reverse = reverse * 10 + num % 10;
            num /= 10;
        }

        reverse == x
    }
}
