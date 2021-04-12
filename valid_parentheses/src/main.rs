fn main() {
    assert_eq!(Solution::is_valid(String::from("()")), true);
    assert_eq!(Solution::is_valid(String::from("()[]{}")), true);
    assert_eq!(Solution::is_valid(String::from("(]")), false);
    assert_eq!(Solution::is_valid(String::from("([)]")), false);
    assert_eq!(Solution::is_valid(String::from("{[]}")), true);
    assert_eq!(Solution::is_valid(String::from("]}")), false);
    assert_eq!(Solution::is_valid(String::from("{[")), false);
}

struct Solution;

impl Solution {
    pub fn is_valid(s: String) -> bool {
        let mut parens = Vec::new();

        for c in s.chars() {
            match c {
                '(' => parens.push(')'),
                '[' => parens.push(']'),
                '{' => parens.push('}'),
                ')' | ']' | '}' if Some(c) != parens.pop() => return false,
                _ => (),
            }
        }

        parens.is_empty()
    }
}
