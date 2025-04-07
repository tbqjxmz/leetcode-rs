/*
 * @lc app=leetcode.cn id=69 lang=rust
 *
 * [69] x 的平方根 
 *
 * https://leetcode.cn/problems/sqrtx/description/
 *
 * algorithms
 * Easy (38.56%)
 * Likes:    1655
 * Dislikes: 0
 * Total Accepted:    1M
 * Total Submissions: 2.7M
 * Testcase Example:  '4'
 *
 * 给你一个非负整数 x ，计算并返回 x 的 算术平方根 。
 * 
 * 由于返回类型是整数，结果只保留 整数部分 ，小数部分将被 舍去 。
 * 
 * 注意：不允许使用任何内置指数函数和算符，例如 pow(x, 0.5) 或者 x ** 0.5 。
 * 
 * 
 * 
 * 示例 1：
 * 
 * 
 * 输入：x = 4
 * 输出：2
 * 
 * 
 * 示例 2：
 * 
 * 
 * 输入：x = 8
 * 输出：2
 * 解释：8 的算术平方根是 2.82842..., 由于返回类型是整数，小数部分将被舍去。
 * 
 * 
 * 
 * 
 * 提示：
 * 
 * 
 * 0 <= x <= 2^31 - 1
 * 
 * 
 */

fn main() {}
struct Solution;

// @lc code=start
impl Solution {
    pub fn my_sqrt(x: i32) -> i32 {
        let mut left = 0;
        let mut right = x;
        while left <= right {
            let mid = left + (right - left) / 2;
            let square = mid as i64 * mid as i64;
            if square == x as i64 {
                return mid;
            } else if square < x as i64 {
                left = mid + 1;
            } else {
                right = mid - 1;
            }
        }
        right
    }
}
// @lc code=end