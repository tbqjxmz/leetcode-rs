/*
 * @lc app=leetcode.cn id=1 lang=rust
 *
 * [1] 两数之和
 *
 * https://leetcode-cn.com/problems/two-sum/description/
 *
 * algorithms
 * Easy (52.41%)
 * Likes:    18652
 * Dislikes: 0
 * Total Accepted:    5.5M
 * Total Submissions: 10.2M
 * Testcase Example:  '[2,7,11,15]\n9'
 *
 * 给定一个整数数组 nums 和一个整数目标值 target，请你在该数组中找出 和为目标值 target  的那 两个 整数，并返回它们的数组下标。
 *
 * 你可以假设每种输入只会对应一个答案。但是，数组中同一个元素在答案里不能重复出现。
 *
 * 你可以按任意顺序返回答案。
 *
 *
 *
 * 示例 1：
 *
 *
 * 输入：nums = [2,7,11,15], target = 9
 * 输出：[0,1]
 * 解释：因为 nums[0] + nums[1] == 9 ，返回 [0, 1] 。
 *
 *
 * 示例 2：
 *
 *
 * 输入：nums = [3,2,4], target = 6
 * 输出：[1,2]
 *
 *
 * 示例 3：
 *
 *
 * 输入：nums = [3,3], target = 6
 * 输出：[0,1]
 *
 *
 *
 *
 * 提示：
 *
 *
 * 2 <= nums.length <= 10^4
 * -10^9 <= nums[i] <= 10^9
 * -10^9 <= target <= 10^9
 * 只会存在一个有效答案
 *
 *
 *
 *
 * 进阶：你可以想出一个时间复杂度小于 O(n^2) 的算法吗？
 *
 */

fn main() {}
struct Solution;

// @lc code=start
use std::collections::HashMap;

impl Solution {
    /// 给定一个整数数组 nums 和一个目标值 target，在该数组中找出和为目标值的两个整数的下标
    ///
    /// # 参数
    /// * `nums` - 整数数组
    /// * `target` - 目标值
    ///
    /// # 返回值
    /// * 返回一个包含两个整数的向量,表示这两个整数的下标
    ///
    /// # 示例
    /// ```
    /// let nums = vec![2, 7, 11, 15];
    /// let target = 9;
    /// assert_eq!(two_sum(nums, target), vec![0, 1]);
    /// ```
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut map = HashMap::new();
        for (i, num) in nums.iter().enumerate() {
            let complement = target - num;
            if let Some(&index) = map.get(&complement) {
                return vec![index, i as i32];
            }
            map.insert(num, i as i32);
        }

        unreachable!()
    }
}
// @lc code=end
