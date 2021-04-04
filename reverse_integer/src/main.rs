fn main() {
    assert_eq!(Solution::reverse(123), 321);
    assert_eq!(Solution::reverse(-123), -321);
    assert_eq!(Solution::reverse(120), 21);
    assert_eq!(Solution::reverse(0), 0);
    assert_eq!(Solution::reverse(1534236469), 0);
}

struct Solution;

impl Solution {
    pub fn reverse(x: i32) -> i32 {
        let mut num = x as i64;
        let mut result: i64 = 0;

        while num != 0 {
            result = result * 10 + num % 10;
            num /= 10;
        }

        if result < (std::i32::MIN as i64) || (std::i32::MAX as i64) < result {
            return 0;
        }

        result as i32
    }
}
