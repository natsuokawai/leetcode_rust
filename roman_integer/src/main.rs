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
            match c {
                'I' => {
                    if let Some(n) = chars.peek() {
                        if *n == 'V' {
                            result += 4;
                            chars.next();
                        } else if *n == 'X' {
                            result += 9;
                            chars.next();
                        } else {
                            result += 1;
                        }
                    } else {
                        result += 1;
                    }
                },
                'X' => {
                    if let Some(n) = chars.peek() {
                        if *n == 'L' {
                            result += 40;
                            chars.next();
                        } else if *n == 'C' {
                            result += 90;
                            chars.next();
                        } else {
                            result += 10;
                        }
                    } else {
                        result += 10;
                    }
                },
                'C' => {
                    if let Some(n) = chars.peek() {
                        if *n == 'D' {
                            result += 400;
                            chars.next();
                        } else if *n == 'M' {
                            result += 900;
                            chars.next();
                        } else {
                            result += 100;
                        }
                    } else {
                        result += 100;
                    }
                },
                _ => { result += Solution::roman_chat_to_int(c); },
            }
        }

        result
    }

    fn roman_chat_to_int(c: char) -> i32 {
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

