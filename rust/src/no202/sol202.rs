use std::collections::HashSet;

struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn is_happy(mut n: i32) -> bool {
        let mut past: HashSet<i32> = HashSet::new();

        loop {
            if n == 1 {
                return true;
            }
            if past.contains(&n) {
                return false;
            } else {
                past.insert(n);
                let mut sum = 0_i32;
                while n > 0 {
                    let last_digit = n % 10;
                    sum += last_digit * last_digit;
                    n = n / 10;
                }
                n = sum;
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test1() {
        let actual = Solution::is_happy(19);
        let expected = true;
        assert_eq!(expected, actual);
    }

    #[test]
    fn test2() {
        let actual = Solution::is_happy(2);
        let expected = false;
        assert_eq!(expected, actual);
    }

    #[test]
    fn test3() {
        let actual = Solution::is_happy(7);
        let expected = true;
        assert_eq!(expected, actual);
    }
}
