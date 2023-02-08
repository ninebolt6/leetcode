use crate::Solution;
/*
 * @lc app=leetcode id=9 lang=rust
 *
 * [9] Palindrome Number
 */

// @lc code=start
impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        x.to_string().chars().rev().eq(x.to_string().chars())
    }
}
// @lc code=end

#[test]
fn test() {
    assert!(!Solution::is_palindrome(-321));
    assert!(Solution::is_palindrome(999));
    assert!(Solution::is_palindrome(16261));
    assert!(Solution::is_palindrome(162261));
}
