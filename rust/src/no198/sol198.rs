use std::cmp;

struct Solution;

#[allow(dead_code)]

impl Solution {
    pub fn rob(nums: Vec<i32>) -> i32 {
        if nums.len() == 1 {
            return nums[0];
        }

        let mut dp = vec![0; nums.len()];
        dp[0] = nums[0];
        dp[1] = cmp::max(nums[0], nums[1]);

        for i in 2..nums.len() {
            dp[i] = cmp::max(dp[i - 1], nums[i] + dp[i - 2])
        }

        println!("{:?}", dp);

        match dp.last() {
            Some(&n) => n,
            _ => 0,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test1() {
        let actual = Solution::rob(vec![1, 2, 3, 1]);
        let expected = 4;
        assert_eq!(expected, actual);
    }
    #[test]
    fn test2() {
        let actual = Solution::rob(vec![2, 7, 9, 3, 1]);
        let expected = 12;
        assert_eq!(expected, actual);
    }
}
