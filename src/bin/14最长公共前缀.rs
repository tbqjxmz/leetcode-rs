/*
 * @lc app=leetcode.cn id=14 lang=rust
 *
 * [14] 最长公共前缀
 *
 * https://leetcode.cn/problems/longest-common-prefix/description/
 *
 * algorithms
 * Easy (44.12%)
 * Likes:    3141
 * Dislikes: 0
 * Total Accepted:    1.3M
 * Total Submissions: 3M
 * Testcase Example:  '["flower","flow","flight"]'
 *
 * 编写一个函数来查找字符串数组中的最长公共前缀。
 *
 * 如果不存在公共前缀，返回空字符串 ""。
 *
 *
 *
 * 示例 1：
 *
 *
 * 输入：strs = ["flower","flow","flight"]
 * 输出："fl"
 *
 *
 * 示例 2：
 *
 *
 * 输入：strs = ["dog","racecar","car"]
 * 输出：""
 * 解释：输入不存在公共前缀。
 *
 *
 *
 * 提示：
 *
 *
 * 1 <= strs.length <= 200
 * 0 <= strs[i].length <= 200
 * strs[i] 仅由小写英文字母组成
 *
 *
 */

fn main() {}
struct Solution;

// @lc code=start
impl Solution {
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        let base = strs[0].as_bytes();
        let mut max_len = base.len();

        for s in &strs[1..] {
            max_len = max_len.min(s.len());
        }

        let (mut low, mut high) = (0, max_len);
        while low < high {
            let mid = (low + high) / 2;
            if strs.iter().all(|s| s.as_bytes()[..mid] == base[..mid]) {
                low = mid;
            } else {
                high = mid - 1;
            }
        }

        String::from_utf8(base[..low].to_vec()).unwrap()
    }
}
// @lc code=end
