use crate::Solution;
/*
* @lc app=leetcode id=1 lang=rust
*
* [1] Two Sum
*/

// @lc code=start
impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let nums = nums.into_iter().enumerate().collect::<Vec<_>>();
        for i in 0..nums.len() {
            for (idx, num) in &nums {
                if i != *idx && nums[i].1 + num == target {
                    return vec![i as i32, *idx as i32];
                }
            }
        }

        unreachable!()
    }
}
// @lc code=end
