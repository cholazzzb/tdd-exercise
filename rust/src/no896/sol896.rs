struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn is_monotonic(nums: Vec<i32>) -> bool {
        let mut inc = true;
        let mut check_idx = 1;

        for idx in 1..nums.len() {
            let cur = nums[idx];
            let prev = nums[idx - 1];

            if prev == cur {
                check_idx += 1;
                continue;
            }

            let is_inc = prev < cur;

            if idx == check_idx {
                inc = is_inc
            } else {
                if inc != is_inc {
                    return false;
                }
            }
        }

        true
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test1() {
        let nums = vec![1, 2, 2, 3];
        let actual = Solution::is_monotonic(nums);
        let expected = true;
        assert_eq!(expected, actual);
    }
    #[test]
    fn test2() {
        let nums = vec![6, 5, 4, 4];
        let actual = Solution::is_monotonic(nums);
        let expected = true;
        assert_eq!(expected, actual);
    }
    #[test]
    fn test3() {
        let nums = vec![1, 3, 2];
        let actual = Solution::is_monotonic(nums);
        let expected = false;
        assert_eq!(expected, actual);
    }
    #[test]
    fn test4() {
        let nums = vec![1, 1, 0];
        let actual = Solution::is_monotonic(nums);
        let expected = true;
        assert_eq!(expected, actual);
    }
}
