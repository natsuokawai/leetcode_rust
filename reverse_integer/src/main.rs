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
        let mut arr = Vec::new();

        while num != 0 {
            arr.push(num % 10);
            num /= 10;
        }

        let result: i64 = arr
            .iter()
            .rev()
            .enumerate()
            .map(|(i, a)| a * 10i64.pow(i as u32))
            .fold(0, |sum, a| sum + a);

        if result < -2i64.pow(31) || 2i64.pow(31) - 1 < result {
            return 0;
        }

        result as i32
    }
}
