struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn array_sign(nums: Vec<i32>) -> i32 {
        let mut minus = 0;

        for num in nums {
            if num == 0 {
                return 0;
            }
            if num < 0 {
                minus += 1;
            }
        }

        match minus % 2 == 0 {
            true => 1,
            false => -1,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test1() {
        let actual = Solution::array_sign(vec![-1, -2, -3, -4, 3, 2, 1]);
        let expected = 1;
        assert_eq!(expected, actual);
    }

    #[test]
    fn test2() {
        let actual = Solution::array_sign(vec![1, 5, 0, 2, -3]);
        let expected = 0;
        assert_eq!(expected, actual);
    }

    #[test]
    fn test3() {
        let actual = Solution::array_sign(vec![-1, 1, -1, 1, -1]);
        let expected = -1;
        assert_eq!(expected, actual);
    }
}
