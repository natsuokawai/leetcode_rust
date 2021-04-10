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
        Solution::longest_common_prefix(vec![
            String::from(""),
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
            for i in 0..st.len() - 1 {
                if st.chars().nth(i) != pre.chars().nth(i) {
                    pre = pre.chars().take(i).collect::<String>();
                    break;
                }
            } 
        }

        pre
    }
}
