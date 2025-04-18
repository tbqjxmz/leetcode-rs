/*
 * @lc app=leetcode.cn id=66 lang=rust
 *
 * [66] 加一
 *
 * https://leetcode.cn/problems/plus-one/description/
 *
 * algorithms
 * Easy (45.88%)
 * Likes:    1484
 * Dislikes: 0
 * Total Accepted:    828.8K
 * Total Submissions: 1.8M
 * Testcase Example:  '[1,2,3]'
 *
 * 给定一个由 整数 组成的 非空 数组所表示的非负整数，在该数的基础上加一。
 * 
 * 最高位数字存放在数组的首位， 数组中每个元素只存储单个数字。
 * 
 * 你可以假设除了整数 0 之外，这个整数不会以零开头。
 * 
 * 
 * 
 * 示例 1：
 * 
 * 
 * 输入：digits = [1,2,3]
 * 输出：[1,2,4]
 * 解释：输入数组表示数字 123。
 * 
 * 
 * 示例 2：
 * 
 * 
 * 输入：digits = [4,3,2,1]
 * 输出：[4,3,2,2]
 * 解释：输入数组表示数字 4321。
 * 
 * 
 * 示例 3：
 * 
 * 
 * 输入：digits = [9]
 * 输出：[1,0]
 * 解释：输入数组表示数字 9。
 * 加 1 得到了 9 + 1 = 10。
 * 因此，结果应该是 [1,0]。
 * 
 * 
 * 
 * 
 * 提示：
 * 
 * 
 * 1 <= digits.length <= 100
 * 0 <= digits[i] <= 9
 * 
 * 
 */

fn main() {}
struct Solution;

// @lc code=start
impl Solution {
    pub fn plus_one(digits: Vec<i32>) -> Vec<i32> {
        let mut digits = digits;
        for i in (0..digits.len()).rev() {
            if digits[i] < 9 {
                digits[i] += 1;
                return digits;
            }
            digits[i] = 0;
        }
        let mut res = vec![1];
        res.extend(digits);
        res
    }
}
// @lc code=end

