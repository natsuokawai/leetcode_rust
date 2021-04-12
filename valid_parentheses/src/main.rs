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
            if c == '(' || c == '[' || c == '{' {
                parens.push(c);
            } else {
                if let Some(p) = parens.pop() {
                    if Self::pair(p) == c {
                        continue;
                    }
                }
                return false;
            }
        }

        parens.is_empty()
    }

    fn pair(c: char) -> char {
        match c {
            '(' => ')',
            '[' => ']',
            '{' => '}',
            _ => panic!("Invalid character."),
        }
    }
}
