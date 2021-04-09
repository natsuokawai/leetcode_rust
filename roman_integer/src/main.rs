fn main() {
    assert_eq!(Solution::roman_to_int(String::from("III")), 3);
    assert_eq!(Solution::roman_to_int(String::from("IV")), 4);
    assert_eq!(Solution::roman_to_int(String::from("IX")), 9);
    assert_eq!(Solution::roman_to_int(String::from("XL")), 40);
    assert_eq!(Solution::roman_to_int(String::from("XC")), 90);
    assert_eq!(Solution::roman_to_int(String::from("CD")), 400);
    assert_eq!(Solution::roman_to_int(String::from("CM")), 900);
    assert_eq!(Solution::roman_to_int(String::from("LVIII")), 58);
    assert_eq!(Solution::roman_to_int(String::from("MCMXCIV")), 1994);
    assert_eq!(Solution::roman_to_int(String::from("MDCCCLXXXIV")), 1884);
}

struct Solution;

impl Solution {
    pub fn roman_to_int(s: String) -> i32 {
        let mut chars = s.chars().peekable();
        let mut result = 0;

        while let Some(c) = chars.next() {
            let current_num = Solution::roman_char_to_int(c);

            if let Some(n) = chars.peek() {
                let next_num = Solution::roman_char_to_int(*n);
                if current_num < next_num {
                    result -= current_num;
                } else {
                    result += current_num;
                }
            } else {
                result += current_num;
            }
        }

        result
    }

    fn roman_char_to_int(c: char) -> i32 {
        match c {
            'I' => 1,
            'V' => 5,
            'X' => 10,
            'L' => 50,
            'C' => 100,
            'D' => 500,
            'M' => 1000,
            _ => 0,
        }
    }
}
