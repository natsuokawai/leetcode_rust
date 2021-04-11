fn main() {
    assert_eq!(
        Solution::longest_common_prefix(vec![
            String::from("flower"),
            String::from("flow"),
            String::from("flight"),
            String::from("fll")
        ]),
        String::from("fl")
    );
    assert_eq!(
        Solution::longest_common_prefix(vec![
            String::from("dog"),
            String::from("racecar"),
            String::from("car")
        ]),
        String::from("")
    );
    assert_eq!(
        Solution::longest_common_prefix(vec![String::from(""), String::from("")]),
        String::from("")
    );
    assert_eq!(
        Solution::longest_common_prefix(vec![String::from("ab"), String::from("a")]),
        String::from("a")
    );
    assert_eq!(
        Solution::longest_common_prefix(vec![
            String::from("abab"),
            String::from("aba"),
            String::from("")
        ]),
        String::from("")
    );
}

struct Solution;

impl Solution {
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        if strs.is_empty() {
            return String::new();
        }

        let mut strs_iter = strs.iter();
        let mut pre = strs_iter.next().unwrap().clone();

        while let Some(st) = strs_iter.next() {
            if st.is_empty() {
                return String::new();
            }
            for (i, c) in pre.chars().enumerate() {
                if let Some(s) = st.chars().nth(i) {
                    if s == c {
                        continue;
                    }
                }
                pre = pre.chars().take(i).collect::<String>();
                break;
            }
        }

        pre
    }
}
