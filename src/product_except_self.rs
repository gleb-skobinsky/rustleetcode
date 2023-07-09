use crate::solution::Solution;

impl Solution {
    pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
        let mut ans = vec![];
        let mut n = 1;
        for i in 0..nums.len() {
            ans.push(n);
            n *= nums[i];
        }
        n = 1;
        for i in (0..nums.len()).rev() {
            ans[i] *= n;
            n *= nums[i];
        }
        ans
    }
}