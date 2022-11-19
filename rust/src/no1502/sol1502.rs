struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn can_make_arithmetic_progression(mut arr: Vec<i32>) -> bool {
        arr.sort();

        let dif = arr[1] - arr[0];
        for idx in 2..arr.len() {
            if arr[idx] - arr[idx - 1] != dif {
                return false;
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
        let arr = vec![3, 5, 1];
        let actual = Solution::can_make_arithmetic_progression(arr);
        let expected = true;
        assert_eq!(expected, actual);
    }
    #[test]
    fn test2() {
        let arr = vec![1, 2, 4];
        let actual = Solution::can_make_arithmetic_progression(arr);
        let expected = false;
        assert_eq!(expected, actual);
    }
}
