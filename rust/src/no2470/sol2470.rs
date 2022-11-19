struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn subarray_lcm(nums: Vec<i32>, k: i32) -> i32 {
        let n = nums.len();
        let mut ans = 0;
        for i in 0..n {
            let mut l = 1;
            for j in i..n {
                l = lcm(l, nums[j]);
                if l == k {
                    ans += 1;
                }
                if l > k {
                    break;
                }
            }
        }
        ans
    }
}

fn gcd(a: i32, b: i32) -> i32 {
    if a == 0 {
        return b;
    }

    return gcd(b % a, a);
}

fn lcm(a: i32, b: i32) -> i32 {
    a * b / gcd(a, b)
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test1() {
        let nums = vec![3, 6, 2, 7, 1];
        let k = 6;
        let actual = Solution::subarray_lcm(nums, k);
        let expected = 4;
        assert_eq!(expected, actual)
    }
    #[test]
    fn test2() {
        let nums = vec![3];
        let k = 2;
        let actual = Solution::subarray_lcm(nums, k);
        let expected = 0;
        assert_eq!(expected, actual)
    }
}
