fn main() {
    assert_eq!(
        Solution::length_of_longest_substring(String::from("pwwkew")),
        3
    )
}

struct Solution;

impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let mut max_len = 0;

        for i in 0..s.len() {
            let mut substr: Vec<char> = vec![];
            for c in (&s[i..]).chars() {
                if substr.contains(&c) {
                    break;
                } else {
                    substr.push(c);
                }
            }

            if substr.len() > max_len {
                max_len = substr.len();
            }
        }

        max_len as i32
    }
}
