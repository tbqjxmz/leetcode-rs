/*
 * @lc app=leetcode.cn id=3190 lang=rust
 *
 * [3190] 使所有元素都可以被 3 整除的最少操作数
 *
 * https://leetcode.cn/problems/find-minimum-operations-to-make-all-elements-divisible-by-three/description/
 *
 * algorithms
 * Easy (92.92%)
 * Likes:    6
 * Dislikes: 0
 * Total Accepted:    8.1K
 * Total Submissions: 10K
 * Testcase Example:  '[1,2,3,4]'
 *
 * 给你一个整数数组 nums 。一次操作中，你可以将 nums 中的 任意 一个元素增加或者减少 1 。
 * 
 * 请你返回将 nums 中所有元素都可以被 3 整除的 最少 操作次数。
 * 
 * 
 * 
 * 示例 1：
 * 
 * 
 * 输入：nums = [1,2,3,4]
 * 
 * 输出：3
 * 
 * 解释：
 * 
 * 通过以下 3 个操作，数组中的所有元素都可以被 3 整除：
 * 
 * 
 * 将 1 减少 1 。
 * 将 2 增加 1 。
 * 将 4 减少 1 。
 * 
 * 
 * 
 * 示例 2：
 * 
 * 
 * 输入：nums = [3,6,9]
 * 
 * 输出：0
 * 
 * 
 * 
 * 
 * 提示：
 * 
 * 
 * 1 <= nums.length <= 50
 * 1 <= nums[i] <= 50
 * 
 * 
 */

fn main() {}
struct Solution;

// @lc code=start
impl Solution {
    pub fn minimum_operations(nums: Vec<i32>) -> i32 {
        let mut count = 0;
        for num in nums {
            match num % 3 {
                1 | -2 => count += 1,
                2 | -1 => count += 1,
                _ => (),
            }
        }
        count
    }
}
// @lc code=end

