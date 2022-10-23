struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn valid_mountain_array(arr: Vec<i32>) -> bool {
        if arr.len() < 3 {
            return false;
        }

        let mut top = -1;
        let mut down = false;

        for idx in 0..arr.len() {
            let height = arr[idx];
            let prev: i32 = match idx {
                0 => -1,
                _ => arr[idx - 1],
            };

            if idx == 1 && height < prev {
                return false;
            }

            if down {
                if height > top {
                    return false;
                }
                if height >= prev {
                    return false;
                }
            } else {
                if height > top {
                    top = height;
                } else if height == prev {
                    return false;
                } else {
                    down = true;
                }
                if !down && height <= prev {
                    return false;
                }
            }
        }

        if !down {
            return false;
        }
        true
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test1() {
        let arr = vec![2, 1];
        let actual = Solution::valid_mountain_array(arr);
        let expected = false;
        assert_eq!(expected, actual);
    }
    #[test]
    fn test2() {
        let arr = vec![3, 5, 5];
        let actual = Solution::valid_mountain_array(arr);
        let expected = false;
        assert_eq!(expected, actual);
    }
    #[test]
    fn test3() {
        let arr = vec![0, 3, 2, 1];
        let actual = Solution::valid_mountain_array(arr);
        let expected = true;
        assert_eq!(expected, actual);
    }
    #[test]
    fn test4() {
        let arr = vec![3, 5, 5];
        let actual = Solution::valid_mountain_array(arr);
        let expected = false;
        assert_eq!(expected, actual);
    }
    #[test]
    fn test5() {
        let arr = vec![9, 8, 7, 6, 5, 4, 3, 2, 1, 0];
        let actual = Solution::valid_mountain_array(arr);
        let expected = false;
        assert_eq!(expected, actual);
    }
}
