use crate::Solution;
/*
 * @lc app=leetcode id=13 lang=rust
 *
 * [13] Roman to Integer
 */

// @lc code=start
#[derive(Copy, Clone)]
enum Symbol {
    I = 1,
    V = 5,
    X = 10,
    L = 50,
    C = 100,
    D = 500,
    M = 1000,
}

impl From<char> for Symbol {
    fn from(value: char) -> Self {
        match value {
            'I' => Self::I,
            'V' => Self::V,
            'X' => Self::X,
            'L' => Self::L,
            'C' => Self::C,
            'D' => Self::D,
            'M' => Self::M,
            _ => unreachable!(),
        }
    }
}

impl Solution {
    pub fn roman_to_int(s: String) -> i32 {
        let mut symbols = s.chars().map(Symbol::from).peekable();
        let mut sum = 0;
        loop {
            if let Some(base_symbol) = symbols.next() {
                if let Some(next_symbol) = symbols.peek() {
                    let val = match (base_symbol, next_symbol) {
                        (Symbol::I, Symbol::V | Symbol::X)
                        | (Symbol::X, Symbol::L | Symbol::C)
                        | (Symbol::C, Symbol::D | Symbol::M) => {
                            symbols.next().unwrap() as i32 - base_symbol as i32
                        }
                        (base, _) => base as i32,
                    };
                    sum += val;
                } else {
                    sum += base_symbol as i32;
                }
            } else {
                break;
            }
        }
        sum
    }
}
// @lc code=end

#[test]
fn test() {
    assert_eq!(Solution::roman_to_int(String::from("III")), 3);
    assert_eq!(Solution::roman_to_int(String::from("LVIII")), 58);
    assert_eq!(Solution::roman_to_int(String::from("MCMXCIV")), 1994);
}
