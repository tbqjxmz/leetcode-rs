/*
 * @lc app=leetcode.cn id=67 lang=rust
 *
 * [67] 二进制求和
 *
 * https://leetcode.cn/problems/add-binary/description/
 *
 * algorithms
 * Easy (53.17%)
 * Likes:    1287
 * Dislikes: 0
 * Total Accepted:    458.5K
 * Total Submissions: 852.6K
 * Testcase Example:  '"11"\n"1"'
 *
 * 给你两个二进制字符串 a 和 b ，以二进制字符串的形式返回它们的和。
 * 
 * 
 * 
 * 示例 1：
 * 
 * 
 * 输入:a = "11", b = "1"
 * 输出："100"
 * 
 * 示例 2：
 * 
 * 
 * 输入：a = "1010", b = "1011"
 * 输出："10101"
 * 
 * 
 * 
 * 提示：
 * 
 * 
 * 1 <= a.length, b.length <= 10^4
 * a 和 b 仅由字符 '0' 或 '1' 组成
 * 字符串如果不是 "0" ，就不含前导零
 * 
 * 
 */

fn main() {}
struct Solution;

// @lc code=start
impl Solution {
    pub fn add_binary(a: String, b: String) -> String {
        let mut result = String::new();
        let mut carry = 0;
        let mut a_chars = a.chars().rev().peekable();
        let mut b_chars = b.chars().rev().peekable();

        while a_chars.peek().is_some() || b_chars.peek().is_some() || carry > 0 {
            let a_bit = a_chars.next().unwrap_or('0').to_digit(2).unwrap();
            let b_bit = b_chars.next().unwrap_or('0').to_digit(2).unwrap();

            let sum = a_bit + b_bit + carry;
            carry = sum / 2;
            result.push(char::from_digit(sum % 2, 10).unwrap());
        }

        result.chars().rev().collect()
    }
}
// @lc code=end

